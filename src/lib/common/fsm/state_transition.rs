use super::state::State;

pub enum StateTransition<Entity, MessageType> {
    None,
    Push(Box<dyn State<Entity=Entity, MessageType=MessageType> + 'static>),
    Pop(),
    Switch(Box<dyn State<Entity=Entity, MessageType=MessageType> + 'static>),
    Exit(),
}