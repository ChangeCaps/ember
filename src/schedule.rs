use std::collections::{HashMap, HashSet};

use crate::{
    label::{PhaseLabel, RawPhaseLabel},
    phase::Phase,
};

pub trait PhaseScheduleCoercion {
    fn entry(self) -> PhaseScheduleEntry;
    fn label(self, label: impl PhaseLabel) -> PhaseScheduleEntry;
    fn before(self, label: impl PhaseLabel) -> PhaseScheduleEntry;
    fn after(self, label: impl PhaseLabel) -> PhaseScheduleEntry;
}

pub struct PhaseScheduleEntry {
    pub labels: HashSet<RawPhaseLabel>,
    pub before: HashSet<RawPhaseLabel>,
    pub after: HashSet<RawPhaseLabel>,
}

impl PhaseScheduleCoercion for PhaseScheduleEntry {
    fn entry(self) -> PhaseScheduleEntry {
        self
    }

    fn label(mut self, label: impl PhaseLabel) -> PhaseScheduleEntry {
        self.labels.insert(label.raw_label());
        self
    }

    fn before(mut self, label: impl PhaseLabel) -> PhaseScheduleEntry {
        self.before.insert(label.raw_label());
        self
    }

    fn after(mut self, label: impl PhaseLabel) -> PhaseScheduleEntry {
        self.after.insert(label.raw_label());
        self
    }
}

#[derive(Default)]
pub struct PhaseSchedule {
    entries: Vec<PhaseScheduleEntry>,
}

impl PhaseSchedule {
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_phase(&mut self, phase: impl PhaseScheduleCoercion) {
        self.entries.push(phase.entry());
    }

    pub fn phase_order(&self) -> Result<Vec<usize>, ScheduleError> {
        let mut labels = HashMap::<RawPhaseLabel, HashSet<usize>>::new();

        for (i, entry) in self.entries.iter().enumerate() {
            for label in entry.labels.iter() {
                labels.entry(label.clone()).or_default().insert(i);
            }
        }

        let mut dependency_graph = HashMap::<usize, HashSet<usize>>::new();

        for (i, entry) in self.entries.iter().enumerate() {
            for label in entry.before.iter() {
                for &after in labels[label].iter() {
                    dependency_graph.entry(after).or_default().insert(i);
                }
            }

            let dependencies = dependency_graph.entry(i).or_default();

            for label in entry.after.iter() {
                for &before in labels[label].iter() {
                    dependencies.insert(before);
                }
            }
        }

        let mut order = Vec::new();

        while !dependency_graph.is_empty() {
            let index = *dependency_graph.keys().next().unwrap();

            fn add_dependencies_recursive(
                order: &mut Vec<usize>,
                index: usize,
                dependency_graph: &mut HashMap<usize, HashSet<usize>>,
            ) -> Result<(), ScheduleError> {
                let dependencies = dependency_graph
                    .remove(&index)
                    .ok_or(ScheduleError::CyclicDependency)?;

                for dependency in dependencies {
                    if !order.contains(&dependency) {
                        add_dependencies_recursive(order, dependency, dependency_graph)?;
                    }
                }

                order.push(index);

                Ok(())
            }

            add_dependencies_recursive(&mut order, index, &mut dependency_graph)?;
        }

        Ok(order)
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ScheduleError {
    #[error("cyclic dependency in schedule")]
    CyclicDependency,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PhaseLabel, Hash)]
    enum PhaseLabel {
        A,
        B,
    }

    #[test]
    fn schedule_order() {
        let mut scheule = PhaseSchedule::new();

        //scheule.add_phase(Phase::new().label(PhaseLabel::A));
        //scheule.add_phase(Phase::new().label(PhaseLabel::B).before(PhaseLabel::A));

        assert_eq!(scheule.phase_order().unwrap(), vec![1, 0]);
    }

    #[test]
    fn detect_cyclic_dependency() {
        let mut scheule = PhaseSchedule::new();

        //scheule.add_phase(Phase::new().label(PhaseLabel::A).before(PhaseLabel::B));
        //scheule.add_phase(Phase::new().label(PhaseLabel::B).before(PhaseLabel::A));

        assert_eq!(
            scheule.phase_order().unwrap_err(),
            ScheduleError::CyclicDependency
        );
    }
}
