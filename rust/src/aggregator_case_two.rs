mod common;

#[macro_use] extern crate rocket;

use std::env;
use std::ops::Deref;
use std::sync::Arc;
use rocket::http::ContentType;
use rocket::serde::json::{Json, to_string};
use tokio::join;
use tokio::sync::Mutex;
use crate::common::kafka_aggregator::*;
use crate::common::logic_case_two::{LocalState, QueryResponse};

#[post("/user_profiles/<cookie>?<time_range>&<limit>", data="<debug>")]
async fn get_user_profiles(state: &rocket::State<Arc<Mutex<LocalState>>>, cookie: String, time_range: String, limit: Option<String>, debug: String) -> Json<QueryResponse> {
    Json::from(state.lock().await.query(cookie, time_range, limit))
}

#[launch]
async fn rocket() -> _ {
    let mut kafka_aggregator = KafkaAggregatorDaemon::new(LocalState::new());
    let state_remote = kafka_aggregator.get_state_remote();

    let input_topic = env::var("INPUT_TOPIC").unwrap();

    tokio::spawn(async move {
        kafka_aggregator.run(input_topic.as_str()).await;
    });

    // let state_remote_clone = state_remote.clone();
    // tokio::spawn(async move {
    //     loop {
    //         {
    //             let guard = state_remote_clone.lock().await;
    //             let state = guard.deref();
    //             println!("State: {:?}", &state);
    //         }
    //         tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    //     }
    // });


    rocket::build().manage(state_remote).mount("/", routes![get_user_profiles])
}
