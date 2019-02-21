use crate::lib::common::fsm::state::State;
use super::miner::Miner;
use super::map::Locations;
use crate::lib::common::fsm::state_transition::StateTransition;

pub struct EnterMineAndDigForNugget;
pub struct VisitBankAndDepositGold;
pub struct GoHomeAndSleepTilRested;
pub struct QuenchThirst;

use colored::*;

pub enum MinerStates{
    None,
}

impl State for EnterMineAndDigForNugget {
    type Entity = Miner;
    type Enum = MinerStates;

    fn new() -> Box<Self> {
        Box::new(EnterMineAndDigForNugget)
    }

    fn enter(&mut self,  miner: &mut Miner) {
        if miner.location() != Locations::Goldmine {
            println!(">> {}: Walkin' to the goldmine", miner.name());
            miner.change_location(Locations::Goldmine)
            use std::sync::mpsc::channel;
            let (sender, receiver) = channel();
        }
    }

    fn execute(&mut self, miner: &mut Miner) -> StateTransition<Self::Entity, Self::MessageType> {
        miner.add_to_gold_carried(1);
        miner.increase_fatigue();
        println!(">> {}: Pickin' up a nugget", miner.name());
        if miner.pocket_is_full() {
            return StateTransition::Switch(VisitBankAndDepositGold::new())
        }
        if miner.thirsty() {
            return StateTransition::Switch(QuenchThirst::new())
        }
        StateTransition::None
    }

    fn exit(&mut self, miner: &mut Miner) {
        println!(">> {}: Ah'm leavin' the goldmine with mah pockets full o' sweet gold", miner.name())
    }

    type MessageType = ();

    fn on_message(&mut self, entity: &mut Self::Entity, message: &Telegram<Self::MessageType>) {
        unimplemented!()
    }
}

impl State for VisitBankAndDepositGold {
    type Entity = Miner;
    type Enum = MinerStates;

    fn new() -> Box<Self> {
        Box::new(VisitBankAndDepositGold)
    }

    fn enter(&mut self, miner: &mut Miner) {
        if miner.location() != Locations::Bank {
            println!(">> {}: Goin' to the bank. Yes siree", miner.name());
            miner.change_location(Locations::Bank)
        }
    }

    fn execute(&mut self, miner: &mut Miner) -> StateTransition<Self::Entity, Self::Enum> {
        miner.add_to_wealth(miner.gold_carried());
        miner.set_gold_carried(0);
        println!(">> {}: Depositing gold. Total savings now: {:?}", miner.name(), miner.wealth());
        if miner.comfortable() {
            println!(">> {}: WooHoo! Rich enough for now. Back home I go", miner.name());
            return StateTransition::Switch(GoHomeAndSleepTilRested::new())
        } else {
            return StateTransition::Switch(EnterMineAndDigForNugget::new())
        }
        StateTransition::None
    }

    fn exit(&mut self, miner: &mut Miner) {
        println!(">> {}: Leavin' the bank", miner.name())
    }
}

impl State for GoHomeAndSleepTilRested {
    type Entity = Miner;
    type Enum = MinerStates;

    fn new() -> Box<Self> {
        Box::new(GoHomeAndSleepTilRested)
    }

    fn enter(&mut self, miner: &mut Miner) {
        if miner.location() != Locations::Shack {
            println!(">> {}: Walkin' home", miner.name());
            miner.change_location(Locations::Shack)
        }
    }

    fn execute(&mut self, miner: &mut Miner) -> StateTransition<Self::Entity, Self::Enum> {
        if !miner.fatigued() {
            println!(">> {}: Wad a God darn fantastic nap! Time to find more gold", miner.name());
            return StateTransition::Switch(EnterMineAndDigForNugget::new())
        } else {
            miner.decrease_fatigue();
            println!(">> {}: Zzzzzz....", miner.name())
        }
        StateTransition::None
    }

    fn exit(&mut self, miner: &mut Miner) {
        println!(">> {}: Leaving the house", miner.name())
    }
}

impl State for QuenchThirst {
    type Entity = Miner;
    type Enum = MinerStates;

    fn new() -> Box<Self> {
        Box::new(QuenchThirst)
    }

    fn enter(&mut self, miner: &mut Miner) {
        if miner.location() != Locations::Saloon {
            println!(">> {}: Boy, ah sure is thursty! Walkin' to the saloon", miner.name());
            miner.change_location(Locations::Saloon);
        }
    }

    fn execute(&mut self, miner: &mut Miner) -> StateTransition<Self::Entity, Self::Enum>  {
        if miner.thirsty() {
            miner.buy_drink_and_whiskey();
            println!(">> {}: That's a mighty fine sippin liquer", miner.name());
            return StateTransition::Switch(EnterMineAndDigForNugget::new())
        } else {
            panic!("WHY DID I GO TO QUENCH MY THIRST WHEN I WASN'T THIRSTY?!")
        }
        StateTransition::None
    }

    fn exit(&mut self, miner: &mut Miner) {
        println!(">> {}: Leaving the saloon, feelin' good", miner.name());
    }
}