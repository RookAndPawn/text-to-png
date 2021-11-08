# Text To Png Cli (txt2png)

This crate provides a command-line application for rendering simple text to
an image

## Usage

This is a classic [clap](https://crates.io/crates/clap) app, you can see all
the options with `-h`

```console
kguthrie@home text-to-png % ./txt2png -h
Text To Png Cli 0.1.0
Kevin G. <kevin.guthrie@gmail.com>
Renders text to a png with some options

USAGE:
    txt2png [OPTIONS] --output <file> [text]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --color <color>            Color of the text: e.g. Brown, #45A2f4, 666 [default: Orange Red]
    -o, --output <file>            Path of the file to write the rendered png
    -f, --font-file <font-file>    ttf or ttc font file to use
    -s, --font-size <font-size>    Font height in pixels [default: 64]


ARGS:
    <text>...    All trailing arguments will be treated as the text to render
```

To render text into a png file with the default font, run:

```console
kguthrie@home text-to-png % ./txt2png -o text.png -c DarkTurquoise -s 64 Rénder this, brö
```

And you'll get

![Rendered Text Image](https://github.com/RookAndPawn/text-to-png/blob/main/readme-resources/text1.png?raw=true)

To render text into a png file with the a custom font, run:

```console
kguthrie@home text-to-png % ./txt2png -o text.png -c 439EC2 -s 52 -f "fonts/Because I am Happy Regular.ttf" "Custom Fonts are Cool\!"
```

And you'll get

![Rendered Text Image](https://github.com/RookAndPawn/text-to-png/blob/main/readme-resources/text2.png?raw=true)
