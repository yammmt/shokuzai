extern crate parking_lot;

use super::models::food::Food;
use self::parking_lot::Mutex;

use rocket::local::Client;
use rocket::http::{ContentType, Status};

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

#[test]
fn food_insertion_passed() {
    run_test!(|client, conn| {
        let inserted_name = "Natto";
        let inserted_date = "2020-05-31";

        let res = client.post("/")
            .header(ContentType::Form)
            .body(format!("name={}&expiry_date={}", inserted_name, inserted_date))
            .dispatch();
        assert_eq!(res.status(), Status::SeeOther);
        let mut cookies = res.headers().get("Set-Cookie");
        assert!(cookies.any(|v| v.contains("success")));

        let foods = Food::all(&conn);
        assert_eq!(foods.len(), 1);
        assert_eq!(foods[0].name, inserted_name);
        assert_eq!(foods[0].expiry_date, inserted_date)
    })
}

#[test]
fn food_insertion_failed() {
    run_test!(|client, conn| {
        // Without body.
        let res = client.post("/")
            .header(ContentType::Form)
            .dispatch();
        assert_eq!(res.status(), Status::UnprocessableEntity);
        let mut cookies = res.headers().get("Set-Cookie");
        assert!(!cookies.any(|v| v.contains("warning")));

        // Without `expiry_date`.
        let res = client.post("/")
            .header(ContentType::Form)
            .body("name=a")
            .dispatch();
        assert_eq!(res.status(), Status::UnprocessableEntity);
        let mut cookies = res.headers().get("Set-Cookie");
        assert!(!cookies.any(|v| v.contains("warning")));

        // Without `name`.
        let res = client.post("/")
            .header(ContentType::Form)
            .body("expiry_date=2020-01-01")
            .dispatch();
        assert_eq!(res.status(), Status::UnprocessableEntity);
        let mut cookies = res.headers().get("Set-Cookie");
        assert!(!cookies.any(|v| v.contains("warning")));

        // Both fields are empty.
        let res = client.post("/")
            .header(ContentType::Form)
            .body("name=&expiry_date=")
            .dispatch();
        assert_eq!(res.status(), Status::SeeOther);
        let mut cookies = res.headers().get("Set-Cookie");
        assert!(cookies.any(|v| v.contains("warning")));

        // Empty `expiry_date`.
        let res = client.post("/")
            .header(ContentType::Form)
            .body("name=aaa&expiry_date=")
            .dispatch();
        assert_eq!(res.status(), Status::SeeOther);
        let mut cookies = res.headers().get("Set-Cookie");
        assert!(cookies.any(|v| v.contains("warning")));

        // Empty `name`.
        let res = client.post("/")
            .header(ContentType::Form)
            .body("name=&expiry_date=2020-02-29")
            .dispatch();
        assert_eq!(res.status(), Status::SeeOther);
        let mut cookies = res.headers().get("Set-Cookie");
        assert!(cookies.any(|v| v.contains("warning")));
    })
}
