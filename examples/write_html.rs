extern crate ytml;

use std::collections::HashMap;
use ytml::{file_handling::file_output::write_html_to_file, ast::Tag};

fn main() {
    let document = vec![
        Tag{
            attributes: HashMap::new(),
            inner: Vec::new(),
            name: String::from("html"),
        }
    ];
    let file_path = "./out.html";
    write_html_to_file(file_path, document, 2);
}