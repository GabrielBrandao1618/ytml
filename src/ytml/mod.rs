use pest::{iterators::Pairs, Parser};

#[derive(pest_derive::Parser)]
#[grammar = "ytml/grammar/ytml.pest"]
pub struct YtmlParser {}

pub fn ytml_to_ast(input: &str) -> Pairs<Rule> {
    let pairs = YtmlParser::parse(Rule::tag, input).unwrap();
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_out() {
        ytml_to_ast("html(lang = \"pt-br\"){{ }}");
    }
}
