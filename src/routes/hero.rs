use rocket_contrib::json::{Json, JsonValue};

use crate::db;
use crate::models::hero::{Hero, NewHero, PartialHero};

#[get{"/<id>"}]
pub fn get_hero(id: i32, conn: db::Conn) -> JsonValue {
    json!(Hero::get(id, &conn))
}

#[get{"/"}]
pub fn get_heroes(conn: db::Conn) -> JsonValue {
    json!(Hero::read(&conn))
}

#[post("/", format = "json", data = "<hero>")]
pub fn post_hero(hero: Json<NewHero>, conn: db::Conn) -> Json<Hero> {
    let insert = NewHero { ..hero.into_inner() };
    Json(Hero::create(insert, &conn))
}

#[patch{"/<id>", format = "json", data = "<hero>"}]
pub fn patch_hero(id: i32, hero: Json<PartialHero>, conn: db::Conn) -> JsonValue {
    let update = PartialHero { ..hero.into_inner() };
    json!({
        "success": Hero::partial_update(id, &update, &conn)
    })
}

#[put{"/<id>", format = "json", data = "<hero>"}]
pub fn put_hero(id: i32, hero: Json<NewHero>, conn: db::Conn) -> JsonValue {
    let replacement = NewHero { ..hero.into_inner() };
    json!({
        "success": Hero::update(id, &replacement, &conn)
    })
}

#[delete("/<id>")]
pub fn delete_hero(id: i32, conn: db::Conn) -> JsonValue {
    json!({
        "success": Hero::delete(id, &conn)
    })
}
