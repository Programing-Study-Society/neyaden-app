// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use neyaden_app::train_info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![
			train_info::get_train_info::get_train_info,
			train_info::delay_info::get_stop_train_info,
			// train_info::stab_get_train_info::success_get_train_info,
			// train_info::stab_get_train_info::failed_get_train_info,
		])
		.run(tauri::generate_context!())?;

	Ok(())
}
