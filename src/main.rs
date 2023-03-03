use std::collections::HashMap;
use std::fmt;

struct Tag<'a> {
    name: String,
    attributes: HashMap<&'a str, &'a str>,
    inner: Vec<Box<Tag<'a>>>,
}

impl fmt::Display for Tag<'static> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut attributes_rep = String::new();
        for (key, val) in self.attributes.iter() {
            let fmt_prop = format!("{key} = {val}, ");
            attributes_rep.push_str(&fmt_prop);
        }
        let mut inner_rep = String::new();
        for inner in &self.inner {
            let unwraped_inner = format!("\n {}", inner);
            inner_rep.push_str(&unwraped_inner);
        }
        write!(
            f,
            "{}({}) {{\n {}\n}}",
            self.name, attributes_rep, inner_rep
        )
    }
}

fn main() {
    let t1 = Tag {
        name: String::from("html"),
        attributes: HashMap::from([("href", "http://google.com"), ("color", "blue")]),
        inner: vec![
            Box::new(Tag {
                name: String::from("head"),
                attributes: HashMap::new(),
                inner: vec![],
            }),
            Box::new(Tag {
                name: String::from("title"),
                attributes: HashMap::new(),
                inner: Vec::new(),
            }),
        ],
    };
    println!("{}", t1);
}
