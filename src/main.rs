#![allow(clippy::needless_return, clippy::module_inception)]
#![feature(proc_macro_hygiene, decl_macro)]
#[cfg(test)]
pub mod tests;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
use dotenv::dotenv;
use rocket::fairing::AdHoc;
use rocket::Rocket;
// #[macro_use] extern crate dotenv_codegen;
// use dotenv_codegen::dotenv;
mod db;
mod models;
mod responses;
mod routers;
use db::DbConnection;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = DbConnection::get_one(&rocket).expect("invalid database connection");
    match diesel_migrations::run_pending_migrations(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            panic!("Failed to run database migrations: {}", e.to_string());
        }
    }
}

fn rocket() -> Rocket {
    let api_version = std::env::var("API_VERSION").expect("invalid API version");
    let server_root = format!("/api/{}", api_version);
    return rocket::ignite()
        .attach(DbConnection::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .mount(
            &routers::tasks::root(&server_root),
            routers::tasks::handlers(),
        );
}

fn main() {
    dotenv().ok();
    // let pg = connect_db();
    // let results = tasks::table
    //     .limit(5)
    //     .load::<Task>(&pg)
    //     .expect("Error loading tasks");

    // println!("Displaying {} tasks", results.len());
    // for task in results {
    //     println!("{}", task.title);
    // }

    rocket().launch();
}

// fn rocket_db() -> Rocket {
// use rocket::config::{Config, Environment, Value};
// use std::collections::HashMap;
//     // loads in env vars from .env
//     dotenv().ok();
//     // Procedurally configs database
//     // quantiz_db = { url = "$DATABASE_URL" }
//     let mut database_config = HashMap::new();
//     let mut databases = HashMap::new();
//     let database_url = std::env::var("DATABASE_URL").unwrap();
//     database_config.insert(
//         "url",
//         Value::from(database_url)
//     );

//     databases.insert(
//         "quantiz_db",
//         Value::from(database_config)
//     );
//     let env = Environment::active().unwrap();
//     let config = Config::build(env)
//         .extra("databases", databases)
//         .finalize()
//         .unwrap();

//     return rocket::custom(config).attach(DbConnection::fairing());
// }

// fn connect_db() -> PgConnection {
//     let database_url = std::env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
//     return PgConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url));
// }
