extern crate ytml;

use std::collections::HashMap;

use ytml::tokens::{Tag, TagInnerElement};

fn main() {
    let p = Tag {
        attributes: HashMap::new(),
        name: String::from("p"),
        inner: vec![TagInnerElement::Text(String::from("This is a paragraph"))],
    };
    println!("{}", p);

    let div = Tag {
        attributes: HashMap::new(),
        name: String::from("div"),
        inner: vec![TagInnerElement::Tag(p)],
    };
    println!("{}", div);
}

