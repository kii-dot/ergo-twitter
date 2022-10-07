use serde_json::{Value, json};
use rocket::serde::json::Json;

use crate::models::*;

#[post("/tweet", format = "application/json", data="<message>")]
pub fn tweet(message: Json<Message>) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": message.message
    }))
}

#[post("/reply", format="application/json", data="<message>")]
pub fn reply(message: Json<Message>) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": message.message
    }))
}

#[post("/retweet", format="application/json", data="<message_content>")]
pub fn retweet(message_content: Json<MessageContent>) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": message_content.message
    }))
}

#[delete("/delete/<tweet_id>")]
pub fn delete(tweet_id: &str) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": tweet_id
    }))
}

#[post("/createProfile", format="application/json", data="<profile_details>")]
pub fn create_profile(profile_details: Json<ProfileDetails>) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": profile_details.address
    }))
}

#[put("/follow", format="application/json", data="<address>")]
pub fn follow(address: Json<Address>) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": address.address
    }))
}

#[get("/feed", format="application/json", data="<address>")]
pub fn feed(address: Json<Address>) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": address.address
    }))
}

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}
