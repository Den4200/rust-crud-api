use rocket_contrib::json::{Json, JsonValue};

use crate::db;
use crate::models::hero::{Hero, NewHero, UpdatedHero};

#[post("/", format = "json", data = "<hero>")]
pub fn post_hero(hero: Json<NewHero>, conn: db::Conn) -> Json<Hero> {
    let insert = NewHero { ..hero.into_inner() };
    Json(Hero::create(insert, &conn))
}

#[get{"/"}]
pub fn get_heroes(conn: db::Conn) -> JsonValue {
    json!(Hero::read(&conn))
}

#[get{"/<id>"}]
pub fn get_hero(id: i32, conn: db::Conn) -> JsonValue {
    json!(Hero::get(id, &conn))
}

#[patch{"/<id>", format = "json", data = "<hero>"}]
pub fn patch_hero(id: i32, hero: Json<UpdatedHero>, conn: db::Conn) -> JsonValue {
    let update = UpdatedHero { ..hero.into_inner() };
    json!({
        "success": Hero::update(id, &update, &conn)
    })
}

#[delete("/<id>")]
pub fn delete_hero(id: i32, conn: db::Conn) -> JsonValue {
    json!({
        "success": Hero::delete(id, &conn)
    })
}
