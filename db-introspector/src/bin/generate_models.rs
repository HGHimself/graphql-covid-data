/**
 * generate_models.rs
 *
 * Provided a schema.rs file generated by diesel, further generate boilerplate code.
 * The schema is parsed into a struct(found in parser.rs)
 * which is then used to fill in a given template file
**/

use db_introspector::parser::parse_schema;
use db_introspector::generator::generate_models;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {

    let schema_string = open_file("./src/schema.rs");
    let template_string = open_file("./templates/model.txt");
    let models_folder_path = "./src/models";

    let schema = parse_schema(&schema_string).expect("Error parsing your schema!");

    let count = generate_models(&template_string, &schema, models_folder_path).expect("Error generating models!");

    println!("Generated {} models", count);
}

fn open_file(url: &str) -> String {
    let path = Path::new(url);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why);
    }

    return s;
}
