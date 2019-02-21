use crate::lib::common::messaging::telegram::Telegram;
use std::sync::mpsc::Sender;

pub type EntityId = usize;

pub trait Entity<MessageType: Eq> {
    fn new(id: EntityId, message_channel: Sender<MessageType>) -> Self where Self: Sized;
    fn update(&mut self);
    fn handle_message(&mut self, telegram : Telegram<MessageType>);
}