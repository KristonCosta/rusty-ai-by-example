use super::state_transition::StateTransition;

pub trait State {
    type Entity;
    type Enum;
    fn new() -> Box<Self> where Self: Sized;
    fn enter(&mut self, entity: &mut Self::Entity);
    fn execute(&mut self, entity: &mut Self::Entity) -> StateTransition<Self::Entity, Self::Enum>;
    fn exit(&mut self, entity: &mut Self::Entity);
}