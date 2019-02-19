use super::entity_manager;
use super::miner;


use std::{thread, time};
use super::entity::Entity;

pub fn main() {
    let ref mut manager = entity_manager::EntityManager::new();
    let ref mut miner = manager.new_entity::<miner::StatefulMiner>();
    for _i in 0..200 {
        miner.update();
        thread::sleep(time::Duration::from_millis(1000));
    }
}