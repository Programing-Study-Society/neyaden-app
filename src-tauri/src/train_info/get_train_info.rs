use chrono::Duration;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

const WALKING_MINUTES: u16 = 11;
const RUNNING_MINUTES: u16 = 6;
const TIME_FORMAT: &str = "%Y/%m/%d %H:%M:%S %z";

#[derive(Serialize, Debug, Clone)]
pub struct ArrivalInfo {
	pub plan_departure_time: String,
	pub real_departure_time: String,
	pub train_type: String,
	pub terminal_station: String,
	pub train_direction: String,
	pub is_delayed: bool,
	pub delay_time: String,
	pub travel_mode: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct TrainInfo {
	pub update_time: String,
	pub yodoyabashi_direction: Vec<ArrivalInfo>,
	pub sanjo_direction: Vec<ArrivalInfo>,
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ReceiveTrainPositionObject {
	// carsOfTrain
	delay_minutes: String,
	// delayMinutesEn
	// delayMinutesKo
	// delayMinutesZhCn
	// delayMinutesZhTw
	// destStationCode
	// destStationNameEn
	dest_station_name_jp: String,
	// destStationNameKo
	// destStationNameZhCn
	// destStationNameZhTw
	// destStationNumber
	// lastPassStation
	// trainNumber
	// trainTypeEn
	// trainTypeIcon
	train_type_jp: String,
	// trainTypeKo
	// trainTypeZhCn
	// trainTypeZhTw
	wdf_block_no: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ReceiveTrainPosition {
	delay: String,
	// delay_en: String,
	// delay_ko: String,
	// delay_zh_cn: String,
	// delay_zh_tw: String,
	// location_col: String,
	// location_row: String,
	train_direction: String,
	// train_icon_type_image_jp: String,
	train_info_objects: Vec<ReceiveTrainPositionObject>,
	// train_type_vis_icon_vis: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ReceiveMovementInfo {
	// file_created_time: String,
	// file_version: String,
	// link_num: String,
	location_objects: Vec<ReceiveTrainPosition>,
}

fn find_all_train_after_move(train_info: &Vec<ArrivalInfo>, minutes: i64) -> Vec<ArrivalInfo> {
	train_info
		.iter()
		.filter(|train| {
			// 時間の整形
			let (train_arrival_hour_str, train_arrival_minutes_str) = train
				.real_departure_time
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
		.collect::<Vec<ArrivalInfo>>()
}

fn convert_recieve_train_info_to_arrival_info(
	train_info_list: &Vec<ReceiveTrainInfo>,
	movement_info: &ReceiveMovementInfo,
) -> Vec<ArrivalInfo> {
	train_info_list
		.iter()
		// 送信する形式に変更する
		.map(|train| {
			let neyagawa_arrival_info = train
				.dia_station_info_objects
				.iter()
				.filter(|station| station.station_name_jp == "寝屋川市")
				.collect::<Vec<&ReceiveStationInfo>>()[0];
			let movement_train_info = movement_info
				.location_objects
				.iter()
				.filter(|movement_info_train| {
					movement_info_train
						.train_info_objects
						.iter()
						.filter(|train_position_object| {
							train_position_object.wdf_block_no == train.wdf_block_no
						})
						.count() != 0
				})
				.collect::<Vec<_>>()[0];
			return ArrivalInfo {
				plan_departure_time: neyagawa_arrival_info.station_dep_time.clone(),
				real_departure_time: neyagawa_arrival_info.station_dep_time.clone(),
				train_type: movement_train_info.train_info_objects[0]
					.train_type_jp
					.clone(),
				terminal_station: movement_train_info.train_info_objects[0]
					.dest_station_name_jp
					.clone(),
				train_direction: movement_train_info.train_direction.clone(),
				is_delayed: if movement_train_info.delay == "" {
					true
				} else {
					false
				},
				delay_time: movement_train_info.train_info_objects[0]
					.delay_minutes
					.clone(),
				travel_mode: "歩き".to_string(),
			};
		})
		.collect_vec()
}

// #[tauri::command]
pub async fn get_train_info() -> anyhow::Result<()> {
	const TIMETABLE_URL: &str = "https://www.keihan.co.jp/zaisen-up/startTimeList.json";
	let train_timetable_res = reqwest::get(TIMETABLE_URL).await?.text().await?;
	let train_timetable = serde_json::from_str::<ReceiveStartTime>(&train_timetable_res)?;

	const MOVEMENT_INFO_URL: &str = "https://www.keihan.co.jp/zaisen-up/trainPositionList.json";
	let movement_info_res = reqwest::get(MOVEMENT_INFO_URL).await?.text().await?;
	let movement_info = serde_json::from_str::<ReceiveMovementInfo>(&movement_info_res)?;

	// 運行中の電車の番号リスト
	let wdf_block_num_list = movement_info
		.location_objects
		.iter()
		.map(|location_object| {
			location_object
				.train_info_objects
				.iter()
				.map(|train| train.wdf_block_no.clone())
				.collect::<Vec<String>>()
		})
		.collect::<Vec<Vec<String>>>()
		.concat();

	let neyagawa_arrive_train_info_list = train_timetable
		.train_info
		.iter()
		.filter(|train| wdf_block_num_list.contains(&train.wdf_block_no))
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

	let neyagawa_arrival_info_list =
		convert_recieve_train_info_to_arrival_info(&neyagawa_arrive_train_info_list, &movement_info);

	let neyagawa_arrival_train_info = TrainInfo {
		update_time: format!("{}", chrono::Local::now()),
		yodoyabashi_direction: find_all_train_after_move(
			&neyagawa_arrival_info_list
				.iter()
				// 淀屋橋方面に絞る
				.filter(|train| train.train_direction == "1")
				.map(|arrival_info| arrival_info.clone())
				.collect::<Vec<ArrivalInfo>>(),
			RUNNING_MINUTES as i64,
		),
		sanjo_direction: find_all_train_after_move(
			&neyagawa_arrival_info_list
				.iter()
				// 淀屋橋方面に絞る
				.filter(|train| train.train_direction == "0")
				.map(|arrival_info| arrival_info.clone())
				.collect::<Vec<ArrivalInfo>>(),
			RUNNING_MINUTES as i64,
		),
	};

	println!("{:?}", neyagawa_arrival_train_info);

	Ok(())
}
