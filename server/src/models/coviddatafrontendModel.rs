/**
 * {file_name}
 * This file was generated by the db-schema-generator
 *
 * Happy Hacking!
**/
use crate::schema::coviddatafrontend;
use diesel::prelude::*;

#[derive(Debug, GraphQLObject, Insertable, Queryable)]
#[table_name = "coviddatafrontend"]
pub struct coviddatafrontendT {
    pub index: i32,
    pub version: Option<String>,
    pub date: Option<String>,
}

pub fn read(conn: &PgConnection) -> Vec<coviddatafrontendT> {
    coviddatafrontend::table
        .order(coviddatafrontend::date.asc())
        .load::<coviddatafrontendT>(conn)
        .expect("Error loading object")
}
