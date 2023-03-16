use std::collections::HashMap;

type Topic = String;
type Message = String;

struct YoutubeChannel {
    subscribers: HashMap<Topic, Vec<Box<dyn FnMut(&Message)>>>,
}

impl YoutubeChannel {
    fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
        }
    }

    fn subscribe<T>(&mut self, topic: Topic, mut subscriber: T)
    where
        T: FnMut(&Message) + 'static,
    {
        self.subscribers
            .entry(topic)
            .or_insert_with(Vec::new)
            .push(Box::new(subscriber));
    }
}

struct Publisher {}

impl Publisher {
    fn publish(&mut self, topic: Topic, message: Message, channel: &mut YoutubeChannel) {
        if let Some(subscribers) = channel.subscribers.get_mut(&topic) {
            for subscriber in subscribers {
                subscriber(&message);
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

        let mut publisher = Publisher {};
        publisher.publish("topic_a".to_string(), msg1.to_string(), &mut yt_channel);
        publisher.publish("topic_b".to_string(), msg2.to_string(), &mut yt_channel);
    }
}
