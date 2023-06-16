extern crate ytml;

use ytml::fs::file_input::read_file_into_ast;

fn main() {
    let file_path = "./index.ytml";
    let tags = read_file_into_ast(file_path);
    for tag in tags {
        println!("{}", tag);
    }
}
