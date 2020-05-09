extern crate parking_lot;

use super::models::food::Food;
use self::parking_lot::Mutex;

use rocket::local::Client;
use rocket::http::Status;

static DB_LOCK: Mutex<()> = Mutex::new(());

macro_rules! run_test {
    (|$client:ident, $conn:ident| $block:expr) => ({
        let _lock = DB_LOCK.lock();
        let rocket = super::rocket();
        let db = super::DbConn::get_one(&rocket);
        let $client = Client::new(rocket).expect("Failed to init Rocket client");
        let $conn = db.expect("Failed to get DB connection");
        assert!(Food::delete_all(&$conn), "Failed to delete all foods");

        $block
    })
}

#[test]
fn food_index_page() {
    run_test!(|client, conn| {
        // Ensure we can access index page.
        let res = client.get("/").dispatch();
        assert_eq!(res.status(), Status::Ok);
    })
}
