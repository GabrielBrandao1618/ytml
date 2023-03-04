use std::collections::HashMap;

use pest::{
    iterators::{Pair, Pairs},
    Parser,
};

use crate::ast::Tag;

#[derive(pest_derive::Parser)]
#[grammar = "ytml/grammar/ytml.pest"]
pub struct YtmlParser {}

pub fn ytml_to_ast(input: &str) -> Tag {
    let pairs = YtmlParser::parse(Rule::tag, input).unwrap();
    let mut initial_tag = Tag {
        name: String::new(),
        attributes: HashMap::new(),
        inner: vec![],
    };
    for tag in pairs.into_iter() {
        for tag_component in tag.into_inner() {
            match tag_component.as_rule() {
                Rule::EOI => break,
                Rule::tag_name => {
                    initial_tag.name = tag_component.as_str().to_owned();
                }
                Rule::tag_props => {
                    for prop in tag_component.into_inner() {
                        let (prop_name, prop_val) = unwrap_tag_prop(prop);
                        initial_tag.attributes.insert(prop_name, prop_val);
                    }
                }
                _ => println!("Did not match"),
            }
        }
    }
    initial_tag
}

fn unwrap_tag_prop(prop: Pair<Rule>) -> (String, String) {
    let mut prop_name = String::new();
    let mut prop_value = String::new();
    for component in prop.into_inner() {
        match component.as_rule() {
            Rule::prop_name => {
                prop_name = component.as_str().to_owned();
            },
            Rule::prop_value => {
                prop_value = component.as_str().to_owned();
            },
            _ => unreachable!()
        }
    }
    (prop_name, prop_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_out() {
        ytml_to_ast("html(lang = \"pt-br\"){{ }}");
    }
}
