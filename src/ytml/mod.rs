use std::collections::HashMap;

use pest::{
    iterators::Pair,
    Parser,
};

use crate::ast::{Tag, TagInnerElement};

#[derive(pest_derive::Parser)]
#[grammar = "ytml/grammar/ytml.pest"]
pub struct YtmlParser {}

pub fn ytml_doc_to_ast(input: &str) -> Vec<Tag> {
    // Store all doc's root tags and then return it
    let mut doc_root_tags: Vec<Tag> = vec![];
    let mut pairs = YtmlParser::parse(Rule::doc, input).unwrap();

    let tags = pairs.next().unwrap().into_inner();
    for tag in tags {
        match tag.as_rule() {
            Rule::EOI => break,
            Rule::tag => {
                let unwrapped_tag = ytml_tag_to_ast(tag);
                doc_root_tags.push(unwrapped_tag);
            },
            _ => unreachable!(),
        }
    }
    doc_root_tags
}

pub fn ytml_tag_to_ast(tag: Pair<Rule>) -> Tag {
    let mut initial_tag = Tag {
        name: String::new(),
        attributes: HashMap::new(),
        inner: vec![],
    };
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
            Rule::tag_inner => {
                for inner_element in tag_component.into_inner() {
                    match inner_element.as_rule() {
                        Rule::tag => {
                            let unwrapped_tag = ytml_tag_to_ast(inner_element);
                            initial_tag
                                .inner
                                .push(TagInnerElement::Tag { tag: unwrapped_tag });
                        }
                        Rule::text => initial_tag.inner.push(TagInnerElement::Text {
                            content: inner_element.as_str().to_owned(),
                        }),
                        _ => unreachable!(),
                    }
                }
            }
            _ => println!("Did not match: {:#?}", tag_component.as_rule()),
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
            }
            Rule::prop_value => {
                prop_value = component.as_str().to_owned();
            }
            _ => unreachable!(),
        }
    }
    (prop_name, prop_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_out() {
        ytml_doc_to_ast("html(lang = \"pt-br\"){ }");
    }
}
