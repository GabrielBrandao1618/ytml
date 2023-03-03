use std::{fs::File, io::Write};

use super::ast::Tag;

pub fn ast_to_html(ast: &Tag, output_path: &str) {
    let mut file = File::create(output_path).unwrap();
    file.write(format!("{}", ast.name).as_bytes());
}
