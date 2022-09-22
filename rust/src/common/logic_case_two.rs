use std::collections::HashMap;
use crate::common::data::{Action, UserTagRaw};
use crate::common::nmax::*;
use crate::common::data::UserTag;
use crate::common::time::TimeRange;
use crate::common::kafka_aggregator::Aggregator;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

pub fn get_shard(cookie: String) -> u32 {
    let mut shard = 0;
    for c in cookie.chars() {
        shard += c as u32;
    }
    shard % 2
}

#[derive(Debug)]
pub struct UserState {
    views: NMax<UserTag>,
    buys: NMax<UserTag>,
}

#[derive(Debug)]
pub struct LocalState {
    user_states: HashMap<String, UserState>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct QueryResponse {
    cookie: String,
    buys: Vec<UserTagRaw>,
    views: Vec<UserTagRaw>,
}

impl LocalState {
    pub fn new() -> Self {
        LocalState { user_states: HashMap::new() }
    }

    pub fn get_user_state_mut(&mut self, cookie: String) -> &mut UserState {
        self.user_states.entry(cookie).or_insert(UserState { buys: NMax::new(200), views: NMax::new(200) })
    }

    pub fn add_user_tag(&mut self, user_tag: UserTag) {
        let user_state = self.get_user_state_mut(user_tag.cookie.clone());
        match user_tag.action {
            Action::BUY => user_state.buys.push(user_tag),
            Action::VIEW => user_state.views.push(user_tag),
        };
    }

    pub fn query(&self, cookie: String, time_range_str: String, limit_str: Option<String>) -> QueryResponse {
        let time_range = TimeRange::parse(time_range_str.as_str());
        let limit = limit_str.map(|s| s.parse::<usize>().unwrap()).unwrap_or(2);

        let user_state = self.user_states.get(&cookie).unwrap();

        let mut views: Vec<UserTagRaw> = user_state.views
            .iter()
            .filter(|user_tag| time_range.contains(&user_tag.time))
            .take(limit.clone())
            .map(|user_tag| user_tag.raw.to_owned())
            .collect();

        views.dedup_by(|a, b| a == b);

        let mut buys: Vec<UserTagRaw> = user_state.buys
            .iter()
            .filter(|user_tag| time_range.contains(&user_tag.time))
            .take(limit)
            .map(|user_tag| user_tag.raw.to_owned())
            .collect();

        buys.dedup_by(|a, b| a == b);

        QueryResponse { cookie, buys, views }

    }
}

impl Aggregator for LocalState {
    fn aggregate(&mut self, msg: &[u8]) {
        let user_tag_raw: UserTagRaw = serde_json::from_slice(msg).unwrap();
        // dbg!(&user_tag_raw);
        let user_tag: UserTag = UserTag::from(user_tag_raw);
        self.add_user_tag(user_tag);
    }
}