#[macro_use]
extern crate rocket;

mod data;
mod error;

use crate::data::{AddWordForm, Data, Word};
use rocket::form::{Context, Contextual, Error, Form};
use rocket::fs::FileServer;
use rocket::response::Redirect;
use rocket::{Build, Config, Rocket, State};
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index(data: &State<Data>) -> Template {
    let word = data.get_random_word().unwrap();

    Template::render(
        "index",
        context! {
            value: word.get_value(),
            quote: word.get_quote(),
            keywords: word.get_keywords(),
        },
    )
}

#[get("/<word>")]
fn word(word: &str, data: &State<Data>) -> Option<Template> {
    match data.get_word(word) {
        Ok(word) => Some(Template::render(
            "word",
            context! {
                value: word.get_value(),
                quote: word.get_quote(),
                keywords: word.get_keywords(),
            },
        )),
        Err(_) => None,
    }
}

#[get("/page/ajouter-votre-expression")]
fn add_word() -> Template {
    Template::render(
        "add_word",
        context! {
            form: &Context::default(),
        },
    )
}

#[post("/page/ajouter-votre-expression", data = "<form>")]
fn post_add_word(
    data: &State<Data>,
    mut form: Form<Contextual<'_, AddWordForm>>,
) -> Result<Redirect, Template> {
    if let Some(ref value) = form.value {
        let word = Word::from(value);
        let redirect_url = format!("/{}", word.get_value());

        match data.add(word) {
            Ok(_) => return Ok(Redirect::to(redirect_url)),
            Err(e) => match e {
                error::Error::WordAlreadyExists => form
                    .context
                    .push_error(Error::validation("L'expression existe déjà").with_name("value")),
                e => form.context.push_error(Error::validation(format!(
                    "Une erreur imprévue est survenue ({:?})",
                    e
                ))),
            },
        }
    }

    Err(Template::render(
        "add_word",
        context! {
            form: &form.context,
        },
    ))
}

#[launch]
fn rocket() -> Rocket<Build> {
    println!("debug port: {}", Config::release_default().port);

    rocket::build()
        .manage(Data::from_path("data.csv"))
        .mount("/public", FileServer::from("public/"))
        .mount("/", routes![index, word, add_word, post_add_word,])
        .attach(Template::fairing())
}
