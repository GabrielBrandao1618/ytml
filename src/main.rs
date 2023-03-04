use std::collections::HashMap;

mod ast;
mod html;
mod ytml;

use ast::Tag;
use ytml::ytml_to_ast;

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
    let result = ytml_to_ast("html(lang = \"pt-br\"){{ }}");
    println!("{:#?}", result);
}
