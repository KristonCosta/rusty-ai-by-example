mod entity_manager;
mod entity_names;
mod entity;
mod map;
mod state;
mod miner;
mod miner_states;

use std::{thread, time};

fn main() {
    let ref mut manager = entity_manager::EntityManager::new();
    let ref mut miner = manager.new_entity::<miner::StatefulMiner>();
    use entity::Entity;
    for _i in 0..200 {
        miner.update();
        thread::sleep(time::Duration::from_millis(1000));
    }
}