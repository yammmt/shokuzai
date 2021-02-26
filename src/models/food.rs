use diesel::{self, prelude::*};

use crate::schema::foods;
use crate::schema::foods::dsl::foods as all_foods;

#[derive(Clone, Debug, Serialize, Queryable)]
pub struct Food {
    pub id: i32,
    pub name: String,
    pub expiry_date: String,
}

#[derive(Clone, Debug, FromForm, Insertable, Serialize)]
#[table_name = "foods"]
pub struct FoodForm {
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

    pub fn insert(form: FoodForm, conn: &SqliteConnection) -> bool {
        diesel::insert_into(foods::table)
            .values(&form)
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
