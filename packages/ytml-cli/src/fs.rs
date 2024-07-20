use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use ytml_lang::html::ast_to_html;
use ytml_lang::tokens::Tag;
use ytml_lang::ytml::error::YtmlResult;
use ytml_lang::ytml::parse_ytml_file;

pub fn read_file_into_ast(file_path: &str) -> YtmlResult<Vec<Tag>> {
    let path = Path::new(file_path);
    let mut file = File::open(path).expect("Could not open the file");

    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Could not read the file");
    let parsed_ast = parse_ytml_file(&file_content).unwrap();
    Ok(parsed_ast)
}

pub fn write_html_to_file(file_path: &str, ast: Vec<Tag>, indent: usize) {
    let path = Path::new(file_path);
    let mut output_file = File::create(path).ok().expect("Could not create the file");
    let html_content = ast_to_html(ast, indent);
    output_file
        .write_all(html_content.as_bytes())
        .expect("Could not write to file");
}

pub fn ytml_file_to_html(
    input_path: String,
    output_path: Option<String>,
    indent: usize,
) -> YtmlResult<(String, String)> {
    let actual_output_path = output_path.unwrap_or(
        Path::new(&input_path)
            .with_extension("html")
            .to_str()
            .unwrap()
            .to_owned(),
    );
    let ast = read_file_into_ast(&input_path)?;
    write_html_to_file(&actual_output_path, ast, indent);
    Ok((input_path, actual_output_path))
}
