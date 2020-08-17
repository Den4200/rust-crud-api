use rocket::fairing::AdHoc;
use rocket::config::{Config, Environment, Value};
use std::collections::HashMap;
use std::env;

/// Debug-only secret for JWT encoding & decoding.
const SECRET: &'static str = "UhL2qJixHytkintEoTidJWaf9tzn0pDrSurEWYArbW8";

/// Token expiration time (days)
const TOKEN_TTL: i64 = 365;

pub struct AppState {
    pub secret: Vec<u8>
}

impl AppState {
    pub fn manage() -> AdHoc {
        AdHoc::on_attach("Manage config", |rocket| {
            let secret = env::var("SECRET_KEY").unwrap_or_else(|err| {
                if cfg!(debug_assertions) {
                    SECRET.to_string()
                } else {
                    panic!("SECRET_KEY environment variable not found: {:?}", err)
                }
            });

            Ok(rocket.manage(AppState { secret: secret.into_bytes() }))
        })
    }
}


/// Creates rocket config from environment variables.
pub fn from_env() -> Config {
    let environment = Environment::active().expect("No environment found.");

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable not found.");

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable not found.");

    database_config.insert("url", Value::from(database_url));
    databases.insert("diesel_sqlite_pool", Value::from(database_config));

    Config::build(environment)
        .environment(environment)
        .port(port)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}
