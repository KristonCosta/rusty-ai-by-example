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
    let message_dispatcher = Rc::new(RefCell::new(MessageDispatcher::new()));
    let mut message_handler = MessageProcessor::new();
    let ref mut manager = entity_manager::EntityManager::new(message_dispatcher.clone());
    let miner = manager.new_entity::<StatefulMiner>();
    let wife = manager.new_entity::<StatefulWife>();

    for _i in 0..200 {
        let entity = manager.get_entity_from_id(miner).unwrap();
        entity.update();
        let mut messages = message_dispatcher.borrow_mut().process_queue(manager);
        let mut itr = messages.drain(..);
        loop {
            match itr.next() {
                None => break,
                Some(message) => message_handler.dispatch_message( manager, message)
            }
        }
        manager.get_entity_from_id(wife).unwrap().update();
        message_dispatcher.borrow_mut().process_queue(manager);
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn process_entity(id: EntityId, )