# Markdown to PDF Converter

A simple command-line tool written in Rust that converts Markdown (.md) files into clean PDF documents using using [typst](https://typst.app) and the [cmarker](https://typst.app/universe/package/cmarker) package.

## Features
- Converts Markdown files (.md) into PDFs (.pdf)
- Uses typst for lightweight conversion
- Automatically determines the output filename if not specified

## Requirements
Before running, make sure you have [Typst CLI](https://github.com/typst/typst) installed and available in your `$PATH`.
You can check if Typst is installed with:
```bash
typst --version
```

## Usage
Very basic usage:
```sh
md-typ-pdf input.md
```
This generates `input.pdf`.

You can also specify the output file name:
```sh
md-typ-pdf input.md output.pdf
```

## Why?
I was annoyed by the looks of pandocs markdown to pdf conversion (especially the insanely large default margins) and the need to install Gigabytes of latex...
Also I was annoyed to open another program like VSCode (for this great [extension](https://marketplace.visualstudio.com/items?itemName=yzane.markdown-pdf)) or Obsidian (that generate html out of them first and use chromium webrendering for the PDF), just to generate pleasing PDF files for submissions of assignements for my lectures.
So I created this: A very lightweight tool (including dependencies!) to do the simple task of converting my markdown files to PDF files that look fine for submission of exercises.
