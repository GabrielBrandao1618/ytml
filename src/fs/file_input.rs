use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::Tag;
use crate::ytml::ytml_doc_to_ast;

pub fn read_file_into_ast(file_path: &str) -> Vec<Tag> {
    let path = Path::new(file_path);
    let mut file = File::open(path).unwrap();

    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();
    let parsed_ast = ytml_doc_to_ast(&file_content);
    return parsed_ast;
}
