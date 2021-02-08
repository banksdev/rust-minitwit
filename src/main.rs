#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket_contrib::templates::Template;

#[derive(serde::Serialize)]
struct TemplateContext {
    name: String,
    tweets: Vec<&'static str>
}

#[get("/")]
fn index() -> Template {
    let context = TemplateContext { name: String::from("Alexander"), tweets: vec!["a","b","c"]  };
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
    }
  