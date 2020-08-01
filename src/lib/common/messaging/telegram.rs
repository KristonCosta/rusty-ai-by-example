use crate::lib::common::entity::entity::EntityId;
use std::cmp::Ordering;
use std::time::Instant;
use std::time::Duration;

pub struct Telegram<MessageType: Eq> {
    sender : EntityId,
    receiver : EntityId,
    message : MessageType,
    delay : Option<Duration>,
    dispatch_time : Option<Instant>,
}

pub struct TelegramBuilder<MessageType: Eq> {
    internal: Telegram<MessageType>
}

impl <MessageType: Eq> TelegramBuilder<MessageType> {
    pub fn new(sender: EntityId, receiver: EntityId, message: MessageType) -> TelegramBuilder<MessageType> {
        TelegramBuilder {
            internal: Telegram{
                sender,
                receiver,
                message,
                delay: None,
                dispatch_time: None,
            }
        }
    }

    pub fn set_delay(mut self, duration: Duration) -> Self {
        self.internal.delay = Some(duration);
        self
    }

    pub fn build(self) -> Telegram<MessageType> {
        self.internal
    }
}

impl <MessageType: Eq> Telegram<MessageType> {
    pub fn get_sender(&self) -> EntityId {
        self.sender
    }

    pub fn get_receiver(&self) -> EntityId {
        self.receiver
    }

    pub fn get_message(&self) -> &MessageType {
        &self.message
    }

    pub fn set_dispatch_time(&mut self, time : Instant) {
        self.dispatch_time = Some(time);
    }

    pub fn get_dispatch_time(&self) -> Option<Instant> {
        self.dispatch_time
    }

    pub fn get_delay(&self) -> Option<Duration> {
        self.delay
    }
}

impl <T: Eq> PartialEq for Telegram<T> {
    fn eq(&self, other: &Self) -> bool {
        self.dispatch_time == other.dispatch_time
        && self.sender == other.sender
        && self.receiver == other.receiver
        && self.message == other.message
    }
}

impl <T: Eq> Eq for Telegram<T> {}

impl <T: Eq> PartialOrd for Telegram<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq> Ord for Telegram<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dispatch_time.cmp(&other.dispatch_time)
    }
}