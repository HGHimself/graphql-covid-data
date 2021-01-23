/**
 * {file_name}
 * This file was generated by the db-schema-generator
 *
 * Happy Hacking!
**/

use crate::schema::ProbableConfirmedByAge;
use diesel::prelude::*;


#[derive(Debug,GraphQLObject,Insertable,Queryable,)]
#[table_name="ProbableConfirmedByAge"]
pub struct ProbableConfirmedByAgeT {
    pub id: i32,
    pub AGE_GROUP: Option<String>,
    pub CONFIRMED_DEATH: Option<f64>,
    pub PROBABLE_DEATH: Option<f64>,
    pub date: Option<String>,
}

pub fn read(conn: &PgConnection) -> Vec<ProbableConfirmedByAgeT> {
    ProbableConfirmedByAge::table
        .order(ProbableConfirmedByAge::date.asc())
        .load::<ProbableConfirmedByAgeT>(conn)
        .expect("Error loading object")
}