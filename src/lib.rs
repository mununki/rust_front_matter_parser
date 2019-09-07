extern crate clap;

use clap::ArgMatches;
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::process;

#[derive(Debug, PartialEq)]
pub enum OutputType {
    JSON,
    JS,
}

#[derive(Debug)]
pub struct Config {
    pub output_type: OutputType,
    pub src: String,
    pub filename: String,
}

impl Config {
    pub fn new(matches: ArgMatches) -> Result<Config, &'static str> {
        let output_type = match matches.value_of("type") {
            Some(t) => {
                if t == "js" {
                    OutputType::JS
                } else if t == "json" {
                    OutputType::JSON
                } else {
                    return Err("Output type is not properly set.");
                }
            }
            _ => return Err("Output type is not properly set."),
        };

        let filename = match matches.value_of("filename") {
            Some(f) => f.to_string(),
            _ => return Err("Filename is not properly set."),
        };

        let src = matches.value_of("src").unwrap();

        Ok(Config {
            output_type,
            src: src.to_string(),
            filename,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut result = String::new();

    if let Ok(entries) = fs::read_dir(Path::new(&config.src)) {
        for entry in entries {
            if let Ok(entry) = entry {
                // Here, 'entry' is a 'std::fs::DirEntry'
                if !entry.path().is_dir() {
                    // skip the loop if file extension is not one of 'md' or 'mdx'
                    if let Some(ext) = entry.path().extension().and_then(|s| s.to_str()) {
                        if !((ext == "md") | (ext == "mdx")) {
                            continue;
                        }
                    }

                    if let Ok(markdown_content) = fs::read_to_string(entry.path()) {
                        println!("*** {:?}", entry.file_name());

                        let front_matter_as_vec_str = parse_front_matter(&markdown_content);
                        if !front_matter_as_vec_str.is_empty() {
                            let total_lines: usize = front_matter_as_vec_str.len();
                            result.push_str("{\n");
                            for (counter_line, item) in front_matter_as_vec_str.iter().enumerate() {
                                if config.output_type == OutputType::JS {
                                    if (counter_line + 1) == total_lines {
                                        result.push_str(&format!(
                                            "\t{}\n",
                                            convert_front_matter_js(item)
                                        ));
                                    } else {
                                        result.push_str(&format!(
                                            "\t{},\n",
                                            convert_front_matter_js(item)
                                        ));
                                    }
                                } else if (counter_line + 1) == total_lines {
                                    result.push_str(&format!(
                                        "\t{}\n",
                                        convert_front_matter_json(item)
                                    ));
                                } else {
                                    result.push_str(&format!(
                                        "\t{},\n",
                                        convert_front_matter_json(item)
                                    ));
                                }
                            }
                            result.push_str("},\n");
                        }
                    };
                }
            }
        }
    };

    // remove trailing comma
    result.pop();
    result.pop();
    if config.output_type == OutputType::JS {
        let mut output_result = format!(
            "const {0} = [\n{1}\n]\n\nmodule.exports = {0};",
            config.filename, result
        );
        if let Err(e) = create_write(config, &mut output_result) {
            eprintln!("Application error: {}", e);

            process::exit(1);
        };
    } else {
        let mut output_result = format!(
            "{}\n\"{}\": [\n{}\n]\n{}",
            "{", config.filename, result, "}"
        );
        if let Err(e) = create_write(config, &mut output_result) {
            eprintln!("Application error: {}", e);

            process::exit(1);
        }
    }

    Ok(())
}

pub fn create_write(config: Config, content: &mut str) -> Result<(), Box<dyn Error>> {
    let file_extension = match config.output_type {
        OutputType::JS => "js",
        OutputType::JSON => "json",
    };

    let create_filename = config.src + "/" + &config.filename + "." + file_extension;
    let mut file = fs::File::create(create_filename)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn check_if_array(content: &str) -> bool {
    let mut chars = content.chars();
    chars.next() == Some('[')
}

pub fn check_if_first_blank(content: &str) -> bool {
    let mut chars = content.chars();
    chars.next() == Some(' ')
}

pub fn check_if_first_single_quote(content: &str) -> bool {
    let mut chars = content.chars();
    let first_char = chars.next();

    first_char == Some('\'')
}

pub fn check_if_first_double_quote(content: &str) -> bool {
    let mut chars = content.chars();
    let first_char = chars.next();

    first_char == Some('"')
}

pub fn add_double_quote(content: &str) -> String {
    let mut v = vec!['"'];
    for char in content.chars() {
        v.push(char);
    }
    v.push('"');
    let s: String = v.iter().collect();

    s
}

pub fn remove_quote(content: &str) -> &str {
    let mut chars = content.chars();
    let first_char = chars.next();

    if (first_char == Some('\'')) | (first_char == Some('"')) {
        &content[1..(content.len() - 1)]
    } else {
        content
    }
}

pub fn remove_first_blank(content: &str) -> &str {
    let mut chars = content.chars();
    let first_char = chars.next();

    if first_char == Some(' ') {
        chars.as_str()
    } else {
        content
    }
}

pub fn convert_front_matter_js(line: &str) -> String {
    let mut result = String::new();

    if line.contains(':') {
        let mut split_line = line.split(':');
        if let Some(property) = split_line.next() {
            if check_if_first_single_quote(property) {
                result.push_str(remove_quote(property));
            } else if check_if_first_double_quote(property) {
                result.push_str(remove_quote(property))
            } else {
                result.push_str(property);
            }
        };

        result.push_str(": ");

        if let Some(value) = split_line.next() {
            if check_if_first_blank(value) {
                if check_if_array(remove_first_blank(value)) {
                    result.push_str(remove_first_blank(value));
                } else if check_if_first_single_quote(remove_first_blank(value)) {
                    result.push_str(&add_double_quote(remove_quote(remove_first_blank(value))));
                } else if check_if_first_double_quote(remove_first_blank(value)) {
                    result.push_str(remove_first_blank(value));
                } else {
                    result.push_str(&add_double_quote(remove_first_blank(value)));
                }
            } else if check_if_array(value) {
                result.push_str(value);
            } else if check_if_first_single_quote(value) {
                result.push_str(&add_double_quote(remove_quote(value)));
            } else if check_if_first_double_quote(value) {
                result.push_str(value);
            } else {
                result.push_str(&add_double_quote(value));
            }
        }
    }

    result
}

pub fn convert_front_matter_json(line: &str) -> String {
    let mut result = String::new();

    if line.contains(':') {
        let mut split_line = line.split(':');

        if let Some(property) = split_line.next() {
            if check_if_first_single_quote(property) {
                result.push_str(&add_double_quote(remove_quote(property)));
            } else if check_if_first_double_quote(property) {
                result.push_str(property)
            } else {
                result.push_str(&add_double_quote(property));
            }
        };

        result.push_str(": ");

        if let Some(value) = split_line.next() {
            if check_if_first_blank(value) {
                if check_if_array(remove_first_blank(value)) {
                    result.push_str(remove_first_blank(value));
                } else if check_if_first_single_quote(remove_first_blank(value)) {
                    result.push_str(&add_double_quote(remove_first_blank(value)));
                } else if check_if_first_double_quote(remove_first_blank(value)) {
                    result.push_str(remove_first_blank(value));
                } else {
                    result.push_str(&add_double_quote(remove_first_blank(value)));
                }
            } else if check_if_array(value) {
                result.push_str(value);
            } else if check_if_first_single_quote(value) {
                result.push_str(&add_double_quote(remove_quote(value)));
            } else if check_if_first_double_quote(value) {
                result.push_str(value);
            } else {
                result.push_str(&add_double_quote(value));
            }
        }
    }

    result
}

pub fn parse_front_matter(contents: &str) -> Vec<&str> {
    let mut is_front_matter: bool = false;
    let mut counter_meet_delimiter: u8 = 0;
    let mut front_matter = Vec::new();

    for (line_number, line) in contents.lines().enumerate() {
        if (line_number == 0) & (line != "---") {
            // break the loop, if first line is not "---"
            break;
        } else if (line_number == 0) & (line == "---") {
            // if first line is "---", increase counter_meet_delimiter and set is_front_matter = true
            counter_meet_delimiter += 1;
            is_front_matter = true;
            continue;
        }

        if is_front_matter & (line == "---") {
            // if encounter the second delimiter "---", then break the loop and increase counter_meet_delimiter
            counter_meet_delimiter += 1;
            break;
        }

        if is_front_matter & ((line != "---") | (line != "")) {
            front_matter.push(line);
        }
    }

    if counter_meet_delimiter == 1 {
        // if there are not the closed delimiter
        front_matter = Vec::new();
    }

    front_matter
}
