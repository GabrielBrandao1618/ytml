![ytml-logo](https://github.com/GabrielBrandao1618/ytml/assets/62272513/972fef9e-9cbb-45a9-9c54-0db00a76a18a)

# YTML

A brand new markup language

🚧 This is not meant to be used in production 🚧

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

`ytml parse <INPUT_FILE> [OUTPUT_FILE]`

Where

- INPUT_FILE is the path to the .ytml file you want to compile into html
- OUTPUT_FILE is the path to the .html file you want the ytml to be compiled into

OUTPUT_FILE is optional, the program will use the INPUT_FILE name as the output file name by default

The indentation is 2 by default, but you can pass a custom indentation with the --indent flag:

`ytml parse in.ytml out.html --indent 4`

You can watch for file changes and compile automatically with the watch mode:

`ytml watch in.ytml out.html`

You can also pass a directory path, so all .ytml files within the specified directory will be observed and compiled as well

🚧 File changes made by vim, neovim, and other text-based editors currently can't be observed 🚧
