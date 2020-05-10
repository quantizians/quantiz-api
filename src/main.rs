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
use semver;
use std::fs;
use toml;

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

fn version() -> semver::Version {
    let cargo_toml_str = fs::read_to_string("Cargo.toml").unwrap();
    let cargo_toml = cargo_toml_str.parse::<toml::Value>().unwrap();
    let version_string = cargo_toml["package"]["version"].as_str().unwrap();
    return semver::Version::parse(version_string).unwrap();
}

fn rocket() -> Rocket {
    let api_version = version();
    let server_root = format!("/api/v{}", api_version.major);
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
    rocket().launch();
}
