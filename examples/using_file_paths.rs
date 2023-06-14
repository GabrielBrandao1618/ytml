extern crate ytml;

use ytml::file_handling::parse_ytml_file;

fn main() {
    let ytml_file_path = String::from("./index.ytml");
    let html_file_path = String::from("./out.html");
    let indent = 2;
    parse_ytml_file(ytml_file_path, Some(html_file_path), indent);
}

