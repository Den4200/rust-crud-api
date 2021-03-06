use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};

use crate::schema::heroes;

#[derive(Serialize, Queryable)]
pub struct Hero {
    pub id: i32,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}

#[table_name = "heroes"]
#[derive(AsChangeset, Deserialize, Insertable)]
pub struct NewHero<'a> {
    pub name: &'a str,
    pub identity: &'a str,
    pub hometown: &'a str,
    pub age: i32
}

#[table_name = "heroes"]
#[derive(AsChangeset, Deserialize)]
pub struct PartialHero {
    pub name: Option<String>,
    pub identity: Option<String>,
    pub hometown: Option<String>,
    pub age: Option<i32>
}

impl Hero {

    pub fn get(id: i32, conn: &SqliteConnection) -> Hero {
        heroes::table.find(id).first::<Hero>(conn).unwrap()
    }

    pub fn create(hero: NewHero, conn: &SqliteConnection) -> Hero {
        diesel::insert_into(heroes::table)
            .values(&hero)
            .execute(conn)
            .expect("Error creating new hero");

        heroes::table.order(heroes::id.desc()).first(conn).unwrap()
    }

    pub fn read(conn: &SqliteConnection) -> Vec<Hero> {
        heroes::table.order(heroes::id.asc()).load::<Hero>(conn).unwrap()
    }

    pub fn partial_update(id: i32, hero: &PartialHero, conn: &SqliteConnection) -> bool {
        diesel::update(heroes::table.find(id)).set(hero).execute(conn).is_ok()
    }

    pub fn update(id: i32, hero: &NewHero, conn: &SqliteConnection) -> bool {
        diesel::update(heroes::table.find(id)).set(hero).execute(conn).is_ok()
    }

    pub fn delete(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(heroes::table.find(id)).execute(conn).is_ok()
    }
}
