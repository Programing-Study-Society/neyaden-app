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
