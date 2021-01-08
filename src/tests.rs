extern crate parking_lot;
extern crate rand;

use self::parking_lot::Mutex;
use self::rand::{distributions::Alphanumeric, thread_rng, Rng};
use super::models::food::Food;

use chrono::{Duration, Local};
use rocket::http::{ContentType, Status};
use rocket::local::Client;

static DB_LOCK: Mutex<()> = Mutex::new(());

macro_rules! run_test {
    (|$client:ident, $conn:ident| $block:expr) => {{
        let _lock = DB_LOCK.lock();
        let rocket = super::rocket();
        let db = super::DbConn::get_one(&rocket);
        let $client = Client::new(rocket).expect("Failed to init Rocket client");
        let $conn = db.expect("Failed to get DB connection");
        assert!(Food::delete_all(&$conn), "Failed to delete all foods");

        $block
    }};
}

#[test]
fn food_index_page() {
    run_test!(|client, conn| {
        // Ensure we can access index page.
        let res = client.get("/").dispatch();
        assert_eq!(res.status(), Status::Ok);

        // Ensure foods are sorted by expiry date.
        let food_num = 3;
        let mut rng = thread_rng();
        let mut names: Vec<String> = Vec::with_capacity(food_num);
        let dates = ["2020-02-01", "2020-05-31", "2020-01-01"];
        for date in &dates {
            let name: String = (&mut rng)
                .sample_iter(&Alphanumeric)
                .map(char::from)
                .take(6)
                .collect();
            let res = client
                .post("/")
                .header(ContentType::Form)
                .body(format!("name={}&expiry_date={}", &name, &date))
                .dispatch();
            assert_eq!(res.status(), Status::SeeOther);
            let mut cookies = res.headers().get("Set-Cookie");
            assert!(cookies.any(|v| v.contains("success")));
            names.push(name);
        }
        let mut res = client.get("/").dispatch();
        assert_eq!(res.status(), Status::Ok);
        let body = res.body_string().unwrap();
        assert!(body.find(&names[2]).unwrap() < body.find(&names[0]).unwrap());
        assert!(body.find(&names[0]).unwrap() < body.find(&names[1]).unwrap());

        // Ensure dangerous food has red string.
        let name_red: String = (&mut rng)
            .sample_iter(&Alphanumeric)
            .map(char::from)
            .take(6)
            .collect();
        let name_normal: String = (&mut rng)
            .sample_iter(&Alphanumeric)
            .map(char::from)
            .take(6)
            .collect();
        let date_red = Local::today().naive_local();
        let date_normal = date_red + Duration::days(15);
        let res = client
            .post("/")
            .header(ContentType::Form)
            .body(format!("name={}&expiry_date={}", &name_red, &date_red))
            .dispatch();
        assert_eq!(res.status(), Status::SeeOther);
        let res = client
            .post("/")
            .header(ContentType::Form)
            .body(format!(
                "name={}&expiry_date={}",
                &name_normal, &date_normal
            ))
            .dispatch();
        assert_eq!(res.status(), Status::SeeOther);

        let mut res = client.get("/").dispatch();
        assert_eq!(res.status(), Status::Ok);
        let body = res.body_string().unwrap();
        assert!(body.contains(&format!(r#"<td style="color: red">{}</td>"#, &name_red)));
        assert!(body.contains(&format!("<td>{}</td>", &name_normal)));
    })
}

#[test]
fn food_insertion_passed() {
    run_test!(|client, conn| {
        let inserted_name = "Natto";
        let inserted_date = "2020-05-31";

        let res = client
            .post("/")
            .header(ContentType::Form)
            .body(format!(
                "name={}&expiry_date={}",
                inserted_name, inserted_date
            ))
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
        let res = client.post("/").header(ContentType::Form).dispatch();
        assert_eq!(res.status(), Status::UnprocessableEntity);
        let mut cookies = res.headers().get("Set-Cookie");
        assert!(!cookies.any(|v| v.contains("warning")));

        // Without `expiry_date`.
        let res = client
            .post("/")
            .header(ContentType::Form)
            .body("name=a")
            .dispatch();
        assert_eq!(res.status(), Status::UnprocessableEntity);
        let mut cookies = res.headers().get("Set-Cookie");
        assert!(!cookies.any(|v| v.contains("warning")));

        // Without `name`.
        let res = client
            .post("/")
            .header(ContentType::Form)
            .body("expiry_date=2020-01-01")
            .dispatch();
        assert_eq!(res.status(), Status::UnprocessableEntity);
        let mut cookies = res.headers().get("Set-Cookie");
        assert!(!cookies.any(|v| v.contains("warning")));

        // Both fields are empty.
        let res = client
            .post("/")
            .header(ContentType::Form)
            .body("name=&expiry_date=")
            .dispatch();
        assert_eq!(res.status(), Status::SeeOther);
        let mut cookies = res.headers().get("Set-Cookie");
        assert!(cookies.any(|v| v.contains("warning")));

        // Empty `expiry_date`.
        let res = client
            .post("/")
            .header(ContentType::Form)
            .body("name=aaa&expiry_date=")
            .dispatch();
        assert_eq!(res.status(), Status::SeeOther);
        let mut cookies = res.headers().get("Set-Cookie");
        assert!(cookies.any(|v| v.contains("warning")));

        // Empty `name`.
        let res = client
            .post("/")
            .header(ContentType::Form)
            .body("name=&expiry_date=2020-02-29")
            .dispatch();
        assert_eq!(res.status(), Status::SeeOther);
        let mut cookies = res.headers().get("Set-Cookie");
        assert!(cookies.any(|v| v.contains("warning")));
    })
}

#[test]
fn food_deletion_passed() {
    run_test!(|client, conn| {
        // Create a food to be deleted.
        let res = client
            .post("/")
            .header(ContentType::Form)
            .body("name=deletiontest&expiry_date=2020-03-01")
            .dispatch();
        assert_eq!(res.status(), Status::SeeOther);
        let mut cookies = res.headers().get("Set-Cookie");
        assert!(cookies.any(|v| v.contains("success")));
        let id = Food::all(&conn)[0].id;

        // Delete the created food.
        let res = client.delete(format!("/{}", id)).dispatch();
        assert_eq!(res.status(), Status::SeeOther);
        let mut cookies = res.headers().get("Set-Cookie");
        assert!(cookies.any(|v| v.contains("success")));
        assert_eq!(Food::all(&conn).len(), 0);
    })
}
