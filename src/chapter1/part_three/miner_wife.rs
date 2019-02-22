use super::map;
use super::entity_names;
use super::miner_wife_states::WifesGlobalState;
use super::miner_wife_states::DoHouseWork;

use crate::lib::common::fsm::state_machine::StateMachine;
use crate::lib::common::fsm::state_machine::StateMachineBuilder;
use crate::lib::common::entity::entity::Entity;

use colored::*;
use crate::chapter1::part_three::message_types::MessageTypes;
use crate::lib::common::messaging::telegram::Telegram;
use crate::lib::common::entity::entity::EntityId;
use std::rc::Rc;
use std::cell::RefCell;
use crate::lib::common::messaging::message_dispatcher::MessageDispatcher;
use std::cell::RefMut;


pub struct StatefulWife {
    state_machine : StateMachine<MinerWife, MessageTypes>,
    data: MinerWife
}

pub struct MinerWife {
    base_id: EntityId,
    bob_id: Option<EntityId>,
    name: entity_names::Names,
    message_channel: Rc<RefCell<MessageDispatcher<MessageTypes>>>,
    cooking: bool,
}

impl StatefulWife {
    pub fn set_bob(&mut self, id: EntityId) {
        self.data.bob_id = Some(id)
    }
}

impl Entity<MessageTypes> for StatefulWife {
    fn new(id: EntityId, message_channel: Rc<RefCell<MessageDispatcher<MessageTypes>>>) -> Self {
        use crate::lib::common::fsm::state::State;
        StatefulWife {

            state_machine: StateMachineBuilder::new()
                .set_initial_state(DoHouseWork::new())
                .set_global_state(WifesGlobalState::new())
                .build(),
            data: MinerWife {
                base_id: id,
                bob_id: Some(0),
                name: entity_names::Names::Elsa,
                message_channel,
                cooking: false,
            }
        }
    }

    fn get_id(&self) -> EntityId {
        self.data.base_id
    }

    fn update(&mut self) {
        self.state_machine.update(&mut self.data);
    }

    fn handle_message(&mut self, telegram: Telegram<MessageTypes>) -> bool {
        self.state_machine.handle_message(&mut self.data, &telegram)
    }
}

impl MinerWife {
    pub fn id(&self) -> EntityId {
        self.base_id
    }



    pub fn get_bob(&self) -> EntityId {
        self.bob_id.expect("Bob wasn't set!!!")
    }

    pub fn name(&self) -> ColoredString {
        self.name.to_string().blue()
    }

    pub fn is_cooking(&self) -> bool {
        self.cooking
    }

    pub fn start_cooking(&mut self) {
        self.cooking = true
    }

    pub fn stop_cooking(&mut self) {
        self.cooking = false
    }

    pub fn dispatch(&mut self) -> RefMut<MessageDispatcher<MessageTypes>> {
        self.message_channel.borrow_mut()
    }
}
