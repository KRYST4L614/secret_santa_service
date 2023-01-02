/*extern crate secret_santa;
extern crate dotenv;


use secret_santa::database2;
use dotenv::dotenv;
use std::env;*/
pub mod models;
pub mod service;
pub mod schema;

fn main() {

}

/*fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Error loading the database url");
    
    let mut database2_sample = database2::new(database_url);
    let user = database2_sample.insert_user(String::from("NIGGERS_SUCKS"));
    match user {
        None => {},
        Some(usr) => println!("Success: {}, {}", usr.id, usr.name),
    }
}*/