#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

use dotenv::dotenv;

mod auth;
mod config;
mod db;
mod models;
mod routes;
mod schema;

pub fn rocket() -> rocket::Rocket {
    dotenv().ok();

    rocket::custom(config::from_env())
        .mount(
            "/heroes",
            routes![
                routes::hero::get_hero,
                routes::hero::get_heroes,
                routes::hero::post_hero,
                routes::hero::patch_hero,
                routes::hero::put_hero,
                routes::hero::delete_hero
            ]
        )
        .attach(db::Conn::fairing())
        .attach(config::AppState::manage())
}
