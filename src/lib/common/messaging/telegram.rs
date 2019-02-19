use crate::lib::common::entity::entity::EntityId;
use std::cmp::Ordering;

pub struct Telegram<MessageType: Eq, ExtraInfoEnum> {
    sender : EntityId,
    receiver : EntityId,
    message : MessageType,
    dispatch_time : i64,
    extra_info : ExtraInfoEnum,
}

impl <T: Eq, S> PartialEq for Telegram<T, S> {
    fn eq(&self, other: &Self) -> bool {
        self.dispatch_time == other.dispatch_time
        && self.sender == self.sender
        && self.receiver == self.receiver
        && self.message == self.message
    }
}

impl <T: Eq, S> Eq for Telegram<T, S> {}

impl <T: Eq, S> PartialOrd for Telegram<T, S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq, S> Ord for Telegram<T, S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dispatch_time.cmp(&other.dispatch_time)
    }
}