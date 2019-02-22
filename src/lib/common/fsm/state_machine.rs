use super::state::State;
use super::state_transition::StateTransition;
use crate::lib::common::messaging::telegram::Telegram;

pub type StateDef<Entity, MessageType> = Box<dyn State<Entity=Entity, MessageType=MessageType> + 'static>;

pub struct StateMachine<E, MessageType: Eq> {
    current_state : Option<StateDef<E, MessageType>>,
    previous_state : Option<StateDef<E, MessageType>>,
    global_state: Option<StateDef<E, MessageType>>,
}

pub struct StateMachineBuilder<E, MessageType: Eq> {
    current_state : Option<StateDef<E, MessageType>>,
    global_state: Option<StateDef<E, MessageType>>,}

impl <E, MessageType: Eq> StateMachineBuilder<E, MessageType> {
    pub fn new() -> Self {
        StateMachineBuilder {
            current_state: None,
            global_state: None,
        }
    }

    pub fn set_initial_state(mut self, state : StateDef<E, MessageType>) -> Self {
        self.current_state = Some(state);
        self
    }

    pub fn set_global_state(mut self, state : StateDef<E, MessageType>) -> Self {
        self.global_state = Some(state);
        self
    }

    pub fn build(self) -> StateMachine<E, MessageType> {
        StateMachine {
            global_state: self.global_state,
            current_state: self.current_state,
            previous_state: None,
        }
    }
}

impl <E, MessageType: Eq> StateMachine<E, MessageType> {

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

    fn handle_transition(&mut self, transition: StateTransition<E, MessageType>, entity : &mut E) {
        match transition {
            StateTransition::None => {},
            StateTransition::Push(state) => {
                self.exit_current_state(entity);
                self.previous_state = self.current_state.take();
                self.current_state =  Some(state);
                self.enter_current_state(entity);
            },
            StateTransition::Pop() => {
                self.exit_current_state(entity);
                self.current_state = self.previous_state.take();
                self.enter_current_state(entity);
            },
            StateTransition::Switch(state) => {
                self.exit_current_state(entity);
                self.current_state = Some(state);
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

    pub fn handle_message(&mut self, entity: &mut E, telegram: &Telegram<MessageType>) -> bool {
        let (was_handled, transition) = match self.current_state {
            Some(ref mut state) => state.on_message(entity, telegram),
            None => (false, StateTransition::None)
        };
        if was_handled {
            self.handle_transition(transition, entity);
            return true
        }
        let (was_handled, transition) = match self.global_state {
            Some(ref mut state) => state.on_message(entity, telegram),
            None => (false, StateTransition::None)
        };
        if was_handled {
            self.handle_transition(transition, entity);
            return true
        }
        false

    }
}

#[cfg(test)]
mod tests {
    use crate::lib::common::fsm::state::State;
    use crate::lib::common::fsm::state_transition::StateTransition;
    use crate::lib::common::fsm::state_machine::StateMachine;
    use crate::lib::common::fsm::state_machine::StateMachineBuilder;
    use crate::lib::common::fsm::state_machine::StateDef;
    use crate::lib::common::messaging::telegram::Telegram;
    #[derive(PartialEq, Eq)]
    enum TestMessageEnum {
        State1,
        State2,
    }

    struct TestEntity {
        pub state1 : i32,
        pub state2 : i32,
    }


    struct TestState1;

    impl State for TestState1 {
        type Entity = TestEntity;
        type MessageType = TestMessageEnum;

        fn new() -> Box<Self> {
            Box::new(TestState1{})
        }

        fn enter(&mut self, entity: &mut Self::Entity) {
            println!("Entering state 1");
            entity.state1 += 1;
        }

        fn execute(&mut self, entity: &mut Self::Entity) -> StateTransition<TestEntity, TestMessageEnum> {
            println!("Executing state 1");
            entity.state1 += 20;
            StateTransition::Switch(TestState2::new())
        }

        fn exit(&mut self, entity: &mut Self::Entity) {
            println!("Exiting state 1");
            entity.state1 *= 10;
        }

        fn on_message(&mut self, entity: &mut Self::Entity, message: &Telegram<Self::MessageType>) -> (bool, StateTransition<Self::Entity, Self::MessageType>) {
            unimplemented!()
        }
    }

    struct TestState2;

    impl State for TestState2 {
        type Entity = TestEntity;

        fn new() -> Box<Self> where Self: Sized {
            Box::new(TestState2{})
        }

        fn enter(&mut self, entity: &mut Self::Entity) {
            println!("Entering state 2");
            entity.state2 += 2;
        }

        fn execute(&mut self, entity: &mut Self::Entity) -> StateTransition<TestEntity, TestMessageEnum> {
            println!("Executing state 2");
            entity.state2 += 30;
            StateTransition::None
        }

        fn exit(&mut self, entity: &mut Self::Entity) {
            println!("Exiting state 2");
            entity.state2 *= 10;
        }

        type MessageType = TestMessageEnum;

        fn on_message(&mut self, entity: &mut Self::Entity, message: &Telegram<Self::MessageType>) -> (bool, StateTransition<Self::Entity, Self::MessageType>) {
            unimplemented!()
        }
    }

    #[test]
    fn test_run() {
        let mut machine = StateMachineBuilder::<TestEntity, TestMessageEnum>::new()
            .set_initial_state(Box::new(TestState1{}))
            .build();
        let mut entity = TestEntity{
            state1: 0,
            state2: 0,
        };
        machine.update(&mut entity);
        machine.update(&mut entity);
        assert_eq!(entity.state1, 200);
        assert_eq!(entity.state2, 32);
    }



}
