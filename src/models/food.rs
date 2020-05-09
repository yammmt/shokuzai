use diesel::{self, prelude::*};

mod schema {
    table! {
        foods {
            id -> Nullable<Integer>,
            name -> Text,
            expiry_date -> Text,
        }
    }
}

use self::schema::foods;
use self::schema::foods::dsl::foods as all_foods;

#[table_name="foods"]
#[derive(Clone, Debug, FromForm, Insertable, Serialize, Queryable)]
pub struct Food {
    pub id: Option<i32>,
    pub name: String,
    pub expiry_date: String,
}

impl Food {
    pub fn all(conn: &SqliteConnection) -> Vec<Food> {
        all_foods.load::<Food>(conn).unwrap()
    }

    pub fn insert(form: Food, conn: &SqliteConnection) -> bool {
        let f = Food { id: None, name: form.name, expiry_date: form.expiry_date };
        diesel::insert_into(foods::table).values(&f).execute(conn).is_ok()
    }

    #[cfg(test)]
    pub fn delete_all(conn: &SqliteConnection) -> bool {
        diesel::delete(all_foods).execute(conn).is_ok()
    }
}
