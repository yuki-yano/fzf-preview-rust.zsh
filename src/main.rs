#[macro_use]
extern crate serde_derive;

use anyhow::Result;
use atty::Stream;
use regex::Regex;
use std::io::{self, Read};
use structopt::StructOpt;

mod settings;
use crate::settings::Settings;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(name = "MODE")]
    mode: Option<String>,
    #[structopt(name = "RESOURCE")]
    resource: Option<String>,
}

fn read_from_stdin() -> Result<String> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buf)?;

    Ok(buf)
}

fn snippet_list() {
    let snippets = Settings::new().unwrap().snippets;
    let mut key_width = 0;

    for snippet in &snippets {
        if snippet.keyword.len() > key_width {
            key_width = snippet.keyword.len() + 1;
        }
    }

    for snippet in snippets {
        let keyword_and_snippet = format!(
            "{:width$}  {}",
            format!("{}:", snippet.keyword),
            snippet.snippet,
            width = key_width
        );
        println!("{}", keyword_and_snippet);
    }
}

fn auto_snippet(lbuffer: &str, rbuffer: &str) {
    let rbuffer_re = Regex::new(r"(^$|^\s)").unwrap();
    let snippets = Settings::new().unwrap().snippets;
    let placeholder_re = Regex::new(r"\{\{.*\}\}").unwrap();

    for snippet in snippets {
        let keyword = Regex::new(format!(r"^{}$", &snippet.keyword).as_str()).unwrap();

        if keyword.is_match(lbuffer) && rbuffer_re.is_match(rbuffer) {
            let snippet = snippet.snippet;

            let placeholder_mat = placeholder_re.find(&snippet);
            let (snippet, cursor) = match placeholder_mat {
                Some(mat) => (
                    placeholder_re.replace(&snippet, "").to_string(),
                    mat.start(),
                ),
                None => {
                    let cursor = snippet.len() + 1;
                    (snippet, cursor)
                }
            };

            println!("success");
            println!("{} {}", snippet, rbuffer);
            println!("{}", cursor);
            return;
        }
    }

    println!("failure")
}

fn main() {
    pretty_env_logger::init();

    let opt: Opt = Opt::from_args();

    match opt.mode {
        Some(mode) => {
            let mode = mode.as_str();
            match mode {
                "snippet-list" => snippet_list(),
                "auto-snippet" => {
                    if atty::is(Stream::Stdin) {
                        std::process::exit(1);
                    }
                    let input = read_from_stdin().unwrap();

                    let mut splitter = input.splitn(2, "\n");
                    let lbuffer = splitter.next().unwrap().trim_end();
                    let rbuffer = splitter.next().unwrap().trim_end();
                    auto_snippet(lbuffer, rbuffer)
                }
                _ => {
                    println!("unexpected mode");
                    std::process::exit(1);
                }
            }
        }
        None => {
            println!("Mode is required");
            std::process::exit(1);
        }
    };
}
