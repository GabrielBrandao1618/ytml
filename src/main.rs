use std::collections::HashMap;
use std::fmt;

struct Tag<'a> {
    name: String,
    attributes: HashMap<&'a str, &'a str>,
    inner: Option<Box<Tag<'a>>>
}

impl fmt::Display for Tag<'static> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut attributes_rep = String::new();
        for (key, val) in self.attributes.iter() {
            let fmt_prop = format!("{key} = {val}, ");
            attributes_rep.push_str(&fmt_prop);
        }
        write!(f, "{}({})", self.name, attributes_rep)
    }
}

fn main() {
    let t1 = Tag {
        name: String::from("html"),
        attributes: HashMap::from([
            ("href", "http://google.com"),
            ("color", "blue")
        ]),
        inner: Some(Box::new(Tag{
            name: String::from("head"),
            attributes: HashMap::new(),
            inner: None
        }))
    };
    println!("{}", t1);
}
