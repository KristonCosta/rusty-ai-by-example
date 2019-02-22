use super::telegram::Telegram;
use std::collections::BinaryHeap;
use crate::lib::common::entity::entity::Entity;
use std::time::Instant;
use crate::lib::common::entity::entity_manager::EntityLookup;

pub struct MessageDispatcher<MessageType: Eq> {
    processing_queue: Vec<Telegram<MessageType>>,
}


pub struct MessageProcessor<MessageType: Eq> {
    queue : BinaryHeap<Telegram<MessageType>>
}

impl<MessageType: Eq> MessageDispatcher<MessageType> {
    pub fn new() -> MessageDispatcher<MessageType> {
        MessageDispatcher {
            processing_queue: vec![]
        }
    }

    pub fn send(&mut self, telegram: Telegram<MessageType>) {
        self.processing_queue.push(telegram);
    }

    pub fn process_queue(&mut self) -> Vec<Telegram<MessageType>> {
        self.processing_queue.drain(..).collect()
    }
}

impl<MessageType: Eq> MessageProcessor<MessageType> {
    pub fn new() -> Self {
        MessageProcessor {
            queue: BinaryHeap::new()
        }
    }
    fn discharge(&mut self,
                 entity : &mut Box<Entity<MessageType>>,
                 telegram: Telegram<MessageType>) {
        entity.handle_message(telegram);
    }

    pub fn dispatch_message(&mut self,
                            lookup: &mut EntityLookup<MessageType>,
                            mut telegram: Telegram<MessageType>) {
        let receiver = lookup.get_entity_from_id(&telegram.get_receiver());
        if receiver.is_none() {
            return
        }

        let receiver = receiver.unwrap();

        if telegram.get_delay().is_none() {
            self.discharge(receiver, telegram);
        } else {
            telegram.set_dispatch_time(Instant::now());
            self.queue.push(telegram);
        }
    }

    pub fn dispatch_delayed_messages(&mut self,
                                     lookup: &mut EntityLookup<MessageType>) {
        while !self.queue.is_empty() {
            let peeked = self.queue.peek().unwrap();
            if peeked.get_dispatch_time().unwrap().elapsed() < peeked.get_delay().unwrap() {
                break;
            }
            let next_message = self.queue.pop().unwrap();
            let receiver = lookup.get_entity_from_id(&next_message.get_receiver());
            if receiver.is_some() {
                self.discharge(receiver.unwrap(), next_message);
            }
        }
    }
}

