use super::entity::Entity;
use crate::lib::common::entity::entity::EntityId;

pub struct EntityManager<MessageType: Eq, ExtraInfo> {
    next_entity_id : EntityId,
    entities : Vec<Box<Entity<MessageType, ExtraInfo>>>
}

impl <MessageType: Eq, ExtraInfo> EntityManager<MessageType, ExtraInfo> {
    pub fn new() -> Self {
        return EntityManager{
            next_entity_id : 0,
            entities: vec![],
        }
    }

    pub fn new_entity<T : Entity<MessageType, ExtraInfo>>(&mut self) -> T {
        self.next_entity_id += 1;
        T::new(self.next_entity_id)
    }

    pub fn get_entity_from_id<'a>(id: EntityId) -> Option<&'a mut Box<Entity<MessageType, ExtraInfo>>> {
        None
    }
}