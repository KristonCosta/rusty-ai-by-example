use crate::lib::common::messaging::telegram::Telegram;

pub type EntityId = i64;

pub trait Entity<MessageType: Eq, ExtraInfoEnum> {
    fn new(id: EntityId) -> Self where Self: Sized;
    fn update(&mut self);
    fn handle_message(&mut self, telegram : Telegram<MessageType, ExtraInfoEnum>);
}