use std::fs::{File, OpenOptions};
use std::io::{stdin, Write};
use std::path::Path;

fn create_file(file_name: &str) -> std::io::Result<File> {
    return OpenOptions::new().write(true)
        .create_new(true)
        .open(file_name);
}

pub fn open_file() -> File {
    let file_name = "hbr.txt";
    let is_file_exists = Path::new(file_name).exists();
    let file;

    // Если файл существует - готовимся писать в него
    if is_file_exists {
        file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_name)
            .unwrap();
    } else {
        // Если файл не существует - создаём его и готовимся писать в него
        file = create_file(file_name).expect("TODO: panic message");
    }

    return file;
}

pub fn get_user_data(field_name: &str) -> String {
    println!("Enter {}:", field_name);
    let mut data = String::new();
    stdin().read_line(&mut data)
        .ok()
        .expect("Failed to read line");

    return data;
}

pub fn write_data_to_file(mut file: File, link: String, title: String, opinion: String) {
    write!(&mut file, "Link: {}", link).expect("TODO: panic message");
    write!(&mut file, "Title: {}", title).expect("TODO: panic message");
    write!(&mut file, "Opinion: {}", opinion).expect("TODO: panic message");
    write!(&mut file, "\n").expect("TODO: panic message");
}