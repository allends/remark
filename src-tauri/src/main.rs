#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use pulldown_cmark::{Parser, Options, html};


#[tauri::command]
fn parse_md(input: String) -> String {
  let mut options = Options::empty();
  options.insert(Options::ENABLE_TABLES);
  let parser = Parser::new_ext(&input, options);

  let mut html_output = String::new();
  html::push_html(&mut html_output, parser);

  html_output
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![parse_md])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
