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
