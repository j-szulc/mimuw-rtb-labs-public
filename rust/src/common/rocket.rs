use std::sync::Arc;
use rdkafka::ClientConfig;
use rdkafka::producer::FutureProducer;
use thread_local::ThreadLocal;

pub struct KafkaProducerState(Arc<ThreadLocal<FutureProducer>>);

impl KafkaProducerState {
    pub fn new() -> Self {
        Self(Arc::new(ThreadLocal::new()))
    }

    pub fn get_producer(self: &Self) -> &FutureProducer {
        self.0.get_or(||
            ClientConfig::new()
                .set("bootstrap.servers", "kafka:9092")
                .set("message.timeout.ms", "5000")
                .create()
                .expect("Producer creation error")
        )
    }
}
