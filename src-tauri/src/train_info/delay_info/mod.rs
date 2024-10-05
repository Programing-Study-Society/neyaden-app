use serde::Serialize;

mod get_stop_keihan_train_info;
mod get_stop_westjr_train_info;

use get_stop_keihan_train_info::get_stop_keihan_train_info;
use get_stop_westjr_train_info::get_stop_westjr_train_info;

#[derive(Debug, Serialize)]
pub struct StopTrainInfo {
	pub keihan: bool,
	pub osakaloop: bool,
	pub gakkentoshi: bool,
}

#[tauri::command]
pub async fn get_stop_train_info() -> Result<StopTrainInfo, String> {
	let is_delay_keihan = get_stop_keihan_train_info().await?;
	let delay_westjr = get_stop_westjr_train_info().await?;
	Ok(StopTrainInfo {
		keihan: is_delay_keihan,
		osakaloop: delay_westjr.osakaloop,
		gakkentoshi: delay_westjr.gakkentoshi,
	})
}