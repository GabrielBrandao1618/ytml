use std::collections::HashMap;

use pest::{iterators::Pair, Parser};

use crate::tokens::{Tag, TagInnerElement};

#[derive(pest_derive::Parser)]
#[grammar = "ytml/grammar/ytml.pest"]
struct YtmlParser {}

pub fn ytml_doc_to_ast(input: &str) -> Vec<Tag> {
    // Store all doc's root tags and then return it
    let mut doc_root_tags: Vec<Tag> = vec![];
    let mut pairs = YtmlParser::parse(Rule::doc, input).expect("Syntax error");

    let tags = pairs.next().expect("Syntax error").into_inner();
    for tag in tags {
        match tag.as_rule() {
            Rule::EOI => break,
            Rule::tag => {
                let mut unwrapped_tags = ytml_tag_to_ast(tag);
                doc_root_tags.append(&mut unwrapped_tags);
            }
            _ => unreachable!(),
        }
    }
    doc_root_tags
}

fn ytml_tag_to_ast(tag: Pair<Rule>) -> Vec<Tag> {
    let mut multiplier = 1;
    let mut tags = vec![];
    let mut initial_tag = Tag {
        name: String::new(),
        attributes: HashMap::new(),
        inner: vec![],
    };
    let mut tag_inner = tag.into_inner();
    let ast_tag_name = tag_inner.next().unwrap();
    let parsed_tag_name = ast_tag_name.as_str().to_owned();
    initial_tag.name = parsed_tag_name;

    let ast_tag_modifiers = tag_inner.next().unwrap();
    let parsed_modifiers = parse_tag_modifiers(ast_tag_modifiers.as_str());
    for (name, value) in parsed_modifiers {
        initial_tag.attributes.insert(name, value);
    }

    for tag_component in tag_inner {
        match tag_component.as_rule() {
            Rule::tag_props => {
                for (name, value) in parse_tag_props(tag_component.as_str()) {
                    initial_tag.attributes.insert(name, value);
                }
            }
            Rule::tag_inner => {
                for inner_element in tag_component.into_inner() {
                    match inner_element.as_rule() {
                        Rule::tag => {
                            let unwrapped_tags = ytml_tag_to_ast(inner_element);
                            for unwraped_tag in unwrapped_tags.into_iter() {
                                initial_tag.inner.push(TagInnerElement::Tag(unwraped_tag));
                            }
                        }
                        Rule::text => initial_tag
                            .inner
                            .push(TagInnerElement::Text(inner_element.as_str().to_owned())),
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!("Did not match: {:#?}", tag_component.as_rule()),
        }
    }
    for _ in 1..=multiplier {
        tags.push(initial_tag.clone());
    }
    tags
}

pub fn parse_tag_prop(input: &str) -> (String, String) {
    let ast = YtmlParser::parse(Rule::tag_prop, input)
        .unwrap()
        .next()
        .unwrap();
    let mut inner_ast = ast.into_inner();
    let ast_prop_name = inner_ast.next().unwrap();
    let parsed_prop_name = ast_prop_name.as_str().to_owned();

    let ast_prop_value = inner_ast.next().unwrap();
    let parsed_prop_value = ast_prop_value
        .into_inner()
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap()
        .as_str()
        .to_owned();

    (parsed_prop_name, parsed_prop_value)
}

pub fn parse_tag_props(input: &str) -> HashMap<String, String> {
    let mut props = HashMap::new();
    let ast = YtmlParser::parse(Rule::tag_props, input)
        .unwrap()
        .next()
        .unwrap();

    for ast_prop in ast.into_inner() {
        let (prop_name, prop_value) = parse_tag_prop(ast_prop.as_str());
        props.insert(prop_name, prop_value);
    }

    props
}

pub fn parse_tag_modifier(input: &str) -> (String, String) {
    let ast = YtmlParser::parse(Rule::tag_modifier, input)
        .unwrap()
        .next()
        .unwrap();
    match ast.as_rule() {
        Rule::tag_class => {
            let inner_class = ast.into_inner().next().unwrap();
            ("class".to_owned(), inner_class.as_str().to_owned())
        }
        Rule::tag_id => {
            let inner_class = ast.into_inner().next().unwrap();
            ("id".to_owned(), inner_class.as_str().to_owned())
        }
        Rule::tag_multiplier => todo!(),
        _ => unreachable!(),
    }
}

pub fn parse_tag_modifiers(input: &str) -> HashMap<String, String> {
    let mut modifiers = HashMap::new();
    let ast = YtmlParser::parse(Rule::tag_modifiers, input)
        .unwrap()
        .next()
        .unwrap();
    for modifier in ast.into_inner() {
        let (name, value) = parse_tag_modifier(modifier.as_str());
        modifiers.insert(name, value);
    }

    modifiers
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_prop() {
        let parsed = parse_tag_prop("color = \"red\"");
        assert_eq!(parsed, ("color".to_owned(), "red".to_owned()));
    }
    #[test]
    fn test_parse_props() {
        let parsed = parse_tag_props("(color=\"blue\" bg=\"red\")");
        let mut expected = HashMap::new();
        expected.insert("color".to_owned(), "blue".to_owned());
        expected.insert("bg".to_owned(), "red".to_owned());
        assert_eq!(parsed, expected);
    }
    #[test]
    fn test_parse_modifier() {
        let parsed = parse_tag_modifier(".container");
        assert_eq!(parsed, ("class".to_owned(), "container".to_owned()));

        let parsed = parse_tag_modifier("#unique");
        assert_eq!(parsed, ("id".to_owned(), "unique".to_owned()));
    }
    #[test]
    fn test_parse_modifiers() {
        let parsed = parse_tag_modifiers(".container#unique");
        let mut expected = HashMap::new();
        expected.insert("class".to_owned(), "container".to_owned());
        expected.insert("id".to_owned(), "unique".to_owned());
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse() {
        let raw_ytml =
            "html(lang = \"pt-br\"){ } body.container1#unique2(color = \"blue\"){p(color=\"red\"){content}}";
        let ast = ytml_doc_to_ast(raw_ytml);
        let root = ast.iter().nth(0).unwrap();
        let lang = root.attributes.get("lang").unwrap();
        assert_eq!(lang, "pt-br");

        // Disabling these tests for now

        let body = ast.iter().nth(1).unwrap();
        // Ensure that both class and id properties was parsed sucessfully
        let body_class = body.attributes.get("class").unwrap();
        let body_id = body.attributes.get("id").unwrap();

        assert_eq!(body_class, &String::from("container1"));
        assert_eq!(body_id, &String::from("unique2"));
    }
}
