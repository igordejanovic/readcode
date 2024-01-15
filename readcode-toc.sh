#!/bin/sh
# Create TOC for the given pdf produced by readcode
# Requires https://krasjet.com/voice/pdf.tocgen
if [ "$#" -ne 1 ]; then
    echo "Creates pdf with TOC from the pdf produced by readcode"
    echo "Usage: $0 <input pdf>"
else
pdftocgen $1 <<EOF | sed -e '/"File:"/d' | sed -e 's/"File: /"/' | pdftocio -o ${1%.pdf}-toc.pdf $1
[[heading]]
level = 1
greedy = true
font.size = 14
EOF
fi
