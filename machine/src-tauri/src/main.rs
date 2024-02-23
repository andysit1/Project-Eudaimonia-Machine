// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn click() -> String {
    format!("Thanks for clicking!")
}

#[tauri::command]
fn escape() -> String {
    format!("Good job today!")
}

fn main() {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");

    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
    .add_native_item(MenuItem::Copy)
    .add_item(CustomMenuItem::new("hide", "Hide"))
    .add_submenu(submenu);


    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, click, escape])
        .setup(|app| {
            let docs_window = tauri::WindowBuilder::new(
                app,
                "external", /* the unique window label */
                tauri::WindowUrl::External("https://tauri.app/".parse().unwrap())
                )
                .title("Example Window")
                .fullscreen(true)
                .always_on_top(true)
                .resizable(false)
                .closable(false)
                .focus()
                .tabbing_identifier("Main")
                .build()?;
            let docs_window = tauri::WindowBuilder::new(
                app,
                "local", /* the unique window label */
                tauri::WindowUrl::External("https://youtube.com/".parse().unwrap())
              )
              .title("Example Window")
              .resizable(false)
              .closable(false)
              .focus()
              .fullscreen(true)
              .tabbing_identifier("Main")
              .build()?;
            Ok(())
        })
        .menu(menu)
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "quit" => {
                    std::process::exit(0);
                }
                "close" => {
                    event.window().close().unwrap();
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}