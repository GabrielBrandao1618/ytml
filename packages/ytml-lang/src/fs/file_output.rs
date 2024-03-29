use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::tokens::Tag;
use crate::html::ast_to_html;

pub fn write_html_to_file(file_path: &str, ast: Vec<Tag>, indent: usize) {
    let path = Path::new(file_path);
    let mut output_file = File::create(path).ok().unwrap();
    let html_content = ast_to_html(ast, indent);
    output_file.write_all(html_content.as_bytes()).unwrap();
}
