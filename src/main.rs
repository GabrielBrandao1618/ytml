use std::collections::HashMap;

mod ast;
mod html;
mod ytml;

use ast::{Tag, TagInnerElement};
use ytml::ytml_tag_to_ast;

fn main() {
    let root = Tag {
        name: String::from("html"),
        attributes: HashMap::from([
            (String::from("href"), String::from("http://google.com")),
            (String::from("color"), String::from("blue")),
        ]),
        inner: vec![TagInnerElement::Tag {
            tag: (Tag {
                name: String::from("html"),
                attributes: HashMap::new(),
                inner: vec![],
            }),
        }],
    };
    let raw_ytml = "html(lang = \"pt-br\") { content } ";
    let result = ytml_tag_to_ast(raw_ytml);
    println!("{}", result);
}
