use super::miner::StatefulMiner;
use super::miner_wife::StatefulWife;


use std::{thread, time};
use crate::lib::common::entity::entity::Entity;
use crate::lib::common::entity::entity_manager;
use crate::lib::common::entity::entity::EntityId;

pub fn main() {

    let ref mut manager = entity_manager::EntityManager::new();
    manager.new_entity::<StatefulMiner>();
    manager.new_entity::<StatefulWife>();

    for _i in 0..200 {
        // manager.process_entity(miner);
        // manager.process_entity(wife);
        manager.process_all();
        thread::sleep(time::Duration::from_millis(1000));
    }
}