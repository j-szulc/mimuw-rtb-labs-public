#![allow(dead_code)]
#![allow(unused_variables)]
#[macro_use] extern crate rocket;

mod common;

use std::time::Duration;
use rdkafka::producer::FutureRecord;
use rocket::http::{ContentType, Status};
use rocket::serde::json::Json;
use rocket::State;
use crate::common::rocket::*;
use crate::common::data::*;
use crate::common::logic_case_two::get_shard;

#[post("/user_tags", data = "<user_tag_raw_json>")]
async fn add_user_tags(state: &State<KafkaProducerState>, user_tag_raw_json: Json<UserTagRaw>) -> Status {
        let user_tag_raw = user_tag_raw_json.into_inner();
        let key = user_tag_raw.cookie.clone();
        let topic = format!("user-tags-{}", get_shard(key.clone()));
        {
                let producer = state.get_producer();
                producer.send(
                        FutureRecord::to(&topic)
                            .payload(serde_json::to_string(&user_tag_raw).unwrap().as_bytes())
                            .key(key.as_bytes()),
                        Duration::from_secs(0),
                ).await.unwrap();
        }
        Status::NoContent
}

#[post("/user_profiles/<cookie>?<time_range>&<limit>", data="<debug>")]
fn get_user_profiles(cookie: String, time_range: String, limit: Option<String>, debug: String) -> (ContentType, String) {
        (ContentType::JSON, debug)
}

#[post("/aggregates?<time_range>&<action>&<aggregates>&<origin>&<brand_id>&<category_id>", data="<debug>")]
fn get_aggregates(time_range: String, action: String, aggregates: Vec<String>, origin: Option<String>, brand_id: Option<String>, category_id: Option<String>, debug: String) -> (ContentType, String) {
        (ContentType::JSON, debug)
}

#[launch]
async fn rocket() -> _ {
        rocket::build().manage(KafkaProducerState::new()).mount("/", routes![add_user_tags, get_user_profiles, get_aggregates])
}
