#[macro_use] extern crate rocket;
use rand::distributions::{Distribution, Uniform};

mod common;

use rocket::http::{ContentType, Status};
use rocket::serde::json::{Json, from_str};
use rocket::State;
use rocket::request::{FromRequest, Request, Outcome};
// use reqwest::ClientBuilder;
use common::logic_case_two::get_shard;

#[derive(Debug)]
struct RequestWrapper {
    uri: String
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestWrapper {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let uri = request.uri().to_string();
        Outcome::Success(RequestWrapper{
            uri
        })
    }
}

impl RequestWrapper {
    async fn post(&self, body: String, host: &str) -> Result<String, reqwest::Error> {
        let client = reqwest::Client::new();
        let full_uri = host.to_owned() + &self.uri;
        dbg!(&full_uri);
        let res = client.post(full_uri)
            .body(body)
            .send()
            .await?;
        let body = res.text().await?;
        Ok(body)
    }
}

#[post("/user_tags", data = "<body>")]
async fn add_user_tags(req: RequestWrapper, body: String) -> Status {
    let host = format!("http://mimuw-rtb-labs-pds-5-case-one-{}:8001", Uniform::from(1..=8).sample(&mut rand::thread_rng()));
    req.post(body, &host).await.unwrap();
    Status::NoContent
}

#[post("/user_profiles/<cookie>", data="<body>")]
async fn get_user_profiles(req: RequestWrapper, body: String, cookie: String) -> (ContentType, String) {
    let shard = get_shard(cookie);
    let host = format!("http://case-two-{}:8002", shard);
    let actual = req.post(body.clone(), &host).await.unwrap();
    let expected = body;

    // let actual_obj = from_str::<serde_json::Value>(&actual).unwrap();
    // let expected_obj = from_str::<serde_json::Value>(&expected).unwrap();
    // dbg!(&actual_obj);
    // dbg!(&expected_obj);
    // assert_eq!(&actual_obj, &expected_obj);

    (ContentType::JSON, actual)
}

#[post("/aggregates?<time_range>&<action>&<aggregates>&<origin>&<brand_id>&<category_id>", data="<debug>")]
fn get_aggregates(time_range: String, action: String, aggregates: Vec<String>, origin: Option<String>, brand_id: Option<String>, category_id: Option<String>, debug: String) -> (ContentType, String) {
    (ContentType::JSON, debug)
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![add_user_tags, get_user_profiles, get_aggregates])
}
