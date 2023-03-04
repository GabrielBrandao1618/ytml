use std::collections::HashMap;

mod ast;
mod html;
mod ytml;

use ast::{Tag, TagInnerElement};
use ytml::{ytml_tag_to_ast,ytml_doc_to_ast};
use html::ast_to_html;

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
    let raw_ytml = "html(lang = \"pt-br\") { content test } body(){ p(color = \"blue\"){}} ";
    let result = ytml_doc_to_ast(raw_ytml);
    for tag in result {
        let html_parsed = ast_to_html(&tag, 0);
        println!("{html_parsed}");
    }
}
