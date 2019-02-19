use super::map;
use super::entity;
use super::entity_names;
use crate::lib::common::fsm::state_machine;
use crate::lib::common::fsm::state_machine::StateMachine;
use crate::chapter1::part_two::miner_states::MinerStates;
use crate::lib::common::fsm::state_machine::StateMachineBuilder;
use crate::chapter1::part_two::miner_states::GoHomeAndSleepTilRested;

const COMFORT_LEVEL: i64 = 5;
const MAX_NUGGETS: i64 = 5;
const THIRST_LEVEL: i64 = 5;
const TIREDNESS_THRESHOLD: i64 = 5;

pub struct StatefulMiner {
    base_id : i64,
    state_machine : StateMachine<MinerStates, Miner>,
    data: Miner
}

pub struct Miner {
    location : map::Locations,
    name: entity_names::Names,
    gold_carried : i64,
    money_in_bank : i64,
    thirst : i64,
    fatigue : i64,
}

impl entity::Entity for StatefulMiner {
    fn new(id: i64) -> Self {
        use crate::lib::common::fsm::state::State;
        StatefulMiner {
            base_id: id,
            state_machine: StateMachineBuilder::new().set_initial_state(GoHomeAndSleepTilRested::new()).build(),
            data: Miner {
                name: entity_names::Names::MinerBob,
                location: map::Locations::Shack,
                gold_carried: 0,
                money_in_bank: 0,
                thirst: 0,
                fatigue: 0,
            }
        }
    }

    fn update(&mut self) {
        {
            self.data.thirst += 1;
        }
        self.state_machine.update(&mut self.data);
    }
}

impl Miner {
    pub fn location(&self) -> map::Locations {
        self.location.clone()
    }

    pub fn change_location(&mut self, location: map::Locations) {
        self.location = location
    }

    pub fn set_gold_carried(&mut self, gold: i64) {
        self.gold_carried = gold
    }

    pub fn add_to_gold_carried(&mut self, gold : i64) {
        self.gold_carried += gold
    }

    pub fn increase_fatigue(&mut self) {
        self.fatigue += 1
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn pocket_is_full(&self) -> bool {
        self.gold_carried >= MAX_NUGGETS
    }

    pub fn thirsty(&self) -> bool {
        self.thirst >= THIRST_LEVEL
    }

    pub fn add_to_wealth(&mut self, gold : i64) {
        self.money_in_bank += gold
    }

    pub fn gold_carried(&self) -> i64 {
        self.gold_carried
    }

    pub fn wealth(&self) -> i64 {
        self.money_in_bank
    }

    pub fn comfortable(&self) -> bool {
        self.money_in_bank >= COMFORT_LEVEL
    }

    pub fn fatigued(&self) -> bool {
        self.fatigue >= TIREDNESS_THRESHOLD
    }

    pub fn decrease_fatigue(&mut self) {
        self.fatigue -= 1;
    }

    pub fn buy_drink_and_whiskey(&mut self) {
        self.thirst = 0;
        self.money_in_bank -= 2;
    }
}