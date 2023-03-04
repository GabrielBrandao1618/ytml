use std::collections::HashMap;

mod ast;
mod html;
mod ytml;

use ast::Tag;
use ytml::ytml_to_ast;

fn main() {
    let root = Tag {
        name: String::from("html"),
        attributes: HashMap::from([
            (String::from("href"), String::from("http://google.com")),
            (String::from("color"), String::from("blue")),
        ]),
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
    let result = ytml_to_ast("html(lang = \"pt-br\" color = \"blue\"){{ }}");
    println!("{}", result);
}
