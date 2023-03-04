use std::collections::HashMap;

mod ast;
mod html;
use ast::Tag;
use html::ast_to_html;

fn main() {
    let root = Tag {
        name: String::from("html"),
        attributes: HashMap::from([("href", "http://google.com"), ("color", "blue")]),
        inner: vec![
            Box::new(Tag {
                name: String::from("head"),
                attributes: HashMap::new(),
                inner: vec![],
            }),
            Box::new(Tag {
                name: String::from("title"),
                attributes: HashMap::new(),
                inner: Vec::new(),
            }),
        ],
    };
    let html_out = ast_to_html(&root, 0);
    println!("{}", html_out);
}
