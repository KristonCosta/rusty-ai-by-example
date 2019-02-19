use super::entity;

pub struct EntityManager {
    next_entity_id : i64
}

impl EntityManager {
    pub fn new() -> Self {
        return EntityManager{
            next_entity_id : 0,
        }
    }

    pub fn new_entity<T : entity::Entity>(&mut self) -> T {
        self.next_entity_id += 1;
        T::new(self.next_entity_id)
    }
}