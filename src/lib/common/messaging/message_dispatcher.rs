use super::telegram::Telegram;
use std::collections::BinaryHeap;
use crate::lib::common::entity::entity_manager::EntityManager;
use crate::lib::common::entity::entity::Entity;

struct MessageDispatcher<MessageType: Eq, ExtraInfo> {
    entity_manager: EntityManager<MessageType, ExtraInfo>,
    queue : BinaryHeap<Telegram<MessageType, ExtraInfo>>
}

impl<MessageType: Eq, ExtraInfo> MessageDispatcher<MessageType, ExtraInfo> {
    pub fn discharge(entity : Box<Entity<MessageType, ExtraInfo>>, telegram: Telegram<MessageType, ExtraInfo>) {

    }
}