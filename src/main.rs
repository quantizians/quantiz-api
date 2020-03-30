#![feature(proc_macro_hygiene, decl_macro)]

#[cfg(test)] pub mod tests;

#[macro_use] extern crate rocket;
use rocket::Rocket;

#[macro_use]
extern crate dotenv_codegen;
use dotenv::dotenv;

#[get("/")]
fn index() -> &'static str {
    return "Quantiz index";
}

fn rocket() -> Rocket {
    return rocket::ignite().mount("/", routes![index]);   
}

fn main() {
    dotenv().ok();
    println!("DATABASE_URL: {}", dotenv!("DATABASE_URL"));
    rocket().launch();
}