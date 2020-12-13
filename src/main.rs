#![feature(proc_macro_hygiene, decl_macro)]
use rocket::*;
use rocket::response::content::Html;

#[get("/echo/<echo>")]
fn echo_fn(echo: String) -> String {
    format!("{}", echo)
}

#[get("/")]
fn hello_fn() -> Html<String> {
    Html("<h1>Hello, world!</h1><p>From inside a DigitalOcean App</p>".into())
}

fn main() {
    rocket::ignite().mount("/", routes![echo_fn, hello_fn]).launch();
}
