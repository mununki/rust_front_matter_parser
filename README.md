# Markdown front matter parser

This is a front matter in markdown file(`*.md`, `*.mdx`) parser built with Rust.

## What is front matter?

The front matter stays on top of markdown content and contains the metadata of itself.

```json
---
title: First Post
createdAt: 2018-08-24
category: ['devlogs']
keyword: ['first', 'post']
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
$ rust_front_matter_parser -t js -f postList -s test
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

- [ ] Write the output to file
- [ ] Error Handling (!important)
