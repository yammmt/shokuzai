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
#[derive(Clone, Debug, Insertable, Serialize, Queryable)]
pub struct Food {
    pub id: Option<i32>,
    pub name: String,
    pub expiry_date: String,
}

impl Food {
    pub fn all(conn: &SqliteConnection) -> Vec<Food> {
        all_foods.load::<Food>(conn).unwrap()
    }

    #[cfg(test)]
    pub fn delete_all(conn: &SqliteConnection) -> bool {
        diesel::delete(all_foods).execute(conn).is_ok()
    }
}
