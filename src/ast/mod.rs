use std::collections::HashMap;
use std::fmt;

pub struct Tag{
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub inner: Vec<Box<Tag>>,
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut attributes_rep = String::new();
        for (key, val) in self.attributes.iter() {
            let fmt_prop = format!("{key} = {val}, ");
            attributes_rep.push_str(&fmt_prop);
        }
        let mut inner_rep = String::new();
        for inner in &self.inner {
            let unwraped_inner = format!("{}", inner);
            inner_rep.push_str(&unwraped_inner);
        }
        write!(
            f,
            "{name}({attributes}) -> {inner_rep}",
            name = self.name, attributes = attributes_rep, inner_rep = inner_rep
        )
    }
}
