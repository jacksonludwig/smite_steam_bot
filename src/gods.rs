use isahc::prelude::*;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;

pub fn read_in_names() -> Vec<String> {
    let path = Path::new("resources/names.txt");
    let mut file = File::open(&path).unwrap();

    let mut god_names = String::new();
    match file.read_to_string(&mut god_names) {
        Err(e) => panic!("Couldnt read file: {}", e),
        Ok(_) => println!("Read file"),
    }

    god_names.split('\n').map(|s| s.to_string()).collect()
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
fn download_html(god: &str) -> Result<String> {
    let link = get_link(god);
    let mut page = isahc::get(link)?;
    Ok(page.text()?)
}

// Formats name
pub fn download_all_html(names: Vec<&str>) -> Result<()> {
    for name in names {
        let name = format_name(&name);
        let page = download_html(&name)?;

        let mut str_path = String::from("resources/");
        str_path.push_str(&name);

        let path = Path::new(&str_path);
        let mut file = File::create(path)?;

        match file.write_all(&page.as_bytes()) {
            Err(e) => panic!("Error writing: {}", e),
            Ok(_) => println!("Success writing to file for: {}", name),
        }
    }
    Ok(())
}

pub fn read_html_from_file(god: &str) -> Result<String> {
    let name = format_name(god);
    let mut str_path = String::from("resources/");
    str_path.push_str(&name);

    let path = Path::new(&str_path);
    let mut file = File::open(&path)?;

    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Err(e) => panic!("Couldnt read file: {}", e),
        Ok(_) => println!("Read file"),
    }

    Ok(data)
}

pub fn delete_all_html(names: Vec<&str>) -> Result<()> {
    for name in names {
        let name = format_name(&name);

        let mut str_path = String::from("resources/");
        str_path.push_str(&name);

        let path = Path::new(&str_path);

        fs::remove_file(path);
    }
    Ok(())
}
