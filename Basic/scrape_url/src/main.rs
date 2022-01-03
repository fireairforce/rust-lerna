use std::fs;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let (url, output) = (&args[1], &args[2]);
    println!("The all args are is {:?}", args);

    println!("Fetching url: {}", url);

    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);
    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", output);

}