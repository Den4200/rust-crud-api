use rocket_contrib::json::{Json, JsonValue}

use crate::models::hero::Hero;

#[post("/", format = "json", data = "<hero>")]
fn create_hero(hero: Json<Hero>) -> Json<Hero> {
    hero
}

#[get{"/"}]
fn get_heros() -> JsonValue {
    json!([
        "hero 1",
        "hero 2",
        "hero 3"
    ])
}

#[put("/<id>", format = "json", data = "<hero>")]
fn update_hero(id: u32, hero: Json<Hero>) -> Json<Hero> {
    hero
}

#[delete("/<id>")]
fn delete_hero(id: u32) -> JsonValue {
    json!({"status": "ok"})
}
