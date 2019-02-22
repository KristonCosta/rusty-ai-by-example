use super::miner::StatefulMiner;
use super::miner_wife::StatefulWife;


use std::{thread, time};
use crate::lib::common::entity::entity::Entity;
use crate::lib::common::entity::entity_manager;
use std::sync::mpsc::channel;
use crate::chapter1::part_three::miner::Miner;
use std::any::Any;
use crate::lib::common::messaging::message_dispatcher::MessageDispatcher;
use std::rc::Rc;
use std::cell::RefCell;
use crate::lib::common::messaging::message_dispatcher::MessageProcessor;
use crate::lib::common::entity::entity::EntityId;

pub fn main() {

    let ref mut manager = entity_manager::EntityManager::new();
    let miner = manager.new_entity::<StatefulMiner>();
    let wife = manager.new_entity::<StatefulWife>();

    for _i in 0..200 {
        // manager.process_entity(miner);
        // manager.process_entity(wife);
        manager.process_all();
        thread::sleep(time::Duration::from_millis(1000));
    }
}