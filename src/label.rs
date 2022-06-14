use std::{
    any::{type_name, Any, TypeId},
    borrow::Cow,
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use ahash::AHasher;

pub use ember_macro::PhaseLabel;

#[derive(Clone, Debug)]
pub struct RawLabel {
    ty: Option<TypeId>,
    name: Cow<'static, str>,
    hash: u64,
}

impl RawLabel {
    pub fn new<T>(label: &T) -> Self
    where
        T: Hash + Any,
    {
        let mut hasher = AHasher::new_with_keys(420, 69);
        label.hash(&mut hasher);
        let hash = hasher.finish();

        Self {
            ty: Some(TypeId::of::<T>()),
            name: Cow::Borrowed(type_name::<T>()),
            hash,
        }
    }

    pub fn ty(&self) -> Option<TypeId> {
        self.ty
    }

    pub fn name(&self) -> &Cow<'static, str> {
        &self.name
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }
}

impl PartialEq for RawLabel {
    fn eq(&self, other: &Self) -> bool {
        if let (Some(this_ty), Some(other_ty)) = (self.ty, other.ty) {
            this_ty == other_ty && self.hash == other.hash
        } else {
            self.name == other.name && self.hash == other.hash
        }
    }
}

impl Eq for RawLabel {}

impl PartialOrd for RawLabel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let (Some(this_ty), Some(other_ty)) = (self.ty, other.ty) {
            if this_ty == other_ty {
                Some(self.hash.cmp(&other.hash))
            } else {
                None
            }
        } else {
            if self.name == other.name {
                Some(self.hash.cmp(&other.hash))
            } else {
                None
            }
        }
    }
}

impl Hash for RawLabel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.as_ref().hash(state);
        state.write_u64(self.hash);
    }
}

macro_rules! label {
    ($trait_ident:ident, $raw_ident:ident) => {
        pub trait $trait_ident {
            fn raw_label(&self) -> $raw_ident;
        }

        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Hash)]
        pub struct $raw_ident(RawLabel);

        impl From<RawLabel> for $raw_ident {
            fn from(label: RawLabel) -> Self {
                Self(label)
            }
        }

        impl<T: $trait_ident> From<T> for $raw_ident {
            fn from(label: T) -> Self {
                label.raw_label()
            }
        }
    };
}

label!(StageLabel, RawStageLabel);
label!(PhaseLabel, RawPhaseLabel);
