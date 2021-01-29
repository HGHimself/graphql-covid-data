/**
 * {file_name}
 * This file was generated by the db-schema-generator
 *
 * Happy Hacking!
**/

use crate::schema::ZctaToModzcta;
use diesel::prelude::*;


#[derive(Debug,GraphQLObject,Insertable,Queryable,)]
#[table_name="ZctaToModzcta"]
pub struct ZctaToModzctaT {
    pub id: i32,
    pub ZCTA: Option<i64>,
    pub MODZCTA: Option<i64>,
}

pub fn read(conn: &PgConnection) -> Vec<ZctaToModzctaT> {
    ZctaToModzcta::table
        .order(ZctaToModzcta::date.asc())
        .load::<ZctaToModzctaT>(conn)
        .expect("Error loading object")
}