extern crate ytml;

use std::collections::HashMap;

use ytml::ast::{Tag, TagInnerElement};

fn main() {
    let p = Tag{
        attributes: HashMap::new(),
        name: String::from("p"),
        inner: vec![TagInnerElement::Text { content: String::from("This is a paragraph") }]
    };
    println!("{}", p);

    let div = Tag{
        attributes: HashMap::new(),
        name: String::from("div"),
        inner: vec![TagInnerElement::Tag { tag: p }]
    };
    println!("{}", div);
}