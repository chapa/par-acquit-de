#[macro_use]
extern crate rocket;

mod data;
mod error;

use crate::data::Data;
use rocket::{Build, Rocket, State};

#[get("/")]
fn index(data: &State<Data>) -> String {
    format!("{:#?}", data.get_random_word())
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .manage(Data::from_path("data.csv"))
        .mount("/", routes![index])
}
