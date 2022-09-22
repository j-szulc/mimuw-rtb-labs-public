mod common;

use std::ops::Deref;
use tokio::join;
use crate::common::kafka_aggregator::*;
use crate::common::logic_demo::State;

#[tokio::main]
async fn main() {
    let mut kafka_aggregator = KafkaAggregatorDaemon::new(State::new(0));
    let state_remote = kafka_aggregator.get_state_remote();
    let future1 = kafka_aggregator.run("aggregator-demo-test");
    let future2 = tokio::spawn(async move {
        // Process each socket concurrently.
        loop {
            {
                let guard = state_remote.lock().await;
                let state = guard.deref();
                println!("State: {:?}", &state);
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });
    join!(future1, future2);
}
