use super::state::State;
use super::state_transition::StateTransition;
// use super::state_factory::StateFactory;

type StateFactory<StateEnum, S>
    where S : State<Enum=StateEnum> + Sized = Fn(StateEnum) -> S;
// type StateFactory<StateEnum, S, E>
// pub trait StateFactory<StateEnum, S, E> where S : State<Enum=StateEnum, Entity=E> + Sized {
// fn make_state(&mut self, enum_val: StateEnum) -> S;

pub struct StateMachine<StateEnum, StateType, E>
    where StateType: State<Entity=E, Enum=StateEnum> {
    current_state : Option<StateType>,
    previous_state : Option<StateType>,
    global_state: Option<StateType>,
    state_factory: StateFactory<StateEnum, StateType>
}


impl <StateEnum, StateType, E> StateMachine<StateEnum, StateType, E>
    where StateType: State<Entity=E, Enum=StateEnum> {

    pub fn update(&mut self, entity : &mut E) {
        let global_transition = match &mut self.global_state {
            Some(state) => state.execute(entity),
            None => StateTransition::None,
        };
        self.handle_transition(global_transition, entity);

        let current_transition = match &mut self.current_state {
            Some(state) => state.execute(entity),
            None => StateTransition::None,
        };
        self.handle_transition(current_transition, entity);
    }

    fn handle_transition(&mut self, transition: StateTransition<StateEnum>, entity : &mut E) {
        match transition {
            StateTransition::None => {},
            StateTransition::Push(state) => {
                self.exit_current_state(entity);
                self.previous_state = self.current_state.take();
                self.current_state =  Some(self.state_factory.make_state(state));
                self.enter_current_state(entity);
            },
            StateTransition::Pop() => {
                self.exit_current_state(entity);
                self.current_state = self.previous_state.take();
                self.enter_current_state(entity);
            },
            StateTransition::Switch(state) => {
                self.exit_current_state(entity);
                self.current_state = Some(self.state_factory.make_state(state));
                self.enter_current_state(entity);
            }
            StateTransition::Exit() => {},
        }
    }

    fn exit_current_state(&mut self, entity : &mut E) {
        match self.current_state {
            Some(ref mut state) => state.exit(entity),
            _ => ()
        }
    }

    fn enter_current_state(&mut self, entity : &mut E) {
        match self.current_state {
            Some(ref mut state) => state.enter(entity),
            _ => ()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::common::fsm::state_factory::StateFactory;
    use crate::lib::common::fsm::state::State;
    use crate::lib::common::fsm::state_transition::StateTransition;
    use crate::lib::common::fsm::state_machine::StateMachine;

    enum TestStateEnum {
        State1,
        State2,
    }

    struct TestEntity {
        pub state1 : i32,
        pub state2 : i32,
    }

    fn make_test_state(enum_val : TestStateEnum) -> Box<State<Enum=TestStateEnum, Entity=TestEntity>> {
        match enum_val {
            TestStateEnum::State1 => Box::new(TestState1{}),
            TestStateEnum::State2 => Box::new(TestState2{}),
        }
    }

    struct TestState1 {}

    impl State for TestState1 {
        type Entity = TestEntity;
        type Enum = TestStateEnum;

        fn new() -> Self {
            TestState1{}
        }

        fn enter(&mut self, entity: &mut Self::Entity) {
            println!("Entering state 1");
            entity.state1 += 1;
        }

        fn execute(&mut self, entity: &mut Self::Entity) -> StateTransition<Self::Enum> {
            println!("Executing state 1");
            entity.state1 += 20;
            StateTransition::Switch(TestStateEnum::State2(TestState2{}))
        }

        fn exit(&mut self, entity: &mut Self::Entity) {
            println!("Exiting state 1");
            entity.state1 *= 10;
        }
    }

    struct TestState2{}

    impl State for TestState2 {
        type Entity = TestEntity;
        type Enum = TestStateEnum;

        fn new() -> Self where Self: Sized {
            TestState2{}
        }

        fn enter(&mut self, entity: &mut Self::Entity) {
            println!("Entering state 2");
            entity.state2 += 2;
        }

        fn execute(&mut self, entity: &mut Self::Entity) -> StateTransition<Self::Enum> {
            println!("Executing state 2");
            entity.state2 += 30;
            StateTransition::None
        }

        fn exit(&mut self, entity: &mut Self::Entity) {
            println!("Exiting state 2");
            entity.state2 *= 10;
        }
    }

    #[test]
    fn test_run() {
        let mut machine = StateMachine {
            current_state: Some(TestState1{}),
            previous_state: None,
            global_state: None,
            state_factory: make_test_state,
        };
        let mut entity = TestEntity{
            state1: 0,
            state2: 0,
        };
        machine.update(&mut entity);
    }



}
