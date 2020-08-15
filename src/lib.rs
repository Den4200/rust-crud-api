#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod models;
mod routes;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/heroes",
            routes![
                routes::hero::create_hero,
                routes::hero::get_heroes,
                routes::hero::update_hero,
                routes::hero::delete_hero
            ]
        )
}
