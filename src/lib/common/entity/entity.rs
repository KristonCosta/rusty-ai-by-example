use crate::lib::common::messaging::telegram::Telegram;
use std::sync::mpsc::Sender;
use std::rc::Rc;
use std::cell::RefCell;
use crate::lib::common::messaging::message_dispatcher::MessageDispatcher;

pub type EntityId = usize;

pub trait Entity<MessageType: Eq> {
    fn new(id: EntityId, dispatcher: Rc<RefCell<MessageDispatcher<MessageType>>>) -> Self where Self: Sized;
    fn get_id(&self) -> EntityId;
    fn update(&mut self);
    fn handle_message(&mut self, telegram : Telegram<MessageType>) -> bool;
}