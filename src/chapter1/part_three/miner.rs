use super::map;
use super::entity_names;
use super::miner_states::GoHomeAndSleepTilRested;

use crate::lib::common::fsm::state_machine::StateMachine;
use crate::lib::common::fsm::state_machine::StateMachineBuilder;
use crate::lib::common::entity::entity::Entity;

use colored::*;
use crate::lib::common::messaging::telegram::Telegram;
use crate::chapter1::part_three::message_types::MessageTypes;
use crate::lib::common::entity::entity::EntityId;
use std::sync::mpsc::Sender;
use std::rc::Rc;
use crate::lib::common::messaging::message_dispatcher::MessageDispatcher;
use std::cell::RefMut;
use std::cell::RefCell;


const COMFORT_LEVEL: i64 = 5;
const MAX_NUGGETS: i64 = 5;
const THIRST_LEVEL: i64 = 5;
const TIREDNESS_THRESHOLD: i64 = 5;

pub struct StatefulMiner {
    state_machine : StateMachine<Miner, MessageTypes>,
    data: Miner
}

pub struct Miner {
    base_id: EntityId,
    location : map::Locations,
    name: entity_names::Names,
    gold_carried : i64,
    money_in_bank : i64,
    thirst : i64,
    fatigue : i64,
    message_channel: Rc<RefCell<MessageDispatcher<MessageTypes>>>,
    wife_id: Option<EntityId>,
}

impl StatefulMiner {
    pub fn set_wife(&mut self, id: EntityId) {
        self.data.wife_id = Some(id)
    }
}

impl Entity<MessageTypes> for StatefulMiner {
    fn new(id: EntityId, dispatcher: Rc<RefCell<MessageDispatcher<MessageTypes>>>) -> Self {
        use crate::lib::common::fsm::state::State;
        StatefulMiner {
            state_machine: StateMachineBuilder::new()
                .set_initial_state(GoHomeAndSleepTilRested::new())
                .build(),
            data: Miner {
                base_id: id,
                name: entity_names::Names::MinerBob,
                location: map::Locations::Shack,
                gold_carried: 0,
                money_in_bank: 0,
                thirst: 0,
                fatigue: 0,
                message_channel: dispatcher,
                wife_id: Some(1)
            }
        }
    }

    fn get_id(&self) -> EntityId {
        self.data.base_id
    }

    fn update(&mut self) {
        {
            self.data.thirst += 1;
        }
        self.state_machine.update(&mut self.data);

    }

    fn handle_message(&mut self, telegram: Telegram<MessageTypes>) -> bool {
        self.state_machine.handle_message(&mut self.data, &telegram)
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

    pub fn name(&self) -> ColoredString {
        self.name.to_string().red()
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



    pub fn get_wife(&self) -> EntityId {
        self.wife_id.expect("Wife id wasn't set!!!")
    }

    pub fn dispatch(&mut self) -> RefMut<MessageDispatcher<MessageTypes>> {
        self.message_channel.borrow_mut()
    }

    pub fn id(&self) -> EntityId {
        self.base_id
    }
}