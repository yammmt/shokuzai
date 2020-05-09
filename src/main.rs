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

use rocket::{Rocket, fairing::AdHoc};
use rocket_contrib::{templates::Template, serve::StaticFiles};
use diesel::SqliteConnection;

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

fn rocket() -> Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_attach("DB Migrations", run_db_migrations))
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![
            routes::food::index,
            routes::food::new,
        ])
        .attach(Template::fairing())
}
fn main() {
    rocket().launch();
}
