use diesel::{self, prelude::*};

use crate::schema::foods;
use crate::schema::foods::dsl::foods as all_foods;

#[table_name = "foods"]
#[derive(Clone, Debug, FromForm, Insertable, Serialize, Queryable)]
pub struct Food {
    pub id: Option<i32>,
    pub name: String,
    pub expiry_date: String,
}

impl Food {
    pub fn all(conn: &SqliteConnection) -> Vec<Food> {
        all_foods
            .order(foods::expiry_date.asc())
            .load::<Food>(conn)
            .unwrap()
    }

    pub fn insert(form: Food, conn: &SqliteConnection) -> bool {
        let f = Food {
            id: None,
            name: form.name,
            expiry_date: form.expiry_date,
        };
        diesel::insert_into(foods::table)
            .values(&f)
            .execute(conn)
            .is_ok()
    }

    pub fn delete(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(all_foods.find(id)).execute(conn).is_ok()
    }

    #[cfg(test)]
    pub fn delete_all(conn: &SqliteConnection) -> bool {
        diesel::delete(all_foods).execute(conn).is_ok()
    }
}
