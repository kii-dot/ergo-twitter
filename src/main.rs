#[macro_use] extern crate rocket;

use routes::*;

mod routes;
mod models;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/",
    routes![
        index,
        tweet,
        reply,
        retweet,
        delete,
        create_profile,
        follow,
        feed
    ])
}