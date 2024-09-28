use serde::Serialize;

use crate::train_info::stop_train_info_structs::{ReceiveDelayInfo, ReceiveInfoFiles};

#[derive(Debug, Serialize)]
pub struct StopTrainInfo {
	pub keihan: bool,
}

#[tauri::command]
pub async fn get_stop_train_info() -> Result<StopTrainInfo, String> {
	Ok(StopTrainInfo {
		keihan: get_stop_keihan_train_info().await?,
	})
}

pub async fn get_stop_keihan_train_info() -> Result<bool, String> {
	const INFO_FILES_URL: &str = "https://www.keihan.co.jp/tinfo/05-flist/FileList.xml";
	let info_files_res = reqwest::get(INFO_FILES_URL)
		.await
		.map_err(|_| "情報の取得に失敗しました")?
		.text()
		.await
		.map_err(|_| "情報の取得に失敗しました")?;

	let info_files = serde_xml_rs::from_str::<ReceiveInfoFiles>(&info_files_res).unwrap();
	let delay_xml_res = reqwest::get(format!(
		"{}{}",
		"https://www.keihan.co.jp/tinfo/01-traininfo/", &info_files.train_info
	))
	.await
	.map_err(|_| "情報の取得に失敗しました")?
	.text()
	.await
	.map_err(|_| "情報の取得に失敗しました")?;

	let delay_xml = serde_xml_rs::from_str::<ReceiveDelayInfo>(&delay_xml_res).unwrap();

	// 京阪本線が運行休止中は寝屋川市も止まっているため本線が休止しているか確認
	let mut is_stopped_keihan = delay_xml.info.dif.all != 0;
	if is_stopped_keihan {
		is_stopped_keihan = delay_xml.info.dif.eif.unwrap().iter().any(|eif| {
			eif
				.lin
				.iter()
				.any(|lin| lin.nm.iter().any(|nm| nm.value.contains("京阪本線")))
				|| eif
					.sts
					.iter()
					.any(|sts| sts.nm.iter().any(|nm| nm.value.contains("運転見合わせ")))
		})
	}

	Ok(is_stopped_keihan)
}
