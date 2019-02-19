use super::state::State;

pub enum StateTransition<T> {
    None,
    Push(T),
    Pop(),
    Switch(T),
    Exit(),
}