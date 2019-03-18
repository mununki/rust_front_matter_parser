# Markdown front-matter parser

This is a front matter in markdown file(`*.md`, `*.mdx`) parser built with Rust.

## What is front-matter?

The front matter is the metadata of markdown content. It stays on top of markdown content and contains the metadata of itself.

```json
---
title: First Post
createdAt: 2018-08-24
category: ["devlogs"]
keyword: ["first", "post"]
---

// markdown body content...
```

## Features

- Parsing the front matter in markdown file
- Blazing fast
- Creating `*.json` or `*.js` file from all markdown files in target directory
- CLI tool

## Build

> Cargo is required.

```shell
$ git clone https://github.com/mattdamon108/rust_front_matter_parser.git
$ cd rust_front_matter_parser
$ cargo build --release
$ cargo install --path .
```

## How to use

```shell
$ mdfmp -h
Markdown front-matter parser 0.1.0
moondaddi <woonki.moon@gmail.com>
Parse the front-matter data from '*.md' or '*.mdx' files

USAGE:
		mdfmp [OPTIONS] --filename <FILE> --type <TYPE>

FLAGS:
		-h, --help       Prints help information
		-V, --version    Prints version information

OPTIONS:
		-f, --filename <FILE>    Set the filename of output
		-s, --src <directory>    Set the source directory [default: .]
		-t, --type <TYPE>        Set the type of output [possible values: js, json]
```

```shell
$ mdfmp -t js -f postList -s test
```

### Options

- -t : (required) enum OutputType { `json`, `js` }
- -f : (required) filename of output
- -s : (default: `.`) target directory

### Output

JSON

```json
{
  "postList": [
    {
      "title": "Kill the process using port",
      "createdAt": "2019-03-05",
      "category": ["devlogs"],
      "keyword": ["macos", "kill", "port"]
    },
    {
      "title": "First Post",
      "createdAt": "2018-08-24",
      "category": ["devlogs"],
      "keyword": ["first", "post"]
    }
  ]
}
```

JS

```js
const postList = [
  {
    title: 'Kill the process using port',
    createdAt: '2019-03-05',
    category: ['devlogs'],
    keyword: ['macos', 'kill', 'port'],
  },
  {
    title: 'First Post',
    createdAt: '2018-08-24',
    category: ['devlogs'],
    keyword: ['first', 'post'],
  },
];

export default postList;
```

### Next to do

- [x] Write the output to file
- [x] Parse only md, mdx files
- [x] CLI message
- [x] Optional CLI args with Clap
- [x] Error Handling (!important)
