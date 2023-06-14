extern crate ytml;

use std::collections::HashMap;

use ytml::tokens::Tag;
use ytml::html::ast_tag_to_html;

fn main() {
    let t1 = Tag {
        name: String::from("html"),
        attributes: HashMap::new(),
        inner: Vec::new(),
    };
    let html = ast_tag_to_html(&t1, 0, 2);
    println!("{}", html);
}
