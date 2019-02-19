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
use crate::chapter1::part_three::extra_info_enum::ExtraInfo;
use crate::lib::common::messaging::telegram::Telegram;


pub struct StatefulWife {
    base_id : i64,
    state_machine : StateMachine<MinerWifeStates, MinerWife>,
    data: MinerWife
}

pub struct MinerWife {
    location : map::Locations,
    name: entity_names::Names,
}

impl Entity<MessageTypes, ExtraInfo> for StatefulWife {
    fn new(id: i64) -> Self {
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
            }
        }
    }

    fn update(&mut self) {
        self.state_machine.update(&mut self.data);
    }

    fn handle_message(&mut self, telegram: Telegram<MessageTypes, ExtraInfo>) {
        unimplemented!()
    }
}

impl MinerWife {
    pub fn name(&self) -> ColoredString {
        self.name.to_string().blue()
    }
}
