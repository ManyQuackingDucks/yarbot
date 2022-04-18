use crate::schema::points;
#[derive(Queryable, PartialEq, PartialOrd, Eq, Ord)]
pub struct UserQueryPoint {
    pub id: String,
    pub user_points: i32,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = points)]
pub struct UserInsertPoint<'a> {
    pub id: &'a str,
    pub user_points: i32,
}

use crate::schema::config;
#[derive(Queryable, PartialEq, PartialOrd, Eq, Ord)]
pub struct ConfigQuery {
    pub key: String,
    pub value: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = config)]
pub struct ConfigInsert<'a> {
    pub key: &'a str,
    pub value: &'a str,
}
