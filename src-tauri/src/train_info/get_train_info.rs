use chrono::Duration;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

const WALKING_MINUTES: u16 = 11;
const RUNNING_MINUTES: u16 = 6;
const TIME_FORMAT: &str = "%Y/%m/%d %H:%M:%S %z";

#[derive(Serialize)]
struct ArrivalInfo {
	plan_departure_time: String,
	real_departure_time: String,
	train_type: String,
	terminal_station: String,
	is_delayed: bool,
	delay_time: String,
	travel_mode: String,
}

#[derive(Serialize)]
struct TrainInfo {
	update_time: String,
	yodoyabashi_direction: ArrivalInfo,
	sanjo_direction: ArrivalInfo,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ReceiveStationInfo {
	station_number: String,
	station_dep_time: String,
	station_name_jp: String,
	station_name_en: String,
	station_name_zh_tw: String,
	station_name_zh_cn: String,
	station_name_ko: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ReceiveTrainInfo {
	wdf_block_no: String,
	ext_train: String,
	premium_car: String,
	dia_station_info_objects: Vec<ReceiveStationInfo>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ReceiveStartTime {
	file_created_time: String,
	file_version: String,
	#[serde(rename = "TrainInfo")]
	train_info: Vec<ReceiveTrainInfo>,
}

fn find_all_train_after_move(
	train_info: &Vec<ReceiveTrainInfo>,
	minutes: i64,
) -> Vec<ReceiveTrainInfo> {
	train_info
		.iter()
		.filter(|train| {
			let arrival_neyagawa_station_info = train
				.dia_station_info_objects
				.iter()
				.filter(|station| station.station_name_jp == "寝屋川市")
				.collect::<Vec<&ReceiveStationInfo>>()[0];
			// 時間の整形
			let (train_arrival_hour_str, train_arrival_minutes_str) = arrival_neyagawa_station_info
				.station_dep_time
				.split(":")
				.collect_tuple()
				.unwrap();
			let mut train_arrival_hour: i32 = train_arrival_hour_str.parse().unwrap();
			train_arrival_hour = if train_arrival_hour >= 24 {
				train_arrival_hour - 24
			} else {
				train_arrival_hour
			};
			let train_arrival_minutes: i32 = train_arrival_minutes_str.parse().unwrap();

			let student_arrival_time = chrono::Local::now() + Duration::minutes(minutes as i64);
			// 整形後の時間
			let train_arrival_time = chrono::DateTime::parse_from_str(
				&format!(
					"{} {}:{}:0 +0900",
					&student_arrival_time
						.format(&TIME_FORMAT)
						.to_string()
						.split(" ")
						.collect::<Vec<&str>>()[0],
					if train_arrival_hour < 10 {
						format!("0{}", train_arrival_hour)
					} else {
						train_arrival_hour.to_string()
					},
					if train_arrival_minutes < 10 {
						format!("0{}", train_arrival_minutes)
					} else {
						train_arrival_minutes.to_string()
					},
				),
				&TIME_FORMAT,
			)
			.unwrap();
			return train_arrival_time >= student_arrival_time;
		})
		.map(|train| train.clone())
		.collect::<Vec<ReceiveTrainInfo>>()
}

// #[tauri::command]
pub async fn get_train_info() -> anyhow::Result<()> {
	const URL: &str = "https://www.keihan.co.jp/zaisen-up/startTimeList.json";
	let train_timetable_res = reqwest::get(URL).await?.text().await?;
	let train_timetable = serde_json::from_str::<ReceiveStartTime>(&train_timetable_res)?;

	let neyagawa_arrive_train_info_list = train_timetable
		.train_info
		.iter()
		// 止まらない駅を除く
		.map(move |train| ReceiveTrainInfo {
			wdf_block_no: train.wdf_block_no.clone(),
			ext_train: train.ext_train.clone(),
			premium_car: train.premium_car.clone(),
			dia_station_info_objects: train
				.dia_station_info_objects
				.iter()
				.filter(|&station| station.station_dep_time != "99:99")
				.map(|station| station.clone())
				.collect::<Vec<ReceiveStationInfo>>(),
		})
		// 寝屋川市に止まる電車のみを取得
		.filter(|train| {
			train
				.dia_station_info_objects
				.iter()
				.filter(|station| station.station_name_jp == "寝屋川市")
				.count() != 0
		})
		.collect::<Vec<ReceiveTrainInfo>>();

	let neyagawa_still_not_arrive_train_info_list =
		find_all_train_after_move(&neyagawa_arrive_train_info_list, RUNNING_MINUTES as i64);
	println!(
		"{:?}",
		neyagawa_still_not_arrive_train_info_list[0]
			.dia_station_info_objects
			.iter()
			.filter(|station| station.station_name_jp == "寝屋川市")
			.collect::<Vec<&ReceiveStationInfo>>()[0]
	);

	let neyagawa_still_not_arrive_train_info_list =
		find_all_train_after_move(&neyagawa_arrive_train_info_list, WALKING_MINUTES as i64);
	println!(
		"{:?}",
		neyagawa_still_not_arrive_train_info_list[0]
			.dia_station_info_objects
			.iter()
			.filter(|station| station.station_name_jp == "寝屋川市")
			.collect::<Vec<&ReceiveStationInfo>>()[0]
	);

	println!(
		"{}",
		chrono::Local::now() + Duration::minutes(RUNNING_MINUTES as i64)
	);

	Ok(())
}
