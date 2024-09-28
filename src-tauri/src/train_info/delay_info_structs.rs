use serde::Deserialize;

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
}

#[derive(Debug, Deserialize)]
pub struct Lin {
	pub nm: Vec<NM>,
}

#[derive(Debug, Deserialize)]
pub struct NM {
	#[serde(rename = "$value")]
	pub value: String,
}
