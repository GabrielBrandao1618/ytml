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
    for tag_component in tag.into_inner() {
        match tag_component.as_rule() {
            Rule::EOI => break,
            Rule::tag_name => {
                initial_tag.name = tag_component.as_str().to_owned();
            }
            Rule::tag_props => {
                for prop in tag_component.into_inner() {
                    let (prop_name, prop_val) = parse_tag_prop(prop.as_str());
                    initial_tag.attributes.insert(prop_name, prop_val);
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
            Rule::tag_multiplier => {
                let new_multiplier: u32 = tag_component
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap();
                multiplier = new_multiplier;
            }
            Rule::tag_class => {
                let class_name = tag_component
                    .into_inner()
                    .next()
                    .expect("Error: could not find class name")
                    .as_str();
                let mut full_classname = String::new();
                match initial_tag.attributes.get("class") {
                    Some(val) => full_classname = val.to_owned(),
                    None => (),
                }
                full_classname.push_str(&format!("{}", class_name));
                initial_tag
                    .attributes
                    .insert(String::from("class"), full_classname);
            }
            Rule::tag_id => {
                let id = tag_component
                    .into_inner()
                    .next()
                    .expect("Error: could not find id value")
                    .as_str();
                initial_tag
                    .attributes
                    .insert(String::from("id"), format!("{}", id.to_owned()));
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_prop() {
        let parsed = parse_tag_prop("color = \"red\"");
        assert_eq!(parsed, ("color".to_owned(), "red".to_owned()));
    }
    #[test]
    fn test_parse() {
        let raw_ytml =
            "html(lang = \"pt-br\"){ } body.container1#unique2(color = \"blue\"){p(color=\"red\"){content}}";
        let ast = ytml_doc_to_ast(raw_ytml);
        let root = ast.iter().nth(0).unwrap();
        let lang = root.attributes.get("lang").unwrap();
        assert_eq!(lang, "pt-br");

        let body = ast.iter().nth(1).unwrap();
        // Ensure that both class and id properties was persed sucessfully
        let body_class = body.attributes.get("class").unwrap();
        let body_id = body.attributes.get("id").unwrap();

        assert_eq!(body_class, &String::from("container1"));
        assert_eq!(body_id, &String::from("unique2"));
    }
}
