#![feature(proc_macro_hygiene, decl_macro)]

#[cfg(test)] pub mod tests;

#[macro_use] extern crate rocket;
use rocket::Rocket;

#[get("/")]
fn index() -> &'static str {
    return "Quantiz index";
}

fn rocket() -> Rocket {
    return rocket::ignite().mount("/", routes![index]);   
}

fn main() {
    rocket().launch();
}