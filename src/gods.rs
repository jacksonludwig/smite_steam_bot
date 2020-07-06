use isahc::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_in_names<'a>() -> Vec<&'a str> {
    let god_names = include_str!("names.txt");
    god_names.split("\n").collect()
}

fn format_name(name: &str) -> String {
    name.trim()
        .to_lowercase()
        .chars()
        .map(|c| match c {
            ' ' => '-',
            '\'' => '-',
            _ => c,
        })
        .collect::<String>()
}

// Requires formatted name
fn get_link(god: &str) -> String {
    let mut link = String::from("https://smite.guru/builds/");
    link.push_str(&god);
    link
}

// Requires formatted name
fn download_html(god: &str) -> Result<String, isahc::Error> {
    let link = get_link(god);
    let mut page = isahc::get(link)?;
    Ok(page.text().unwrap())
}

// Formats name
pub fn download_all_html(names: Vec<&str>) {
    for name in names {
        let name = format_name(&name);
        let page = download_html(&name).unwrap();

        let mut str_path = String::from("src/site_data/");
        str_path.push_str(&name);

        let path = Path::new(&str_path);
        let mut file = File::create(path).unwrap();

        match file.write_all(&page.as_bytes()) {
            Err(e) => panic!("Error writing: {}", e),
            Ok(_) => println!("Success writing to file for: {}", name),
        }
    }
}

pub fn read_html_from_file(god: &str) -> String {
    let name = format_name(god);
    let mut str_path = String::from("src/site_data/");
    str_path.push_str(&name);

    let path = Path::new(&str_path);
    let mut file = File::open(&path).unwrap();

    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Err(e) => panic!("Couldnt read file: {}", e),
        Ok(_) => println!("Read file"),
    }

    data
}
