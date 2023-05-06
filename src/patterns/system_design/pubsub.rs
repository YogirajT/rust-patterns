#![allow(dead_code)]
use std::collections::HashMap;

struct SubscriberFunctions(Box<dyn FnMut(&str)>);

struct YoutubeChannel {
    subscribers: HashMap<String, Vec<SubscriberFunctions>>,
}

impl YoutubeChannel {
    fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
        }
    }

    fn subscribe<T>(&mut self, topic: String, subscriber: T)
    where
        T: FnMut(&str) + 'static,
    {
        self.subscribers
            .entry(topic)
            .or_insert_with(Vec::new)
            .push(SubscriberFunctions(Box::new(subscriber)));
    }
}

struct Publisher {}

impl Publisher {
    fn publish(&self, topic: String, message: String, channel: &mut YoutubeChannel) {
        if let Some(subscribers) = channel.subscribers.get_mut(&topic) {
            for subscriber in subscribers {
                subscriber.0(&message);
            }
        }
    }
}

#[cfg(test)]
mod pubsub_tests {
    use super::{Publisher, YoutubeChannel};

    #[test]
    fn test_pubsub() {
        let mut yt_channel = YoutubeChannel::new();

        let msg1 = "message1";
        let msg2 = "message2";
        yt_channel.subscribe("topic_a".to_string(), move |message| {
            assert_eq!(msg1, message)
        });

        yt_channel.subscribe("topic_b".to_string(), move |message| {
            assert_eq!(msg2, message)
        });

        let publisher = Publisher {};
        publisher.publish("topic_a".to_string(), msg1.to_string(), &mut yt_channel);
        publisher.publish("topic_b".to_string(), msg2.to_string(), &mut yt_channel);
    }
}
