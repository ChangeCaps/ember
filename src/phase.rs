pub trait Phase {
    type Input;
    type Output;
    type State;

    fn render();
}
