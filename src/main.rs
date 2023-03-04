mod ast;
mod html;
mod ytml;

use html::ast_to_html;
use ytml::ytml_doc_to_ast;

fn main() {
    let raw_ytml = "html(lang = \"pt-br\") { content test } body(){ p(color = \"blue\"){}} ";
    let result = ytml_doc_to_ast(raw_ytml);
    for tag in result {
        let html_parsed = ast_to_html(&tag, 0);
        println!("{html_parsed}");
    }
}
