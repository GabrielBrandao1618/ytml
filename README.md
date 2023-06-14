# YTML

A brand new markup language

## How it works

It works like Typescript, which is transpiled into Javascript. ytml files can be transpiled into html

## Syntax

```html
<html lang="pt-br">
  <body>
    <p color="blue" class="paragraph" id="first">Hello there</p>
  </body>
</html>
```

Is equivalent to

```
html(lang = "pt-br"){
  body {
    p.paragraph#first(color = "blue") {
      Hello there
    }
  }
}
```

Also, with the multiply operator(\*), you can avoid repetition, so

```html
<html>
  <body>
    <p>Hello!</p>
    <p>Hello!</p>
    <p>Hello!</p>
    <p>Hello!</p>
  </body>
</html>
```

Can be wrote this way

```
html {
  body {
    p*4 {
      Hello!
    }
  }
}
```

## Usage

Create a tag:

```rust
use ytml::tokens::Tag;
use ytml::html::ast_tag_to_html;

fn main() {
  let tag = Tag {
    name: String::from("html"),
    attributes: HashMap::new(),
    inner: Vec::new(),
  };
  let indent = 2; // The indentation
  let indent_start = 0; // The initial indentation
  let html_output = ast_tag_to_html(&tag, indent_start, indent);
  println!("{}", html_output);
  // <html></html>
}
```

Read ytml code into tag:

```rust
use ytml::file_handling::file_input::read_file_into_ast;

fn main() {
  let file_path = "./index.ytml";
  let tags = read_file_into_ast(file_path);
  for tag in tags {
      println!("{}", tag);
  }
}
```

Write html code:

```rust
use std::collections::HashMap;
use ytml::{file_handling::file_output::write_html_to_file, tokens::Tag};

fn main() {
  let document = vec![
    Tag{
      attributes: HashMap::new(),
      inner: Vec::new(),
      name: String::from("html"),
    }
  ];
  let file_path = "./out.html";
  write_html_to_file(file_path, document, 2);
}
```

Define tag with a inner content:

```rust
use std::collections::HashMap;

use ytml::tokens::{Tag, TagInnerElement};

fn main() {
    let p = Tag{
        attributes: HashMap::new(),
        name: String::from("p"),
        inner: vec![TagInnerElement::Text(String::from("This is a paragraph"))]
    };
    println!("{}", p);

    let div = Tag{
        attributes: HashMap::new(),
        name: String::from("div"),
        inner: vec![TagInnerElement::Tag(p)]
    };
    println!("{}", div);
}
```

Using file paths only:

```rust
use ytml::file_handling::compile_ytml_file;

fn main() {
    let ytml_file_path = String::from("./index.ytml");
    let html_file_path = String::from("./out.html");
    let indent = 2;
    compile_ytml_file(ytml_file_path, Some(html_file_path), indent);
}
```
