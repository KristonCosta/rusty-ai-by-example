/*
    Make it so you can define a set of states in an enum that can then
    implement type "state"
    enum MinerState{
        ...
        ...
        ...
    }

    Then have it so the FSM takes in a generic of type T : Sized + State

    Make a state factory that takes in a state enum and generates a state
    make the machine take in a state factory and an enum of all states, the factory
    will take the state enum and make a state object
*/

use enum_dispatch::enum_dispatch;
use super::state_transition::StateTransition;

#[enum_dispatch]
pub trait State {
    type Entity;
    type Enum;
    fn new() -> Box<Self> where Self: Sized;
    fn enter(&mut self, entity: &mut Self::Entity);
    fn execute(&mut self, entity: &mut Self::Entity) -> StateTransition<Self::Entity, Self::Enum>;
    fn exit(&mut self, entity: &mut Self::Entity);
}