use std::sync::Arc;
use std::time::{Duration, Instant};
use rdkafka::admin::{AdminClient, AdminOptions, AlterConfig, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use rdkafka::{ClientConfig, Message};
use rdkafka::admin::ResourceSpecifier::Broker;
use rdkafka::consumer::{BaseConsumer, CommitMode, Consumer};
use rdkafka::message::BorrowedMessage;
use tokio::sync::Mutex;

pub trait Aggregator {
    fn aggregate(&mut self, msg: &[u8]);
}

async fn kafka_aggregate<T: Aggregator>(state_remote: &Arc<Mutex<T>>, input_topic: &str) {
    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", "kafka:9092");
    config.set("message.timeout.ms", "5000");
    config.set("group.id", "test-group-2");
    config.set("enable.auto.commit", "false");
    config.set("auto.offset.reset", "beginning");

    let poll_timeout = Duration::from_secs(0);
    let poll_expand_timeout = Duration::from_secs(1);

    let msg_consumer: BaseConsumer = config.create().expect("Consumer creation error");
    let admin: AdminClient<DefaultClientContext> = config.create().expect("AdminClient creation error");

    admin.create_topics(
        &[
            NewTopic::new(input_topic, 1, TopicReplication::Fixed(1)),
            // NewTopic::new(state_topic, 1, TopicReplication::Fixed(1))
        ],
        &AdminOptions::new(),
    ).await.expect("Topic creation error");
    admin.alter_configs(
        &[
            AlterConfig::new(Broker(1))
                .set("log.retention.bytes", "100000000")
        ],
        &AdminOptions::new(),
    ).await.expect("Config alter error");
    msg_consumer.subscribe(&[input_topic]).unwrap();

    loop {
        let mut maybe_kafka_result = msg_consumer.poll(None);
        let mut messages: Vec<BorrowedMessage> = vec![];

        let start = Instant::now();
        let stop_max = start + poll_expand_timeout - poll_timeout;

        while let Some(kafka_result) = maybe_kafka_result
        {
            let msg: BorrowedMessage = kafka_result.unwrap();
            messages.push(msg);
            if Instant::now() >= stop_max {
                break;
            }
            maybe_kafka_result = msg_consumer.poll(poll_timeout);
        }

        {
            let mut state = state_remote.lock().await;
            for msg in messages {
                state.aggregate(msg.payload().unwrap());
            }
        }

        msg_consumer.commit_consumer_state(CommitMode::Sync).unwrap();
    }
}

pub struct KafkaAggregatorDaemon<T: Aggregator> {
    state_remote: Arc<Mutex<T>>
}

impl<T: Aggregator> KafkaAggregatorDaemon<T> {
    pub fn new(initial_state: T) -> Self {
        KafkaAggregatorDaemon { state_remote: Arc::new(Mutex::new(initial_state)) }
    }

    pub fn get_state_remote(&self) -> Arc<Mutex<T>> {
        self.state_remote.clone()
    }

    pub async fn run(&self, input_topic: &str) {
        kafka_aggregate(&self.state_remote, input_topic).await;
    }
}
