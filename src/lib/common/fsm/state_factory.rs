use super::state::State;

pub trait StateFactory<StateEnum, S, E> where S : State<Enum=StateEnum, Entity=E> + Sized {
    fn make_state(&mut self, enum_val: StateEnum) -> S;
}