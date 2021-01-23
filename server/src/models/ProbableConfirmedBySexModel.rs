/**
 * {file_name}
 * This file was generated by the db-schema-generator
 *
 * Happy Hacking!
**/

use crate::schema::ProbableConfirmedBySex;
use diesel::prelude::*;


#[derive(Debug,GraphQLObject,Insertable,Queryable,)]
#[table_name="ProbableConfirmedBySex"]
pub struct ProbableConfirmedBySexT {
    pub id: i32,
    pub SEX: Option<String>,
    pub CONFIRMED_DEATH: Option<f64>,
    pub PROBABLE_DEATH: Option<f64>,
    pub date: Option<String>,
}

pub fn read(conn: &PgConnection) -> Vec<ProbableConfirmedBySexT> {
    ProbableConfirmedBySex::table
        .order(ProbableConfirmedBySex::date.asc())
        .load::<ProbableConfirmedBySexT>(conn)
        .expect("Error loading object")
}
