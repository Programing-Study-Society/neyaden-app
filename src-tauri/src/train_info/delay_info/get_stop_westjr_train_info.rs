use serde::Deserialize;

#[derive(Deserialize)]
struct ReceiveDelayInfo {
	lines: Lines,
}

#[derive(Deserialize)]
struct Lines {
	#[serde(rename="osakaloop")]
	osakaloop: Option<serde_json::Value>,
	#[serde(rename="gakkentoshi")]
	gakkenntoshi: Option<serde_json::Value>,
}

pub struct DelayInfo {
	pub osakaloop: bool,
	pub gakkentoshi: bool,
}

pub async fn get_stop_westjr_train_info() -> Result<DelayInfo, String> {
	const URL: &str = "https://www.train-guide.westjr.co.jp/api/v3/area_kinki_trafficinfo.json";

	let delay_res = reqwest::get(URL)
		.await
		.map_err(|_| "情報の取得に失敗しました")?
		.text()
		.await
		.map_err(|_| "情報の取得に失敗しました")?;

	let delay_info = serde_json::from_str::<ReceiveDelayInfo>(&delay_res).map_err(|_| "形式が間違っています")?;

	Ok(DelayInfo {
		osakaloop: delay_info.lines.osakaloop.is_some(),
		gakkentoshi: delay_info.lines.gakkenntoshi.is_some(),
	})
}