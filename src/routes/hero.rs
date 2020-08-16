use rocket_contrib::json::{Json, JsonValue};

use crate::db;
use crate::models::hero::Hero;

#[post("/", format = "json", data = "<hero>")]
pub fn create_hero(hero: Json<Hero>, conn: db::Conn) -> Json<Hero> {
    let insert = Hero { ..hero.into_inner() };
    Json(Hero::create(insert, &conn))
}

#[get{"/"}]
pub fn get_heroes(conn: db::Conn) -> JsonValue {
    json!(Hero::read(&conn))
}

#[put("/<id>", format = "json", data = "<hero>")]
pub fn update_hero(id: i32, hero: Json<Hero>, conn: db::Conn) -> JsonValue {
    let update = Hero { id, ..hero.into_inner() };
    json!({
        "success": Hero::update(id, update, &conn)
    })
}

#[delete("/<id>")]
pub fn delete_hero(id: i32, conn: db::Conn) -> JsonValue {
    json!({
        "success": Hero::delete(id, &conn)
    })
}
