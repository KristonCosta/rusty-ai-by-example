use super::entity::Entity;
use crate::lib::common::entity::entity::EntityId;



use crate::lib::common::messaging::message_dispatcher::MessageDispatcher;
use std::cell::RefCell;
use std::rc::Rc;
use crate::lib::common::messaging::message_dispatcher::MessageProcessor;
use std::collections::HashMap;


pub struct EntityManager<MessageType: Eq> {
    next_entity_id : EntityId,
    entity_lookup : EntityLookup<MessageType>,
    free_ids : Vec<EntityId>,
    dispatcher : Rc<RefCell<MessageDispatcher<MessageType>>>,
    processor : MessageProcessor<MessageType>

}

pub struct EntityLookup<MessageType: Eq> {
    entities : HashMap<EntityId, Box<Entity<MessageType>>>,
}

impl<MessageType: Eq> EntityLookup<MessageType> {
    pub fn get_entity_from_id(&mut self, id: &EntityId) -> Option<&mut Box<Entity<MessageType>>> {
        self.entities.get_mut(id)

    }
}

impl <MessageType: Eq> EntityManager<MessageType> {
    pub fn new() -> Self {
        let message_dispatcher = Rc::new(RefCell::new(MessageDispatcher::new()));
        let message_handler = MessageProcessor::new();
        return EntityManager{
            next_entity_id : 0,
            entity_lookup: EntityLookup {
                entities: HashMap::new()
            },
            free_ids: vec![],
            dispatcher: message_dispatcher,
            processor: message_handler,
        }
    }

    pub fn new_entity<T : Entity<MessageType> + 'static>(&mut self) -> EntityId {
        let entity = T::new(self.next_entity_id, self.dispatcher.clone());
        let next_id = if !self.free_ids.is_empty() {
            self.free_ids.pop().unwrap()
        } else {
            let next = self.next_entity_id;
            self.next_entity_id += 1;
            next
        };
        self.entity_lookup.entities.insert(next_id, Box::new(entity));
        next_id
    }


    pub fn process_entity(&mut self, id: &EntityId) {
        match self.entity_lookup.get_entity_from_id(id) {
            None => {},
            Some(entity) => {
                entity.update();
                let mut messages = self.dispatcher.borrow_mut().process_queue();
                let mut itr = messages.drain(..);
                loop {
                    match itr.next() {
                        None => break,
                        Some(message) => self.processor.dispatch_message( &mut self.entity_lookup, message)
                    }
                }
            }
        }
    }

    pub fn process_all(&mut self) {
        let entities = self.entity_lookup.entities.keys().cloned().collect::<Vec<_>>();
        for key in entities {
            self.process_entity(&key);
        }
        self.processor.dispatch_delayed_messages(&mut self.entity_lookup);
    }
}