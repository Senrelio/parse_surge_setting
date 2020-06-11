use update_subs::handle;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (subs_filename, new_conf_fielname) = match args.len() {
        1 => ("subscribe.conf", "parsed.conf"),
        2 => (args.get(1).unwrap().as_str(), "parsed.conf"),
        3 => (args.get(1).unwrap().as_str(), args.get(2).unwrap().as_str()),
        _ => panic!("illegal arguments"),
    };
    handle(subs_filename, new_conf_fielname);
}
