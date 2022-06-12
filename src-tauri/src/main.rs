#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use native_dialog::{FileDialog };
use pulldown_cmark::{html, Options, Parser};
use std::{fs::File, io::{Read, Write}, path::{PathBuf, Path}, str::FromStr};

#[derive(serde::Serialize)]
struct FileMessage {
  contents: String,
  path: String,
}

#[tauri::command]
fn parse_md(input: String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

#[tauri::command]
fn load_file() -> FileMessage {
    let path = FileDialog::new()
        .set_location("~/Documents")
        .add_filter("Markdown File", &["md"])
        .show_open_single_file()
        .unwrap();

    let path = match path {
        Some(path) => path,
        None => return FileMessage {
            contents: "".to_string(),
            path: "".to_string()
        },
    };

    let mut f = File::open(&path).unwrap();
    let mut result = String::new();
    f.read_to_string(&mut result);

    FileMessage { contents: result, path: path.to_str().unwrap().to_string() }
}

#[tauri::command]
fn save_file(updated_contents: String, path: String) {
    let path = PathBuf::from_str(&path);
    let path = match path {
        Ok(path) => path,
        Err(_) => return,
    };

    let mut file = File::open(&path).unwrap();
    file.write(updated_contents.as_bytes());

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse_md, load_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
