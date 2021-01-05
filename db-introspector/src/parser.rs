/**
 * parser.rs
 *
 * this file will be used to parse a schema.rs file generated by diesel
 * turning it into the various structures defined below
**/

use std::fmt;

#[derive(Debug, Clone)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse the schema provided")
    }
}

type Result<T> = std::result::Result<T, ParseError>;

pub struct Table {
    pub name: String,
    pub primary_key: String,
    pub columns: Vec<Column>,
}

// name, type, nullable
pub type Column = (String, DataType);

pub type DataType = (String, bool);

pub fn parse_schema(tables: &str) -> Result<Vec<Table>> {
    parser::parse_schema(tables)
        .map(|t| t.1)
        .map_err(|_| ParseError)
}

pub(self) mod parser {
    use super::Table;
    use super::Column;
    use super::DataType;
    use nom::{
        branch::{alt, permutation},
        bytes::complete::{tag,take_till1},
        character::{
            is_space,
            complete::multispace0
        },
        combinator::map,
        multi::many1,
        sequence::{delimited,terminated},
        IResult,
    };

    // a column looks like demo_variable -> Nullable<Text>,
    fn parse_column_name(i: &str) -> IResult<&str, String> {
        terminated(
            map(take_till1( |c| is_space(c as u8) ), |s: &str| s.to_string()),
            tag(" -> ")
        )(i)
    }

    fn parse_type(i: &str) -> IResult<&str, String> {
        map(take_till1( |c| c == '>' || c == ','), |s: &str| s.to_string())(i)
    }

    fn parse_nullable(i: &str) -> IResult<&str, DataType> {
        delimited(
            tag("Nullable<"),
            map(parse_type, |s| (s, true)),
            tag(">")
        )(i)
    }

    fn parse_non_nullable(i: &str) -> IResult<&str, DataType> {
        map(parse_type, |s| (s, false))(i)
    }

    fn parse_data_type(i: &str) -> IResult<&str, DataType> {
        alt((parse_nullable, parse_non_nullable))(i)
    }

    fn parse_column(i: &str) -> IResult<&str, Column> {
        delimited(
            multispace0,
            permutation((
                parse_column_name,
                parse_data_type
            )),
            tag(",")
        )(i)
    }

    fn parse_columns(i: &str) -> IResult<&str, Vec<Column>> {
        delimited(
            permutation((tag("{"), multispace0)),
            many1(parse_column),
            permutation((tag("}"), multispace0))
        )(i)
    }

    fn parse_table_name(i: &str) -> IResult<&str, String> {
        map(take_till1( |c| is_space(c as u8) ), |s: &str| s.to_string())(i)
    }

    fn parse_table_key(i: &str) -> IResult<&str, String> {
        delimited(
            tag(" ("),
            map(take_till1( |c| c == ')' ), |s: &str| s.to_string()),
            tag(") ")
        )(i)
    }

    fn parse_table(i: &str) -> IResult<&str, (String, String, Vec<Column>)> {
        delimited(
            permutation((multispace0, tag("table! {"), multispace0)),
            permutation((
                parse_table_name,
                parse_table_key,
                parse_columns,
            )),
            permutation((multispace0, tag("}"))),
        )(i)
    }

    pub fn parse_schema(i: &str) -> IResult<&str, Vec<Table>> {
        many1(map(
            parse_table,
            |tuple| Table {
                name: tuple.0,
                primary_key: tuple.1,
                columns: tuple.2,
            }
        ))(i)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_column_name() {
            assert_eq!(
                parse_column_name("demo_variable -> "),
                Ok(("", String::from("demo_variable")))
            );
        }

        #[test]
        fn test_parse_data_type() {
            assert_eq!(
                parse_data_type("Text"),
                Ok(("", (String::from("Text"), false)))
            );

            assert_eq!(
                parse_nullable("Nullable<Text>"),
                Ok(("", (String::from("Text"), true)))
            );
            assert_eq!(
                parse_data_type("Nullable<Text>"),
                Ok(("", (String::from("Text"), true)))
            );
        }

        #[test]
        fn test_parse_column() {
            assert_eq!(
                parse_column("demo_variable -> Nullable<Text>,"),
                Ok(("", (String::from("demo_variable"), (String::from("Text"), true))))
            );
            assert_eq!(
                parse_column("demo_variable -> Text,"),
                Ok(("", (String::from("demo_variable"), (String::from("Text"), false))))
            );
        }

        #[test]
        fn test_parse_columns() {
            assert_eq!(
                parse_columns("{
                index -> Text,
                demo_variable -> Nullable<Text>,
                NUM_PEOP_TEST -> Nullable<Float8>,
            }"),
            Ok(("", vec![
                    (String::from("index"), (String::from("Text"), false)),
                    (String::from("demo_variable"), (String::from("Text"), true)),
                    (String::from("NUM_PEOP_TEST"), (String::from("Float8"), true))
                ]))
            );
        }

        #[test]
        fn test_parse_table_headers() {
            assert_eq!(
                parse_table_name("AntibodyByAge"),
                Ok(("", String::from("AntibodyByAge")))
            );
            assert_eq!(
                parse_table_key(" (index) "),
                Ok(("", String::from("index")))
            );
            assert_eq!(
                permutation((
                    parse_table_name,
                    parse_table_key,
                ))("AntibodyByAge (index) "),
                Ok(("", (String::from("AntibodyByAge"), String::from("index"))))
            );

        }

        #[test]
        fn test_parse_table() {
            assert_eq!(
                parse_table(
                    "table! {
                    AntibodyByAge (index) {
                        index -> Text,
                        demo_variable -> Nullable<Text>,
                        NUM_PEOP_TEST -> Nullable<Float8>,
                        NUM_PEOP_POS -> Nullable<Float8>,
                        PERCENT_POSITIVE -> Nullable<Float8>,
                        TEST_RATE -> Nullable<Float8>,
                        date -> Nullable<Text>,
                    }
                }"
                ),
                Ok(("", (String::from("AntibodyByAge"),
                String::from("index"),
                vec![
                    (String::from("index"), (String::from("Text"), false)),
                    (String::from("demo_variable"), (String::from("Text"), true)),
                    (String::from("NUM_PEOP_TEST"), (String::from("Float8"), true)),
                    (String::from("NUM_PEOP_POS"), (String::from("Float8"), true)),
                    (String::from("PERCENT_POSITIVE"), (String::from("Float8"), true)),
                    (String::from("TEST_RATE"), (String::from("Float8"), true)),
                    (String::from("date"), (String::from("Text"), true)),
                ],)))
            );
        }
    }
}
