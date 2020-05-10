#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate log;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod models;
mod routes;
#[cfg(test)] mod tests;

use chrono::{Duration, Local};
use rocket::{Rocket, fairing::AdHoc};
use rocket_contrib::{templates::{Template, tera::Error}, serve::StaticFiles};
use diesel::SqliteConnection;
use serde_json::value::Value;
use std::collections::HashMap;

embed_migrations!();

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = DbConn::get_one(&rocket).expect("Failed to connect to DB");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run DB migrations: {:?}", e);
            Err(rocket)
        }
    }
}

fn is_red_date<S: std::hash::BuildHasher>(map: HashMap<String, Value, S>) -> Result<Value, Error> {
    if let Some(Value::String(expiry_date)) = map.get("expiry_date") {
        let red_day = Local::now() + Duration::days(14);
        // For example, `"2020-05-31" > "2020-05-24 14:51:11.914438 +09:00"`.
        // String is compared as `Vec[u8]`.
        if expiry_date > &red_day.to_string() {
            Ok(Value::Bool(false))
        } else {
            Ok(Value::Bool(true))
        }
    } else {
        Err("Failed to parse `expiry_date`".into())
    }
}

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_attach("DB Migrations", run_db_migrations))
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![
            routes::food::index,
            routes::food::new,
            routes::food::delete,
        ])
        .attach(Template::custom(|engines| {
            engines.tera.register_function("is_red_date", Box::new(is_red_date));
        }))
}
fn main() {
    rocket().launch();
}
