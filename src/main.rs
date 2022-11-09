#[macro_use]
extern crate rocket;

mod data;
mod error;

use crate::data::Data;
use rocket::{Build, Rocket, State};
use rocket::fs::FileServer;
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index(data: &State<Data>) -> Template {
    let word = data.get_random_word().unwrap();

    Template::render("index", context! {
        value: word.get_value(),
        quote: word.get_quote(),
        keywords: word.get_keywords(),
        title: format!("Par acquit de {}", word.get_value()),
        contentClasses: "top"
    })
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .manage(Data::from_path("data.csv"))
        .mount("/", FileServer::from("public/"))
        .mount("/", routes![index])
        .attach(Template::fairing())
}
