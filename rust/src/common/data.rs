use std::borrow::Borrow;
use rocket::serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use std::str::FromStr;

use crate::common::time::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(crate = "rocket::serde")]
pub struct ProductInfo {
    product_id: i32,
    brand_id: String,
    category_id: String,
    price: i32
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(crate = "rocket::serde")]
pub struct UserTagRaw {
    time: String,
    pub(crate) cookie: String,
    country: String,
    device: String,
    action: String,
    origin: String,
    product_info: ProductInfo
}

#[derive(EnumString, Serialize, Deserialize, Debug)]
pub enum Action {
    VIEW,
    BUY,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserTag {
    pub(crate) time: WrappedDateTimeUtc,
    pub(crate) cookie: String,
    country: String,
    device: String,
    pub(crate) action: Action,
    origin: String,
    product_info: ProductInfo,
    pub(crate) raw: UserTagRaw
}

impl From<UserTagRaw> for UserTag{
    fn from(raw: UserTagRaw) -> Self {
        let raw_cloned = raw.clone();
        UserTag {
            time: WrappedDateTimeUtc::parse(raw.time.as_str()).unwrap(),
            cookie: raw.cookie,
            country: raw.country,
            device: raw.device,
            action: Action::from_str(raw.action.as_str()).unwrap(),
            origin: raw.origin,
            product_info: raw.product_info,
            raw: raw_cloned
        }
    }
}