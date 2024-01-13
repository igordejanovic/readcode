# readcode

A simple tool for converting codebase to a pdf file.

# Motivation

A desire to read code on a eink book reader.

# Installation

``` sh
git clone git@github.com:igordejanovic/readcode.git
cargo install readcode

```

# Run

``` sh
readcode <path to project>
```

# Current features

- Recurse over given folder and produces a highlighted text for each file.
- Ignore all files ignored by .gitignore

# Creating TOC

See [this](https://krasjet.com/voice/pdf.tocgen/) tool.

Basically what you need to do after creating `output.pdf` with this tools is:

``` sh
cat > recipe.toml << EOF
[[heading]]
level = 1
greedy = true
font.size = 14
EOF

pdftocgen output.pdf < recipe.toml > toc
# Workaround for empty lines
sed -i '/"File:"/d' toc
# Remove File prefixes
sed -i 's/"File: /"/' toc
# check toc file to see if the outline is correct
pdftocio -o output-toc.pdf output.pdf < toc

```

# TODO

- Improve title detection. E.g. using `File:` prefix instead of font size.
- Make TOC which respects directory nesting.
