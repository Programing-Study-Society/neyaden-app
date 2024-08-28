// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use neyaden_app::train_info;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![
			greet,
			train_info::get_train_info::get_train_info,
			train_info::stab_get_train_info::success_get_train_info,
			train_info::stab_get_train_info::failed_get_train_info,
		])
		.run(tauri::generate_context!())?;

	Ok(())
}
