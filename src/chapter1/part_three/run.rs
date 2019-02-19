use super::miner::StatefulMiner;
use super::miner_wife::StatefulWife;


use std::{thread, time};
use crate::lib::common::entity::entity::Entity;
use crate::lib::common::entity::entity_manager;

pub fn main() {
    let ref mut manager = entity_manager::EntityManager::new();
    let ref mut miner = manager.new_entity::<StatefulMiner>();
    let ref mut wife = manager.new_entity::<StatefulWife>();
    for _i in 0..200 {
        miner.update();
        wife.update();
        thread::sleep(time::Duration::from_millis(1000));
    }
}