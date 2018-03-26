#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate juniper;

extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate dotenv;
extern crate serde;
extern crate serde_json;

pub mod schema;
pub mod models;
pub mod db;
pub mod gql;
