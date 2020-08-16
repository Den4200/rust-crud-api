use rocket_contrib::databases::diesel;

#[database("diesel_sqlite_pool")]
pub struct Conn(diesel::SqliteConnection);
