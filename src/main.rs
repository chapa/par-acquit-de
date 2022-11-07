#[macro_use]
extern crate rocket;

use indexmap::IndexMap;
use rand::Rng;
use rocket::{Build, Rocket, State};

#[get("/")]
fn index(data: &State<Data>) -> String {
    let mut rng = rand::thread_rng();

    format!(
        "{:#?}",
        data.words.get_index(rng.gen_range(0..data.words.len()))
    )
}

#[derive(Debug)]
struct Word {
    quote: String,
    keywords: Vec<String>,
}

#[derive(Debug)]
struct Data {
    words: IndexMap<String, Word>,
    keywords: IndexMap<String, Vec<String>>,
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .manage(Data {
            words: IndexMap::from([
                (
                    String::from("conservation"),
                    Word {
                        quote: String::from("j'ai placé cette banane sous la cloche"),
                        keywords: vec![
                            String::from("fraicheur"),
                            String::from("fruit"),
                            String::from("mûr"),
                        ],
                    },
                ),
                (
                    String::from("consumérisme"),
                    Word {
                        quote: String::from("j'ai acheté cette lime à ongles électrique"),
                        keywords: vec![
                            String::from("consommation"),
                            String::from("capitalisme"),
                            String::from("gâchis"),
                        ],
                    },
                ),
                (
                    String::from("construction"),
                    Word {
                        quote: String::from("j'ai bâtis cette maison"),
                        keywords: vec![],
                    },
                ),
                (
                    String::from("confiture"),
                    Word {
                        quote: String::from("parce que c'est drôle"),
                        keywords: vec![String::from("fruit")],
                    },
                ),
            ]),
            keywords: IndexMap::from([
                (
                    String::from("capitalisme"),
                    vec![String::from("consumérisme")],
                ),
                (
                    String::from("consommation"),
                    vec![String::from("consumérisme")],
                ),
                (
                    String::from("fraicheur"),
                    vec![String::from("conservation")],
                ),
                (
                    String::from("fruit"),
                    vec![String::from("conservation"), String::from("confiture")],
                ),
                (String::from("gâchis"), vec![String::from("consumérisme")]),
                (String::from("mur"), vec![String::from("conservation")]),
            ]),
        })
        .mount("/", routes![index])
}
