use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

static HEADERS_PATH: &str = "./configs/headers.conf";
static RULES_PATH: &str = "./configs/rules.conf";

#[test]
fn test_get_subs() {
    let mut sub_file: File = get_readonly_file("test.conf").unwrap();
    for i in get_subs_from_conf(&mut sub_file) {
        println!("{}", i);
    }
}

pub fn handle(sub_filename: &str, target_filename: &str) {
    let mut sub_file: File = get_readonly_file(sub_filename).unwrap();
    let mut target_file = get_writable_file(target_filename).unwrap();
    translate(&mut sub_file, &mut target_file);
}

fn translate(sub_file: &mut File, target_file: &mut File) {
    target_file
        .write_all(get_headers().as_bytes())
        .expect("error writing headers");
    let subs: &Vec<String> = &get_subs_from_conf(sub_file);
    target_file
        .write(String::from("\n[Proxy]\n").as_bytes())
        .expect("error writing [Proxy]");
    target_file
        .write_all(&subs.join("\n").as_bytes())
        .expect("error writing subs");
    let proxy_titles: Vec<String> = subs
        .iter()
        .map(|s| String::from(s.split("=").nth(0).unwrap()))
        .collect();
    let mut proxy_group = String::from("\n[Proxy Group]\n");
    proxy_group.push_str("Proxy = select, ");
    proxy_group.push_str(
        proxy_titles
            .iter()
            .map(|s| s.trim())
            .collect::<Vec<&str>>()
            .join(", ")
            .as_str(),
    );
    proxy_group.push_str("\n");
    target_file
        .write_all(proxy_group.as_bytes())
        .expect("error writing proxy groups");

    target_file
        .write_all(get_rules().as_bytes())
        .expect("error writing rules");
}

fn get_headers() -> String {
    let path = Path::new(HEADERS_PATH);
    let mut headers_file = File::open(path).unwrap();
    let mut headers: String = String::new();
    headers_file
        .read_to_string(&mut headers)
        .expect("error reading headers");
    headers
}

fn get_subs_from_conf(conf: &mut File) -> Vec<String> {
    let mut lines_iter = io::BufReader::new(conf).lines();
    let mut subs: Vec<String> = Vec::new();
    lines_iter
        .position(|line| line.expect("error parsing line").starts_with("[Proxy]"))
        .expect("error finding [Proxy]");
    let lines: Vec<String> = lines_iter
        .map(|line| line.expect("error parsing line"))
        .collect();
    for line in lines.as_slice().iter() {
        if !line.starts_with("[") && !line.is_empty() {
            subs.push(line.to_owned());
        } else {
            break;
        }
    }
    subs
}

fn get_rules() -> String {
    let path = Path::new(RULES_PATH);
    let mut rules_file: File = File::open(path).expect("error reading rules config");
    let mut rules: String = String::new();
    rules_file
        .read_to_string(&mut rules)
        .expect("error parsing rules");
    rules
}

fn get_writable_file(filename: &str) -> std::io::Result<File> {
    let path = Path::new(filename);
    File::create(path)
}

fn get_readonly_file(filename: &str) -> std::io::Result<File> {
    let path = Path::new(filename);
    File::open(path)
}
