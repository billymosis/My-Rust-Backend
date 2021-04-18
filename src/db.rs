pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use dotenv::dotenv;
use std::env;

use models::*;
use schema::file_list;
use schema::file_list::dsl::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn get_files() -> Vec<FileList> {
    let connection = establish_connection();
    file_list.load::<FileList>(&connection).expect("ERROR")
}

pub fn insert_files() {
    let connection = establish_connection();
    let new_file = NewFile {
        file_name: "bacot.kimpoi",
        md5: "random",
        commit_message: "commit from app",
        version_number: 6,
        user_uploader: "app mantab",
    };

    diesel::insert_into(file_list::table)
        .values(&new_file)
        .execute(&connection)
        .expect("erroroaer");
}
