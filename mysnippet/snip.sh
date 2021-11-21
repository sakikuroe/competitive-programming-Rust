#!/bin/bash

cp /dev/null ~/.vim/UltiSnips/rust.snippets
cargo snippet -t ultisnips > ~/.vim/UltiSnips/rust.snippets
exit 0
