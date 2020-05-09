extern crate rocket;
extern crate rocket_contrib;
extern crate serde_derive;

use crate::DbConn;
use crate::models::food::Food;

use rocket::request::FlashMessage;
use rocket_contrib::templates::Template;

#[derive(Debug, Serialize)]
struct IndexContext<'a, 'b> { msg: Option<(&'a str, &'b str)>, foods: Vec<Food> }

impl<'a, 'b> IndexContext<'a, 'b> {
    pub fn raw(conn: &DbConn, msg: Option<(&'a str, &'b str)>) -> IndexContext<'a, 'b> {
        IndexContext{ msg, foods: Food::all(conn) }
    }
}

#[get("/")]
pub fn index(msg: Option<FlashMessage>, conn: DbConn) -> Template {
    Template::render("food/index", &match msg {
        Some(ref msg) => IndexContext::raw(&conn, Some((msg.name(), msg.msg()))),
        None => IndexContext::raw(&conn, None),
    })
}
