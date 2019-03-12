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
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let _output_check = match args.next() {
            Some(arg) => {
                if arg == "-t".to_string() {
                    arg
                } else {
                    return Err("Type option should be '-t'");
                }
            }
            None => return Err("Type option is required."),
        };

        let output_type = match args.next() {
            Some(arg) => {
                if arg == "js" {
                    OutputType::JS
                } else if arg == "json" {
                    OutputType::JSON
                } else {
                    return Err("Type option should be either js or json");
                }
            }
            None => return Err("Didn't get a output option."),
        };

        let _filename_check = match args.next() {
            Some(arg) => {
                if arg == "-f".to_string() {
                    arg
                } else {
                    return Err("Filename option should be '-f'");
                }
            }
            None => return Err("Filename option is requred."),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename option."),
        };

        let _src_check = match args.next() {
            Some(arg) => {
                if arg == "-s".to_string() {
                    arg
                } else {
                    return Err("Source option should be '-s'");
                }
            }
            None => String::from("-s"),
        };

        let src = match args.next() {
            Some(arg) => arg,
            None => String::from("."),
        };

        Ok(Config {
            output_type,
            src,
            filename,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
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
                        if front_matter_as_vec_str.len() != 0 {
                            let total_lines: usize = front_matter_as_vec_str.len();
                            let mut counter_line: usize = 0;
                            result.push_str("{\n");
                            for item in front_matter_as_vec_str {
                                counter_line += 1;
                                if config.output_type == OutputType::JS {
                                    if counter_line == total_lines {
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
                                } else {
                                    if counter_line == total_lines {
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

pub fn create_write(config: Config, content: &mut str) -> Result<(), Box<Error>> {
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
    if chars.next() == Some('[') {
        true
    } else {
        false
    }
}

pub fn check_if_first_blank(content: &str) -> bool {
    let mut chars = content.chars();
    if chars.next() == Some(' ') {
        true
    } else {
        false
    }
}

pub fn check_if_first_single_quote(content: &str) -> bool {
    let mut chars = content.chars();
    let first_char = chars.next();

    if first_char == Some('\'') {
        true
    } else {
        false
    }
}

pub fn check_if_first_double_quote(content: &str) -> bool {
    let mut chars = content.chars();
    let first_char = chars.next();

    if first_char == Some('"') {
        true
    } else {
        false
    }
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

    if line.contains(":") {
        let mut split_line = line.split(":");
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
                } else {
                    if check_if_first_single_quote(remove_first_blank(value)) {
                        result.push_str(&add_double_quote(remove_quote(remove_first_blank(value))));
                    } else if check_if_first_double_quote(remove_first_blank(value)) {
                        result.push_str(remove_first_blank(value));
                    } else {
                        result.push_str(&add_double_quote(remove_first_blank(value)));
                    }
                }
            } else {
                if check_if_array(value) {
                    result.push_str(value);
                } else {
                    if check_if_first_single_quote(value) {
                        result.push_str(&add_double_quote(remove_quote(value)));
                    } else if check_if_first_double_quote(value) {
                        result.push_str(value);
                    } else {
                        result.push_str(&add_double_quote(value));
                    }
                }
            }
        }
    }

    result
}

pub fn convert_front_matter_json(line: &str) -> String {
    let mut result = String::new();

    if line.contains(":") {
        let mut split_line = line.split(":");
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
                } else {
                    if check_if_first_single_quote(remove_first_blank(value)) {
                        result.push_str(&add_double_quote(remove_first_blank(value)));
                    } else if check_if_first_double_quote(remove_first_blank(value)) {
                        result.push_str(remove_first_blank(value));
                    } else {
                        result.push_str(&add_double_quote(remove_first_blank(value)));
                    }
                }
            } else {
                if check_if_array(value) {
                    result.push_str(value);
                } else {
                    if check_if_first_single_quote(value) {
                        result.push_str(&add_double_quote(remove_quote(value)));
                    } else if check_if_first_double_quote(value) {
                        result.push_str(value);
                    } else {
                        result.push_str(&add_double_quote(value));
                    }
                }
            }
        }
    }

    result
}

pub fn parse_front_matter(contents: &str) -> Vec<&str> {
    let mut is_front_matter: bool = false;
    let mut counter_meet_delimiter: u8 = 0;
    let mut line_number: usize = 0;
    let mut front_matter = Vec::new();

    for line in contents.lines() {
        line_number += 1;

        if (line_number == 1) & (line != "---") {
            // break the loop, if first line is not "---"
            break;
        } else if (line_number == 1) & (line == "---") {
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
