pub mod file_input;
pub mod file_output;

use std::path::Path;

use file_output::write_html_to_file;
use file_input::read_file_into_ast;

pub fn parse_ytml_file(input_path: String, output_path: Option<String>, indent: usize) -> (String, String) {
    let actual_output_path = output_path.unwrap_or(
        Path::new(&input_path)
            .with_extension("html")
            .to_str()
            .unwrap()
            .to_owned(),
    );
    let ast = read_file_into_ast(&input_path);
    write_html_to_file(&actual_output_path, ast, indent);
    (input_path, actual_output_path)
}
