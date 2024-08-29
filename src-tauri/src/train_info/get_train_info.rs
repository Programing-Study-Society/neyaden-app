use chrono::Timelike;
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
	station_dep_time: String,
	station_name_jp: String,
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
	#[serde(rename = "TrainInfo")]
	train_info: Vec<ReceiveTrainInfo>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ReceiveTrainPositionObject {
	delay_minutes: String,
	dest_station_name_jp: String,
	train_type_jp: String,
	wdf_block_no: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ReceiveTrainPosition {
	delay: String,
	train_direction: String,
	train_info_objects: Vec<ReceiveTrainPositionObject>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ReceiveMovementInfo {
	location_objects: Vec<ReceiveTrainPosition>,
}

// 電車情報の時間をDateTimeにパース
fn parse_datetime_from_train_time(train_time: &str) -> chrono::DateTime<chrono::FixedOffset> {
	let now = chrono::Local::now();
	let (mut train_time_hour, train_time_minutes) = train_time
		.split(":")
		.map(|time| time.parse::<i32>().unwrap())
		.collect_tuple()
		.unwrap();
	let mut add_day = 0;
	train_time_hour = if train_time_hour >= 24 {
		add_day = if 0 as u32 <= now.hour() && now.hour() <= 5 {
			0
		} else {
			1
		};
		train_time_hour - 24
	} else {
		train_time_hour
	};
	chrono::DateTime::parse_from_str(
		&format!(
			"{} {}:{}:0 +0900",
			&(now + chrono::Duration::days(add_day as i64))
				.format(&TIME_FORMAT)
				.to_string()
				.split(" ")
				.collect::<Vec<&str>>()[0],
			if train_time_hour < 10 {
				format!("0{}", train_time_hour)
			} else {
				train_time_hour.to_string()
			},
			if train_time_minutes < 10 {
				format!("0{}", train_time_minutes)
			} else {
				train_time_minutes.to_string()
			},
		),
		&TIME_FORMAT,
	)
	.unwrap()
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
					false
				} else {
					true
				},
				delay_time: movement_train_info.train_info_objects[0]
					.delay_minutes
					.clone(),
				travel_mode: "走り".to_string(),
			};
		})
		.collect_vec()
}

#[tauri::command]
pub async fn get_train_info() -> Result<TrainInfo, String> {
	const TIMETABLE_URL: &str = "https://www.keihan.co.jp/zaisen-up/startTimeList.json";
	let train_timetable_res = reqwest::get(TIMETABLE_URL)
		.await
		.map_err(|_| "情報の取得に失敗しました")?
		.text()
		.await
		.map_err(|_| "情報の取得に失敗しました")?;
	let train_timetable = serde_json::from_str::<ReceiveStartTime>(&train_timetable_res).unwrap();

	const MOVEMENT_INFO_URL: &str = "https://www.keihan.co.jp/zaisen-up/trainPositionList.json";
	let movement_info_res = reqwest::get(MOVEMENT_INFO_URL)
		.await
		.map_err(|_| "情報の取得に失敗しました")?
		.text()
		.await
		.map_err(|_| "情報の取得に失敗しました")?;
	let movement_info = serde_json::from_str::<ReceiveMovementInfo>(&movement_info_res).unwrap();

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

	let mut neyagawa_arrive_train_info_list = train_timetable
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
		// 最低走って間に合う必要があるためそれ以上のものだけを取得
		.filter(|train| {
			parse_datetime_from_train_time(
				&train
					.dia_station_info_objects
					.iter()
					.filter(|station| station.station_name_jp == "寝屋川市")
					.collect::<Vec<&ReceiveStationInfo>>()[0]
					.station_dep_time,
			) >= chrono::Local::now() + chrono::Duration::minutes(RUNNING_MINUTES as i64)
		})
		.collect::<Vec<ReceiveTrainInfo>>();

	// 日付順にソート
	neyagawa_arrive_train_info_list.sort_by(|a, b| {
		let a_arrive_time_string = &a
			.dia_station_info_objects
			.iter()
			.filter(|station| station.station_name_jp == "寝屋川市")
			.collect::<Vec<&ReceiveStationInfo>>()[0]
			.station_dep_time;
		let b_arrive_time_string = &b
			.dia_station_info_objects
			.iter()
			.filter(|station| station.station_name_jp == "寝屋川市")
			.collect::<Vec<&ReceiveStationInfo>>()[0]
			.station_dep_time;
		let a_arrive_time = parse_datetime_from_train_time(&a_arrive_time_string);
		let b_arrive_time = parse_datetime_from_train_time(&b_arrive_time_string);
		a_arrive_time.cmp(&b_arrive_time)
	});

	let neyagawa_arrival_info_list =
		convert_recieve_train_info_to_arrival_info(&neyagawa_arrive_train_info_list, &movement_info);

	let now = chrono::Local::now();
	let mut neyagawa_arrival_train_info = TrainInfo {
		update_time: format!(
			"{}:{}",
			if now.hour() < 10 {
				format!("0{}", now.hour())
			} else {
				now.hour().to_string()
			},
			if now.minute() < 10 {
				format!("0{}", now.minute())
			} else {
				now.minute().to_string()
			}
		),
		yodoyabashi_direction: neyagawa_arrival_info_list
			.iter()
			// 淀屋橋方面に絞る
			.filter(|train| train.train_direction == "1")
			.map(|arrival_info| arrival_info.clone())
			.collect::<Vec<ArrivalInfo>>(),
		sanjo_direction: neyagawa_arrival_info_list
			.iter()
			// 三条方面に絞る
			.filter(|train| train.train_direction == "0")
			.map(|arrival_info| arrival_info.clone())
			.collect::<Vec<ArrivalInfo>>(),
	};

	neyagawa_arrival_train_info.yodoyabashi_direction = neyagawa_arrival_train_info
		.yodoyabashi_direction[..(2usize.min(neyagawa_arrival_train_info.yodoyabashi_direction.len()))]
		.to_vec();

	neyagawa_arrival_train_info
		.yodoyabashi_direction
		.iter_mut()
		.for_each(|arrival_info| {
			if parse_datetime_from_train_time(&arrival_info.real_departure_time)
				> (chrono::Local::now() + chrono::Duration::minutes(WALKING_MINUTES as i64))
			{
				arrival_info.travel_mode = "歩き".to_string();
			};
		});

	neyagawa_arrival_train_info.sanjo_direction = neyagawa_arrival_train_info.sanjo_direction
		[..(2usize.min(neyagawa_arrival_train_info.sanjo_direction.len() as usize))]
		.to_vec();

	neyagawa_arrival_train_info
		.sanjo_direction
		.iter_mut()
		.for_each(|arrival_info| {
			if parse_datetime_from_train_time(&arrival_info.real_departure_time)
				> (chrono::Local::now() + chrono::Duration::minutes(WALKING_MINUTES as i64))
			{
				arrival_info.travel_mode = "歩き".to_string();
			};
		});

	Ok(neyagawa_arrival_train_info)
}
