extern crate secret_santa;
extern crate dotenv;


use secret_santa::database2;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Error loading the database url");
    
    let mut database2_sample = database2::new(database_url);

    let user = database2_sample.get_user(String::from("Aaa"));
    //println!("{}", user.unwrap().name);
}