use std::collections::HashMap;

use pest::Parser;

use crate::tokens::{Tag, TagInnerElement};

use self::error::{YtmlError, YtmlErrorKind, YtmlResult};

pub mod error;

#[derive(pest_derive::Parser)]
#[grammar = "ytml/grammar/ytml.pest"]
struct YtmlParser {}

pub fn parse_ytml_file(input: &str) -> YtmlResult<Vec<Tag>> {
    // Store all doc's root tags and then return it
    let mut doc_root_tags: Vec<Tag> = vec![];
    let mut pairs =
        YtmlParser::parse(Rule::doc, input).map_err(|_| YtmlError::new(YtmlErrorKind::Parsing))?;

    let tags = pairs
        .next()
        .ok_or(YtmlError::new(YtmlErrorKind::Parsing))?
        .into_inner();
    for tag in tags {
        match tag.as_rule() {
            Rule::EOI => break,
            Rule::tag => {
                let parsed_tag = parse_tag(tag.as_str())?;
                doc_root_tags.push(parsed_tag);
            }
            _ => unreachable!(),
        }
    }
    Ok(doc_root_tags)
}

fn parse_tag(input: &str) -> YtmlResult<Tag> {
    let ast = YtmlParser::parse(Rule::tag, input)
        .map_err(|_| YtmlError::new(YtmlErrorKind::Parsing))?
        .next()
        .ok_or(YtmlError::new(YtmlErrorKind::Parsing))?;
    let mut ast_inner = ast.into_inner();
    let ast_tag_name = ast_inner
        .next()
        .ok_or(YtmlError::new(YtmlErrorKind::Parsing))?;
    let parsed_tag_name = ast_tag_name.as_str().to_owned();

    let ast_tag_modifiers = ast_inner
        .next()
        .ok_or(YtmlError::new(YtmlErrorKind::Parsing))?;
    let mut tag_attributes = parse_tag_modifiers(ast_tag_modifiers.as_str())?;

    let ast_props = ast_inner
        .next()
        .ok_or(YtmlError::new(YtmlErrorKind::Parsing))?;

    tag_attributes.extend(parse_tag_props(ast_props.as_str())?);

    let tag_inner = ast_inner
        .next()
        .ok_or(YtmlError::new(YtmlErrorKind::Parsing))?;
    let parsed_inner = parse_tag_inner(tag_inner.as_str())?;

    let parsed_tag = Tag {
        name: parsed_tag_name,
        attributes: tag_attributes,
        inner: parsed_inner,
    };

    Ok(parsed_tag)
}

pub fn parse_tag_prop(input: &str) -> YtmlResult<(String, String)> {
    let ast = YtmlParser::parse(Rule::tag_prop, input)
        .map_err(|_| YtmlError::new(YtmlErrorKind::Parsing))?
        .next()
        .ok_or(YtmlError::new(YtmlErrorKind::Parsing))?;
    let mut inner_ast = ast.into_inner();
    let ast_prop_name = inner_ast
        .next()
        .ok_or(YtmlError::new(YtmlErrorKind::Parsing))?;
    let parsed_prop_name = ast_prop_name.as_str().to_owned();

    let ast_prop_value = inner_ast
        .next()
        .ok_or(YtmlError::new(YtmlErrorKind::Parsing))?;
    let parsed_prop_value = ast_prop_value
        .into_inner()
        .next()
        .ok_or(YtmlError::new(YtmlErrorKind::Parsing))?
        .into_inner()
        .next()
        .ok_or(YtmlError::new(YtmlErrorKind::Parsing))?
        .as_str()
        .to_owned();

    Ok((parsed_prop_name, parsed_prop_value))
}

pub fn parse_tag_props(input: &str) -> YtmlResult<HashMap<String, String>> {
    let mut props = HashMap::new();
    let ast = YtmlParser::parse(Rule::tag_props, input)
        .map_err(|_| YtmlError::new(YtmlErrorKind::Parsing))?
        .next()
        .ok_or(YtmlError::new(YtmlErrorKind::Parsing))?;

    for ast_prop in ast.into_inner() {
        let (prop_name, prop_value) = parse_tag_prop(ast_prop.as_str())?;
        props.insert(prop_name, prop_value);
    }

    Ok(props)
}

pub fn parse_tag_modifier(input: &str) -> YtmlResult<(String, String)> {
    let ast = YtmlParser::parse(Rule::tag_modifier, input)
        .map_err(|_| YtmlError::new(YtmlErrorKind::Parsing))?
        .next()
        .ok_or(YtmlError::new(YtmlErrorKind::Parsing))?;
    let inner_value = ast
        .clone()
        .into_inner()
        .next()
        .ok_or(YtmlError::new(YtmlErrorKind::Parsing))?;
    match ast.as_rule() {
        Rule::tag_class => Ok(("class".to_owned(), inner_value.as_str().to_owned())),
        Rule::tag_id => Ok(("id".to_owned(), inner_value.as_str().to_owned())),
        Rule::tag_multiplier => Ok(("multiplier".to_owned(), inner_value.as_str().to_owned())),
        _ => unreachable!(),
    }
}

pub fn parse_tag_modifiers(input: &str) -> YtmlResult<HashMap<String, String>> {
    let mut modifiers = HashMap::new();
    let ast = YtmlParser::parse(Rule::tag_modifiers, input)
        .map_err(|_| YtmlError::new(YtmlErrorKind::Parsing))?
        .next()
        .ok_or(YtmlError::new(YtmlErrorKind::Parsing))?;
    for modifier in ast.into_inner() {
        let (name, value) = parse_tag_modifier(modifier.as_str())?;
        modifiers.insert(name, value);
    }

    Ok(modifiers)
}

pub fn parse_tag_inner(input: &str) -> YtmlResult<Vec<TagInnerElement>> {
    let mut inner_elements = vec![];
    let ast = YtmlParser::parse(Rule::tag_inner, input)
        .map_err(|_| YtmlError::new(YtmlErrorKind::Parsing))?
        .next()
        .ok_or(YtmlError::new(YtmlErrorKind::Parsing))?;
    for inner_element in ast.into_inner() {
        match inner_element.as_rule() {
            Rule::text => {
                inner_elements.push(TagInnerElement::Text(inner_element.as_str().to_owned()));
            }
            Rule::tag => {
                let parsed_tag = parse_tag(inner_element.as_str())?;
                inner_elements.push(TagInnerElement::Tag(parsed_tag));
            }
            _ => unreachable!(),
        }
    }
    Ok(inner_elements)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_prop() {
        let parsed = parse_tag_prop("color = \"red\"").unwrap();
        assert_eq!(parsed, ("color".to_owned(), "red".to_owned()));
    }
    #[test]
    fn test_parse_props() {
        let parsed = parse_tag_props("(color=\"blue\" bg=\"red\")").unwrap();
        let mut expected = HashMap::new();
        expected.insert("color".to_owned(), "blue".to_owned());
        expected.insert("bg".to_owned(), "red".to_owned());
        assert_eq!(parsed, expected);
    }
    #[test]
    fn test_parse_modifier() {
        let parsed = parse_tag_modifier(".container").unwrap();
        assert_eq!(parsed, ("class".to_owned(), "container".to_owned()));

        let parsed = parse_tag_modifier("#unique").unwrap();
        assert_eq!(parsed, ("id".to_owned(), "unique".to_owned()));
    }
    #[test]
    fn test_parse_modifiers() {
        let parsed = parse_tag_modifiers(".container#unique").unwrap();
        let mut expected = HashMap::new();
        expected.insert("class".to_owned(), "container".to_owned());
        expected.insert("id".to_owned(), "unique".to_owned());
        assert_eq!(parsed, expected);
    }
    #[test]
    fn test_parse_tag_inner() {
        let parsed = parse_tag_inner("Lorem Ipsum").unwrap();
        assert_eq!(
            parsed,
            vec![TagInnerElement::Text("Lorem Ipsum".to_owned())]
        );
    }

    #[test]
    fn test_parse() {
        let raw_ytml =
            "html(lang = \"pt-br\"){ } body.container1#unique2(color = \"blue\"){p(color=\"red\"){content}}";
        let ast = parse_ytml_file(raw_ytml).unwrap();
        let root = ast.iter().nth(0).unwrap();
        let lang = root.attributes.get("lang").unwrap();
        assert_eq!(lang, "pt-br");

        let body = ast.iter().nth(1).unwrap();
        // Ensure that both class and id properties was parsed sucessfully
        let body_class = body.attributes.get("class").unwrap();
        let body_id = body.attributes.get("id").unwrap();

        assert_eq!(body_class, &String::from("container1"));
        assert_eq!(body_id, &String::from("unique2"));
    }
}
