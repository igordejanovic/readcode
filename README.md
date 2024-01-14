# readcode

A simple tool for converting codebase to a pdf file.

![](https://raw.githubusercontent.com/igordejanovic/readcode/main/images/screenshot.png)

# Motivation

A desire to read code on a eink book reader.

# Installation

Make sure you have [Rust installed](https://www.rust-lang.org/tools/install).

Then you can either:
``` sh
git clone git@github.com:igordejanovic/readcode.git
cargo install readcode
```

or just

``` sh
cargo install --git https://github.com/igordejanovic/readcode.git
```

# Run

``` sh
readcode <path to project>
```

`readcode` will recursivelly walk the given directory and for all valid utf-8
files not ignored by .gitignore it will perform syntax highlight and add it to a
pdf together with the file path as the title of the section. At the end you will
get `output.pdf` file containing all the code from the project.

*Note: * The amount of code in a popular projects can be huge and this will make
`readcode` to run out of memory. This might be mitigated in the next versions by
auto-splitting across multiple pdf files but for now give it a subfolder with
the amount of code that can be reasonable for a single pdf.

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
# create pdf with TOC
pdftocio -o output-toc.pdf output.pdf < toc

```

# TODO

- Improve title detection. E.g. using `File:` prefix instead of font size.
- Make TOC which respects directory nesting.
- Do auto-splitting of pdf for larger repositories.
- Make options for excluding folders.
- Make option to filter by file extensions.
