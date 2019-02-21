use super::state_transition::StateTransition;
use crate::lib::common::messaging::telegram::Telegram;

pub trait State {
    type Entity;
    type MessageType : Eq;
    fn new() -> Box<Self> where Self: Sized;
    fn enter(&mut self, entity: &mut Self::Entity);
    fn execute(&mut self, entity: &mut Self::Entity) -> StateTransition<Self::Entity, Self::MessageType>;
    fn exit(&mut self, entity: &mut Self::Entity);
    fn on_message(&mut self, entity: &mut Self::Entity, message: &Telegram<Self::MessageType>) -> bool;
}