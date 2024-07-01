use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::tokens::Tag;
use crate::ytml::parse_ytml_file;

pub fn read_file_into_ast(file_path: &str) -> Vec<Tag> {
    let path = Path::new(file_path);
    let mut file = File::open(path).expect("Could not open the file");

    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Could not read the file");
    let parsed_ast = parse_ytml_file(&file_content);
    return parsed_ast;
}
