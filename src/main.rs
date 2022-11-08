#[macro_use]
extern crate rocket;

mod data;
mod error;

use crate::data::{Data, Word};
use rocket::{Build, Rocket, State};

#[get("/")]
fn index(data: &State<Data>) -> String {
    format!("{:#?}", data.get_random_word())
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .manage(Data::from([
            Word::create(
                "conservation",
                "j'ai placé cette banane sous la cloche",
                vec!["fraicheur", "fruit", "mûr"],
            ),
            Word::create(
                "consumérisme",
                "j'ai acheté cette lime à ongles électrique",
                vec!["consommation", "capitalisme", "gâchis"],
            ),
            Word::create("construction", "j'ai bâti cette maison", vec![]),
            Word::create("confiture", "parce que c'est drôle", vec!["fruit"]),
        ]))
        .mount("/", routes![index])
}
