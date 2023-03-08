# YTML

A brand new markup language

## How it works

It works like Typescript, which is compiled into Javascript. ytml files can be compiled into html

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

Here is how you can compile a ytml file into html

`ytml parse <INPUT_FILE> <OUTPUT_FILE>`

Where

- INPUT_FILE is the path to the .ytml file you want to compile into html
- OUTPUT_FILE is the path to the .html file you want the ytml to be compiled into

The indentation is 2 by default, but you can pass a custom indentation with the --indent flag:

`ytml parse in.ytml out.html --indent 4`

You can watch for file changes and compile automatically with the watch mode:

`ytml watch in.ytml out.html`

🚧 File edits changes with by vim, neovim, and other text-based editors currently can't be observed 🚧
