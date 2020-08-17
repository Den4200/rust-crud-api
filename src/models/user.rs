use crate::auth::Auth;
use chrono::{Duration, Utc};
use crypto::scrypt::{scrypt_check, scrypt_simple, ScryptParams};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::result::{DatabaseErrorKind, Error};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::schema::users;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    
    #[serde(skip_serializing)]
    pub password: String
}

impl User {

    pub fn to_auth(&self, secret: &[u8]) -> UserAuth {
        let exp = Utc::now() + Duration::days(config::TOKEN_TTL);
        let token = Auth {
            id: self.id,
            email: self.email,
            exp: exp.timestamp()
        }
        .token(secret);

        UserAuth {
            email: self.email,
            token
        }
    }

    pub fn create(user: &NewUser, conn: &SqliteConnection) -> User {
        let password = &scrypt_simple(user.password, &ScryptParams::new(14, 8, 1)).expect("Password hasing error.");
        user = NewUser { password, ..user }

        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .expect("Error creating new hero");

        users::table.order(users::id.desc()).first(conn).unwrap()
    }
}

#[derive(Serialize)]
pub struct UserAuth<'a> {
    email: &'a str,
    token: String
}

#[table_name = "users"]
#[derive(Deserialize, Validates)]
pub struct NewUser {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String
}

pub enum UserCreationError {
    DuplicatedEmail
}

impl From<Error> for UserCreationError {
    fn from(err: Error) -> UserCreationError {
        if let Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = &err {
            if let Some("users_email_key") = info.constraint_name() {
                return UserCreationError::DuplicatedEmail;
            }
        }
        panic!("Error creating user: {:?}", err);
    }
}
