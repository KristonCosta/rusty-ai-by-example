use super::miner::StatefulMiner;
use super::miner_wife::StatefulWife;


use std::{thread, time};
use crate::lib::common::entity::entity::Entity;
use crate::lib::common::entity::entity_manager;
use std::sync::mpsc::channel;

pub fn main() {
    let (sender, receiver) = channel();
    let ref mut manager = entity_manager::EntityManager::new(sender);
    // let ref mut message_dispatcher =
    let miner = manager.new_entity::<StatefulMiner>();
    let wife = manager.new_entity::<StatefulWife>();
    for _i in 0..200 {
        manager.get_entity_from_id(miner).unwrap().update();
        manager.get_entity_from_id(wife).unwrap().update();
        thread::sleep(time::Duration::from_millis(1000));
    }
}