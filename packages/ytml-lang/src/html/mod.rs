use crate::tokens::{Tag, TagInnerElement};

pub fn ast_to_html(mut ast: Vec<Tag>, indent: usize) -> String {
    let mut html_content = String::new();

    for root_tag in ast.iter_mut() {
        let html_tag = ast_tag_to_html(root_tag, 0, indent);
        html_content.push_str("<!DOCTYPE html>\n");
        html_content.push_str(&format!("{}\n", html_tag));
    }

    html_content
}

pub fn ast_tag_to_html(ast: &mut Tag, indent_level: usize, indent: usize) -> String {
    let mut tag_content = String::new();

    let multiplier: u32 = ast
        .attributes
        .remove("multiplier")
        .unwrap_or("1".to_owned())
        .parse()
        .unwrap();
    let attributes_rep: String = ast
        .attributes
        .iter()
        .map(|(key, val)| format!("{}=\"{}\"", key, val))
        .collect::<Vec<String>>()
        .join(" ");

    for _ in 0..multiplier {
        tag_content.push_str(&format!(
            "{indent}<{tagname} {attributes_rep}>",
            tagname = ast.name,
            indent = String::from(" ".repeat(indent_level))
        ));
        for child in ast.inner.iter_mut() {
            match child {
                TagInnerElement::Tag(tag) => {
                    tag_content.push_str(&format!(
                        "\n{html}",
                        html = &ast_tag_to_html(tag, indent_level + indent, indent),
                    ));
                }
                TagInnerElement::Text(content) => tag_content.push_str(&format!(
                    "\n{indent}{content}\n",
                    content = content,
                    indent = String::from(" ".repeat(indent_level + indent))
                )),
            }
        }
        tag_content.push_str(&format!(
            "{indent}</{tagname}>\n",
            tagname = ast.name,
            indent = String::from(" ".repeat(indent_level))
        ));
    }
    tag_content
}
