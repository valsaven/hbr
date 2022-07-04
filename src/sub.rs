use std::fs::{File, OpenOptions};
use std::io::{stdin, Write};
use std::path::{Path, PathBuf};

use dirs::home_dir;
use webpage::{Webpage, WebpageOptions};

fn create_file(file_path: &PathBuf) -> std::io::Result<File> {
    return OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file_path);
}

pub fn open_file() -> File {
    let file;
    let file_name = "hbr.txt";

    let home_dir = &home_dir().expect("Error: Impossible to get your home dir.");
    let home = Path::new(home_dir);

    let home_dir = home.join(file_name);

    if !home_dir.exists() {
        // If the file doesn't exist, create it and get ready to write to it
        file = create_file(&home_dir).expect("Error: Impossible to create hbr.txt file.");
    } else {
        // If the file exists, get ready to write into it
        file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(home_dir)
            .unwrap();
    }

    return file;
}

pub fn get_user_data(field_name: &str) -> String {
    println!("Enter {}:", field_name);
    let mut data = String::new();
    stdin()
        .read_line(&mut data)
        .ok()
        .expect("Failed to read line");

    return data;
}

pub fn write_data_to_file(mut file: File, link: String, title: String, opinion: String) {
    let error_text = "Unable to write to file.";

    write!(&mut file, "Link: {}", link).expect(error_text);
    write!(&mut file, "Title: {}", title).expect(error_text);
    write!(&mut file, "Opinion: {}", opinion).expect(error_text);
    write!(&mut file, "\n").expect(error_text);
}

pub fn get_url_title(url: String) -> String {
    let info =
        Webpage::from_url(url.trim(), WebpageOptions::default()).expect("Could not read from URL");

    return info.html.title.unwrap().replace(" / Хабр", "\n");
}
