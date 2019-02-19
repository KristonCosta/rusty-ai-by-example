use super::state::State;

pub enum StateTransition<Entity, StateEnum> {
    None,
    Push(Box<dyn State<Entity=Entity, Enum=StateEnum> + 'static>),
    Pop(),
    Switch(Box<dyn State<Entity=Entity, Enum=StateEnum> + 'static>),
    Exit(),
}