use serde::Deserialize;

/* 遅延情報等の情報ファイルリスト */
#[derive(Deserialize, Debug)]
pub struct ReceiveInfoFiles {
	#[serde(rename = "traininfo")]
	pub train_info: String,
}

#[derive(Debug, Deserialize)]
pub struct ReceiveDelayInfo {
	pub info: Info,
}

#[derive(Debug, Deserialize)]
pub struct Info {
	pub dif: DIF,
}

#[derive(Debug, Deserialize)]
pub struct DIF {
	pub all: u32,
	pub eif: Option<Vec<EIF>>,
}

#[derive(Debug, Deserialize)]
pub struct EIF {
	pub lin: Vec<Lin>,
	pub sts: Vec<Sts>,
}

#[derive(Debug, Deserialize)]
pub struct Lin {
	pub nm: Vec<NM>,
}

#[derive(Debug, Deserialize)]
pub struct Sts {
	pub nm: Vec<NM>,
}

#[derive(Debug, Deserialize)]
pub struct NM {
	#[serde(rename = "$value")]
	pub value: String,
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
