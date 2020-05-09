extern crate rocket;
extern crate rocket_contrib;
extern crate serde_derive;

use crate::DbConn;
use crate::models::food::Food;

use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
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

#[post("/", data = "<food_form>")]
pub fn new(food_form: Form<Food>, conn: DbConn) -> Flash<Redirect> {
    let food = food_form.into_inner();
    if food.name.is_empty() || food.expiry_date.is_empty() {
        Flash::warning(Redirect::to("/"), "Please input name and date.")
    } else if Food::insert(food, &conn) {
        Flash::success(Redirect::to("/"), "New food added.")
    } else {
        Flash::warning(Redirect::to("/"), "The server failed.")
    }
}

#[delete("/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Flash<Redirect> {
    if Food::delete(id, &conn) {
        Flash::success(Redirect::to("/"), "Your food is deleted.")
    } else {
        Flash::warning(Redirect::to("/"), "The server failed.")
    }
}
