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
            title: format!("Par acquit de {}", word.get_value()),
        },
    )
}
//
// #[get("/<word>")]
// fn word(word: &str, data: &State<Data>) -> Result<Template, Redirect> {
//     let result = data.get_word(word);
//
//     if result.is_err() {
//         match result.err().unwrap() {
//             error::Error::ThereIsNoWord => return Err(Redirect::to("/404")),
//             _ => {}
//         }
//     }
//
//     Ok(Template::render("index", context! {
//         value: result.unwrap().get_value(),
//         quote: result.unwrap().get_quote(),
//         keywords: result.unwrap().get_keywords(),
//         title: format!("Par acquit de {}", result.unwrap().get_value()),
//     }))
// }
//
//
// #[get("/page/ajouter-votre-expression")]
// fn add_word() -> Template {
//     Template::render("add_word", context! {
//         form: &Context::default(),
//         title: "Ajouter une expression",
//     })
// }

// #[post("/page/ajouter-votre-expression", data = "<form>")]
// fn post_add_word(data: &mut State<Data>, mut form: Form<Contextual<'_, AddWordForm>>) -> Result<Redirect, Template > {
//     // let value = form.value.as_mut().unwrap();
//     let ctx = &mut form.context;
//
//
//     if ctx.errors().collect::<Vec<&Error<'_>>>().is_empty() {
//         let word = Word::from(form.value.as_mut().unwrap());
//         let redirect_url = format!("/{}", word.get_value());
//
//         match data.add(word) {
//             Ok(_) => {
//                 return Ok(Redirect::to(uri!("/tamer")))
//             }
//             Err(e) => match e {
//                 error::Error::WordAlreadyExists => ctx.push_error(
//                     Error::validation("L'expression existe déjà").with_name("value")
//                 ),
//                 _ => {}
//             }
//         }
//     }
//
//     Err(Template::render("add_word", context! {
//         form: ctx,
//         title: "Ajouter une expression",
//     }))
// }

#[launch]
fn rocket() -> Rocket<Build> {
    println!("debug port: {}", Config::release_default().port);

    rocket::build()
        .manage(Data::from_path("data.csv"))
        .mount("/", FileServer::from("public/"))
        .mount(
            "/",
            routes![
                index,
                // add_word, post_add_word,
            ],
        )
        .attach(Template::fairing())
}
