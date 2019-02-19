use super::map;
use super::entity;
use super::entity_names;
use crate::lib::common::fsm::state_machine;
use crate::lib::common::fsm::state_machine::StateMachine;
use crate::lib::common::fsm::state_machine::StateMachineBuilder;
use crate::chapter1::part_two::miner_wife_states::MinerWifeStates;
use crate::chapter1::part_two::miner_wife_states::WifesGlobalState;
use crate::chapter1::part_two::miner_wife_states::DoHouseWork;


pub struct StatefulWife {
    base_id : i64,
    state_machine : StateMachine<MinerWifeStates, MinerWife>,
    data: MinerWife
}

pub struct MinerWife {
    location : map::Locations,
    name: entity_names::Names,
}

impl entity::Entity for StatefulWife {
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
}

impl MinerWife {
    pub fn name(&self) -> String {
        self.name.to_string()
    }
}
