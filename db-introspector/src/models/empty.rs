
use crate::schema::AntibodyByAge;
use diesel::prelude::*;
use diesel::dsl::Eq;


#[derive(Debug,GraphQLObject,Insertable,Queryable,)]
#[table_name="AntibodyByAge"]
pub struct AntibodyByAgeT {
    pub index: String,
    pub demo_variable: Option<String>,
    pub NUM_PEOP_TEST: Option<f64>,
    pub NUM_PEOP_POS: Option<f64>,
    pub PERCENT_POSITIVE: Option<f64>,
    pub TEST_RATE: Option<f64>,
    pub date: Option<String>,
}

pub fn read(conn: &PgConnection) -> Vec<AntibodyByAgeT> {
    AntibodyByAge::table
        .load::<AntibodyByAgeT>(conn)
        .expect("Error loading object")
}
