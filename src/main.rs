use std::collections::HashMap;

mod ast;
use ast::Tag;

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
    println!("{}", root);
}
