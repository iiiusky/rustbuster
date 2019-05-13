extern crate pretty_env_logger;
extern crate hyper;
extern crate clap;

#[macro_use]
extern crate log;

use std::env;
use std::fs;
use std::str;
use clap::{Arg, App};

mod fetcher;

fn _old_main() {
    pretty_env_logger::init();

    // Some simple CLI args requirements...
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            error!("Usage: client <url>");
            return;
        }
    };

    // HTTPS requires picking a TLS implementation, so give a better
    // warning if the user tries to request an 'https' URL.
    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme_part().map(|s| s.as_ref()) != Some("http") {
        println!("This example only works with 'http' URLs.");
        return;
    }

    fetcher::_run(url);
}

fn main() {
    pretty_env_logger::init();
    let matches = App::new("rustbuster")
        .version("0.1")
        .author("phra <greensoncio@gmail.com>, ps1dr3x <michele@federici.tech>")
        .about("DirBuster for rust")
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .arg(Arg::with_name("url")
            .help("Sets the target URL")
            .short("u")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("wordlist")
            .help("Sets the wordlist")
            .short("w")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("extensions")
            .help("Sets the extensions")
            .short("e")
            .default_value("")
            .use_delimiter(true))
        .arg(Arg::with_name("mode")
            .help("Sets the mode of operation (dir, dns, fuzz)")
            .short("m")
            .takes_value(true)
            .default_value("dir"))
        .get_matches();
    
    let url = matches.value_of("url").unwrap();
    let wordlist_path = matches.value_of("wordlist").unwrap();
    let mode = matches.value_of("mode").unwrap();
    let extensions = matches.values_of("extensions").unwrap().filter(|e| e.len() != 0).collect::<Vec<&str>>();

    debug!("Using url: {:?}", url);
    debug!("Using wordlist: {:?}", wordlist_path);
    debug!("Using mode: {:?}", mode);
    debug!("Using extensions: {:?}", extensions);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("verbose") {
        0 => info!("No verbose info"),
        1 => info!("Some verbose info"),
        2 => info!("Tons of verbose info"),
        3 | _ => info!("Don't be crazy"),
    }

    match mode {
        "dir" => {
            debug!("using mode: dir");
            let urls = load_wordlist_and_build_urls(wordlist_path, url, extensions);
            debug!("urls: {:#?}", urls);
            schedule_work();
            run();
        },
        _ => (),
    }
}

fn load_wordlist_and_build_urls(wordlist_path: &str, url: &str, extensions: Vec<&str>) -> Vec<String> {
    debug!("loading wordlist");
    let contents = fs::read_to_string(wordlist_path)
        .expect("Something went wrong reading the file");
    
    let splitted_lines = contents.lines();
    build_urls(splitted_lines, url, extensions)
}

fn build_urls(splitted_lines: str::Lines, url: &str, extensions: Vec<&str>) -> Vec<String> {
    debug!("building urls");
    let mut urls: Vec<String> = Vec::new();
    let urls_iter = splitted_lines
        .filter(|word| !word.starts_with('#') && !word.starts_with(' '))
        .map(|word| format!("{}{}", url, word))
        .map(|url| url.to_owned());
    
    for url in urls_iter {
        urls.push(url.to_owned());
        for extension in extensions.iter() {
            urls.push(format!("{}.{}", url, extension).to_owned())
        }
    }

    urls
}

fn schedule_work() {
    debug!("scheduling work");
}

fn run() {
    debug!("run!");
}