use super::miner_wife::MinerWife;

use crate::lib::common::fsm::state::State;
use crate::lib::common::fsm::state_transition::StateTransition;
use rand::Rng;
use crate::chapter1::part_three::message_types::MessageTypes;
use crate::lib::common::messaging::telegram::Telegram;
use crate::lib::common::messaging::telegram::TelegramBuilder;
use std::time::Duration;

pub struct WifesGlobalState;
pub struct DoHouseWork;
pub struct VisitBathroom;
pub struct CookStew;

impl State for CookStew {
    type Entity = MinerWife;
    type MessageType = MessageTypes;

    fn new() -> Box<Self> where Self: Sized {
        Box::new(CookStew)
    }

    fn enter(&mut self, entity: &mut Self::Entity) {
        if !entity.is_cooking() {
            println!(">> {}: Putting the stew in the oven", entity.name());
            let telegram = TelegramBuilder::new(entity.id()
                                                , entity.id()
                                                , MessageTypes::StewReady)
                .set_delay(Duration::new(1, 0))
                .build();
            entity.dispatch().send(telegram);
            entity.start_cooking();
        }
    }

    fn execute(&mut self, entity: &mut Self::Entity) -> StateTransition<Self::Entity, Self::MessageType> {
        println!(">> {}: Fussin' over food", entity.name());
        StateTransition::None
    }

    fn exit(&mut self, entity: &mut Self::Entity) {
        println!(">> {}: Puttin' the stew on the table", entity.name());
        entity.stop_cooking();
    }

    fn on_message(&mut self, entity: &mut Self::Entity, message: &Telegram<Self::MessageType>) -> (bool, StateTransition<Self::Entity, Self::MessageType>) {
        match message.get_message() {
            MessageTypes::StewReady => {
                println!(">> {}: StewReady! Lets eat", entity.name());
                let telegram = TelegramBuilder::new(
                    entity.id(),
                    entity.get_bob(),
                    MessageTypes::StewReady,
                ).build();
                entity.dispatch().send(telegram);
                (true, StateTransition::Switch(DoHouseWork::new()))
            }
            _ => (false, StateTransition::None)
        }
    }
}

impl State for WifesGlobalState {
    type Entity = MinerWife;
    type MessageType = MessageTypes;

    fn new() -> Box<Self> {
        Box::new(WifesGlobalState)
    }

    fn enter(&mut self,  wife: &mut MinerWife) {}

    fn execute(&mut self, wife: &mut MinerWife) -> StateTransition<Self::Entity, Self::MessageType> {
        if rand::thread_rng().gen_range(0,10) == 0 {
            return StateTransition::Push(VisitBathroom::new())
        }
        StateTransition::None
    }

    fn exit(&mut self, wife: &mut MinerWife) {}

    fn on_message(&mut self, entity: &mut Self::Entity, message: &Telegram<Self::MessageType>) -> (bool, StateTransition<Self::Entity, Self::MessageType>) {
        match message.get_message() {
            MessageTypes::HiHoneyImHome => {
                println!(">> {}: Hi honey. Let me make you some of mah fine country stew", entity.name());
                (true, StateTransition::Switch(CookStew::new()))
            }
            _ => (false, StateTransition::None)
        }
    }
}

impl State for DoHouseWork {
    type Entity = MinerWife;
    type MessageType = MessageTypes;

    fn new() -> Box<Self> {
        Box::new(DoHouseWork)
    }

    fn enter(&mut self, wife: &mut MinerWife) {}

    fn execute(&mut self, wife: &mut MinerWife) -> StateTransition<Self::Entity, Self::MessageType> {
        match rand::thread_rng().gen_range(0,3) {
            0 => {
                println!(">> {}: Moppin' the floor", wife.name());
            }
            1 => {
                println!(">> {}: Washin' the dishes", wife.name());
            },
            2 => {
                println!(">> {}: Makin' the bed", wife.name());
            },
            _ => {}
        }
        StateTransition::None
    }

    fn exit(&mut self, wife: &mut MinerWife) {}

    fn on_message(&mut self, entity: &mut Self::Entity, message: &Telegram<Self::MessageType>) -> (bool, StateTransition<Self::Entity, Self::MessageType>) {
        (false, StateTransition::None)
    }
}

impl State for VisitBathroom {
    type Entity = MinerWife;
    type MessageType = MessageTypes;

    fn new() -> Box<Self> {
        Box::new(VisitBathroom)
    }

    fn enter(&mut self, wife: &mut MinerWife) {
        println!(">> {}: Walkin' to the can. Need to powda mah pretty li'lle nose", wife.name());
    }

    fn execute(&mut self, wife: &mut MinerWife) -> StateTransition<Self::Entity, Self::MessageType> {
        println!(">> {}: Ahhhhhh! Sweet relief!", wife.name());
        StateTransition::Pop()
    }

    fn exit(&mut self, wife: &mut MinerWife) {
        println!(">> {}: Leavin' the Jon", wife.name())
    }

    fn on_message(&mut self, _entity: &mut Self::Entity, _message: &Telegram<Self::MessageType>) -> (bool, StateTransition<Self::Entity, Self::MessageType>) {
        (false, StateTransition::None)
    }
}