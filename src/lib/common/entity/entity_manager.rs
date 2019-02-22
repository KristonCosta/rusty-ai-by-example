use super::entity::Entity;
use crate::lib::common::entity::entity::EntityId;
use std::collections::HashSet;
use std::sync::mpsc::Sender;
use crate::lib::common::messaging::telegram::Telegram;
use crate::lib::common::messaging::message_dispatcher::MessageDispatcher;
use std::cell::RefCell;
use std::rc::Rc;


pub struct EntityManager<MessageType: Eq> {
    next_entity_id : EntityId,
    entities : Vec<Box<Entity<MessageType>>>,
    free_ids : Vec<EntityId>,
    message_channel : Rc<RefCell<MessageDispatcher<MessageType>>>,
}

impl <MessageType: Eq> EntityManager<MessageType> {
    pub fn new(message_channel:  Rc<RefCell<MessageDispatcher<MessageType>>>) -> Self {
        return EntityManager{
            next_entity_id : 0,
            entities: vec![],
            free_ids: vec![],
            message_channel
        }
    }

    pub fn new_entity<T : Entity<MessageType> + 'static>(&mut self) -> EntityId {
        let entity = T::new(self.next_entity_id, self.message_channel.clone());
        let next_id = if !self.free_ids.is_empty() {
            self.free_ids.pop().unwrap()
        } else {
            let next = self.next_entity_id;
            self.next_entity_id += 1;
            next
        };
        self.entities.insert(next_id, Box::new(entity));
        next_id
    }

    pub fn get_entity_from_id(&mut self, id: EntityId) -> Option<&mut Box<Entity<MessageType>>> {
        if self.entities.len() >= id {
            return Some(&mut self.entities[id])
        }
        None
    }
}