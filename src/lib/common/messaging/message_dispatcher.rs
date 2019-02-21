use super::telegram::Telegram;
use std::collections::BinaryHeap;
use crate::lib::common::entity::entity_manager::EntityManager;
use crate::lib::common::entity::entity::Entity;
use std::sync::mpsc::Receiver;
use std::time::Instant;

struct MessageDispatcher<MessageType: Eq> {
    queue : BinaryHeap<Telegram<MessageType>>,
}

impl<MessageType: Eq> MessageDispatcher<MessageType> {

    pub fn new() -> MessageDispatcher<MessageType> {
        MessageDispatcher {
            queue: BinaryHeap::new()
        }
    }

    fn discharge(&mut self,
                 entity : &mut Box<Entity<MessageType>>,
                 telegram: Telegram<MessageType>) {
        entity.handle_message(telegram)
    }

    pub fn dispatch_message(&mut self,
                            entity_manager: &mut EntityManager<MessageType>,
                            mut telegram: Telegram<MessageType>) {
        let mut receiver = entity_manager.get_entity_from_id(telegram.get_receiver());
        if receiver.is_none() {
            return
        }

        let mut receiver = receiver.unwrap();

        if telegram.get_delay().is_none() {
            println!("Executing message ...");
            self.discharge(receiver, telegram);
        } else {
            telegram.set_dispatch_time(Instant::now());
            self.queue.push(telegram);
        }
    }

    pub fn dispatch_delayed_messages(&mut self,
                                     entity_manager: &mut EntityManager<MessageType>) {
        while !self.queue.is_empty() {
            let peeked = self.queue.peek().unwrap();
            if peeked.get_dispatch_time().unwrap().elapsed() < peeked.get_delay().unwrap() {
                break;
            }
            let next_message = self.queue.pop().unwrap();
            let receiver = entity_manager.get_entity_from_id(next_message.get_receiver());
            if receiver.is_some() {
                self.discharge(receiver.unwrap(), next_message);
            }
        }
    }
}

