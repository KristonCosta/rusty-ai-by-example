use super::map;
use super::entity_names;
use super::miner_wife_states::MinerWifeStates;
use super::miner_wife_states::WifesGlobalState;
use super::miner_wife_states::DoHouseWork;

use crate::lib::common::fsm::state_machine;
use crate::lib::common::fsm::state_machine::StateMachine;
use crate::lib::common::fsm::state_machine::StateMachineBuilder;
use crate::lib::common::entity::entity::Entity;

use colored::*;
use crate::chapter1::part_three::message_types::MessageTypes;
use crate::lib::common::messaging::telegram::Telegram;
use crate::lib::common::entity::entity::EntityId;
use std::sync::mpsc::Sender;


pub struct StatefulWife {
    base_id : EntityId,
    state_machine : StateMachine<MinerWife, MessageTypes>,
    data: MinerWife
}

pub struct MinerWife {
    location : map::Locations,
    name: entity_names::Names,
    message_channel: Sender<MessageTypes>,
}

impl Entity<MessageTypes> for StatefulWife {
    fn new(id: EntityId, message_channel: Sender<MessageTypes>) -> Self {
        use crate::lib::common::fsm::state::State;
        StatefulWife {
            base_id: id,
            state_machine: StateMachineBuilder::new()
                .set_initial_state(DoHouseWork::new())
                .set_global_state(WifesGlobalState::new())
                .build(),
            data: MinerWife {
                location: map::Locations::Shack,
                name: entity_names::Names::Elsa,
                message_channel
            }
        }
    }

    fn update(&mut self) {
        self.state_machine.update(&mut self.data);
    }

    fn handle_message(&mut self, telegram: Telegram<MessageTypes>) {
        unimplemented!()
    }
}

impl MinerWife {
    pub fn name(&self) -> ColoredString {
        self.name.to_string().blue()
    }
}
