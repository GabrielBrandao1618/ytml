mod ast;
mod file_output;
mod html;
mod ytml;
mod file_input;

use file_output::write_html_to_file;
use file_input::read_file_into_ast;

fn main() {
    let file_ast = read_file_into_ast(".out/in.ytml");
    write_html_to_file(".out/out.html", file_ast);
}
