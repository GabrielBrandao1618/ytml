use super::ast::{Tag, TagInnerElement};

pub fn ast_to_html(ast: &Tag, indent_level: usize) -> String {
    let mut tag_content = String::new();
    let mut attributes_rep = String::new();

    for (key, val) in ast.attributes.iter() {
        attributes_rep.push_str(&format!(
            "{attribute} = \"{val}\" ",
            attribute = key,
            val = val
        ));
    }
    tag_content.push_str(&format!(
        "{indent}<{tagname} {attributes_rep}>",
        tagname = ast.name,
        indent = String::from(" ".repeat(indent_level))
    ));
    for child in &ast.inner {
        match child {
            TagInnerElement::Tag { tag } => {
                tag_content.push_str(&format!(
                    "\n{html}",
                    html = &ast_to_html(tag, indent_level + 2),
                ));
            },
            TagInnerElement::Text { content } => {
                tag_content.push_str(&format!("{}", content))
            }
        }
    }
    tag_content.push_str(&format!(
        "\n{indent}</{tagname}>",
        tagname = ast.name,
        indent = String::from(" ".repeat(indent_level))
    ));
    tag_content
}
