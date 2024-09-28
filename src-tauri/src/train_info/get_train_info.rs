use chrono::Timelike;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::train_info::delay_info_structs;

const WALKING_MINUTES: u16 = 16;
const RUNNING_MINUTES: u16 = 10;
const TIME_FORMAT: &str = "%Y/%m/%d %H:%M:%S %z";

#[derive(Serialize, Debug, Clone)]
pub struct DepartureInfo {
	pub plan_departure_time: String,
	pub real_departure_time: String,
	pub arrival_time: String,
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
	pub yodoyabashi_direction: Vec<DepartureInfo>,
	pub sanjo_direction: Vec<DepartureInfo>,
	pub is_stopped: bool,
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
	train_car: String,
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
	file_created_time: String,
	location_objects: Vec<ReceiveTrainPosition>,
}

/* 遅延情報等の情報ファイルリスト */
#[derive(Deserialize, Debug)]
struct ReceiveInfoFiles {
	#[serde(rename = "traininfo")]
	train_info: String,
}

fn find_neyagawa_station_info(station_list: &[ReceiveStationInfo]) -> Option<&ReceiveStationInfo> {
	station_list
		.iter()
		.find(|station| station.station_name_jp == "寝屋川市")
}

fn zero_padding(time_num: i32) -> String {
	if time_num < 10 {
		format!("0{}", time_num)
	} else {
		time_num.to_string()
	}
}

// 電車情報の時間をDateTimeにパース
fn parse_datetime_from_train_time(train_time: &str) -> chrono::DateTime<chrono::FixedOffset> {
	let now = chrono::Local::now();
	let (mut train_time_hour, train_time_minutes) = train_time
		.split(":")
		.map(|time| time.parse::<i32>().unwrap())
		.collect_tuple()
		.unwrap();
	let add_day: i64;
	// 1日のファイル更新開始が4時40分
	let is_after_day = now.hour() < 4 || (now.hour() == 4 && now.minute() < 40);
	if train_time_hour >= 24 {
		add_day = if is_after_day { 0 } else { 1 };
		train_time_hour -= 24;
	} else {
		add_day = if is_after_day { -1 } else { 0 };
	};
	chrono::DateTime::parse_from_str(
		&format!(
			"{} {}:{}:0 +0900",
			&(now + chrono::Duration::days(add_day))
				.format(TIME_FORMAT)
				.to_string()
				.split(" ")
				.collect::<Vec<&str>>()[0],
			zero_padding(train_time_hour),
			zero_padding(train_time_minutes),
		),
		TIME_FORMAT,
	)
	.unwrap()
}

fn convert_receive_train_info_to_arrival_info(
	train_info_list: &[ReceiveTrainInfo],
	movement_info: &ReceiveMovementInfo,
) -> Vec<DepartureInfo> {
	struct DirectionAndStartStation {
		start_station: String,
		direction: String,
	}
	const START_STATION: [&str; 8] = [
		"枚方市",
		"出町柳",
		"寝屋川市",
		"三条",
		"樟葉",
		"淀屋橋",
		"中之島",
		"淀",
	];
	// 淀屋橋方面 : 1, 出町柳方面 : 0
	const DIRECTION_LIST: [&str; 8] = ["1", "1", "0", "1", "1", "0", "0", "1"];
	let direction_list_from_station = START_STATION
		.iter()
		.zip(DIRECTION_LIST.iter())
		.map(|start_station_and_direction| DirectionAndStartStation {
			start_station: start_station_and_direction.0.to_string(),
			direction: start_station_and_direction.1.to_string(),
		})
		.collect::<Vec<DirectionAndStartStation>>();

	train_info_list
		.iter()
		// 送信する形式に変更する
		.map(|train| {
			let neyagawa_arrival_info =
				find_neyagawa_station_info(&train.dia_station_info_objects).unwrap();
			let movement_train_info = movement_info
				.location_objects
				.iter()
				.filter(|movement_train_info| {
					let train_position_objects = movement_train_info
						.train_info_objects
						.iter()
						.filter(|train_position_object| {
							train_position_object.wdf_block_no == train.wdf_block_no
						})
						.collect::<Vec<&ReceiveTrainPositionObject>>();
					!train_position_objects.is_empty()
				})
				.collect::<Vec<_>>();

			let exist_bound_info = !movement_train_info.is_empty();
			let plan_departure_time =
				parse_datetime_from_train_time(&neyagawa_arrival_info.station_dep_time)
					- chrono::Duration::minutes(
						if exist_bound_info && (!movement_train_info[0].delay.is_empty()) {
							movement_train_info[0].delay.parse::<i64>().unwrap()
						} else {
							0
						},
					);
			return DepartureInfo {
				plan_departure_time: format!(
					"{}:{}",
					zero_padding(plan_departure_time.hour() as i32),
					zero_padding(plan_departure_time.minute() as i32)
				),
				real_departure_time: neyagawa_arrival_info.station_dep_time.clone(),
				arrival_time: (parse_datetime_from_train_time(&neyagawa_arrival_info.station_dep_time)
					.naive_local()
					- chrono::Local::now().naive_local())
				.num_minutes()
				.to_string(),
				train_type: match exist_bound_info {
					true => movement_train_info[0].train_info_objects[0]
						.train_type_jp
						.clone(),
					false => String::from(""),
				},
				terminal_station: train.dia_station_info_objects[train.dia_station_info_objects.len() - 1]
					.station_name_jp
					.clone(),
				train_direction: match exist_bound_info {
					true => movement_train_info[0].train_direction.clone(),
					false => direction_list_from_station
						.iter()
						.filter(|ele| ele.start_station == train.dia_station_info_objects[0].station_name_jp)
						.collect::<Vec<&DirectionAndStartStation>>()[0]
						.direction
						.clone(),
				},
				is_delayed: exist_bound_info && (!movement_train_info[0].delay.is_empty()),
				delay_time: match exist_bound_info {
					true => movement_train_info[0].train_info_objects[0]
						.delay_minutes
						.clone(),
					false => String::from(""),
				},
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

	let delay_xml =
		serde_xml_rs::from_str::<delay_info_structs::ReceiveDelayInfo>(&delay_xml_res).unwrap();

	let mut train_info_list_only_neyagawa = train_timetable
		.train_info
		.iter()
		// 走らない電車を除く
		.filter(|train| train.train_car != "00000")
		// 止まらない駅を除く
		.map(|train| ReceiveTrainInfo {
			wdf_block_no: train.wdf_block_no.clone(),
			ext_train: train.ext_train.clone(),
			train_car: train.train_car.clone(),
			premium_car: train.premium_car.clone(),
			dia_station_info_objects: train
				.dia_station_info_objects
				.iter()
				.filter(|&station| station.station_dep_time != "99:99")
				.cloned()
				.collect::<Vec<ReceiveStationInfo>>(),
		})
		// 寝屋川市に止まる電車のみを取得
		.filter(|train| find_neyagawa_station_info(&train.dia_station_info_objects).is_some())
		// 最低走って間に合う必要があるためそれ以上のものだけを取得
		.filter(|train| {
			parse_datetime_from_train_time(
				&find_neyagawa_station_info(&train.dia_station_info_objects)
					.unwrap()
					.station_dep_time,
			) > chrono::Local::now() + chrono::Duration::minutes(RUNNING_MINUTES as i64)
		})
		.collect::<Vec<ReceiveTrainInfo>>();

	// 日付順にソート
	train_info_list_only_neyagawa.sort_by(|a, b| {
		let a_arrive_time_string = &find_neyagawa_station_info(&a.dia_station_info_objects)
			.unwrap()
			.station_dep_time;
		let b_arrive_time_string = &find_neyagawa_station_info(&b.dia_station_info_objects)
			.unwrap()
			.station_dep_time;
		let a_arrive_time = parse_datetime_from_train_time(a_arrive_time_string);
		let b_arrive_time = parse_datetime_from_train_time(b_arrive_time_string);
		a_arrive_time.cmp(&b_arrive_time)
	});

	let mut train_dep_info_list =
		convert_receive_train_info_to_arrival_info(&train_info_list_only_neyagawa, &movement_info)
			.iter()
			.filter(|arrival_info| arrival_info.terminal_station != "寝屋川市")
			.cloned()
			.collect::<Vec<DepartureInfo>>();

	train_dep_info_list.iter_mut().for_each(|arrival_info| {
		if parse_datetime_from_train_time(&arrival_info.real_departure_time)
			> (chrono::Local::now() + chrono::Duration::minutes(WALKING_MINUTES as i64))
		{
			arrival_info.travel_mode = "歩き".to_string();
		};
	});

	train_dep_info_list.sort_by(|a, b| {
		let a_arrive_time = parse_datetime_from_train_time(&a.real_departure_time);
		let b_arrive_time = parse_datetime_from_train_time(&b.real_departure_time);
		a_arrive_time.cmp(&b_arrive_time)
	});

	// 京阪本線が運行休止中は寝屋川市も止まっているため本線が休止しているか確認
	let mut is_stopped = delay_xml.info.dif.all != 0;
	if is_stopped {
		is_stopped = delay_xml.info.dif.eif.unwrap().iter().any(|eif| {
			eif
				.lin
				.iter()
				.any(|lin| lin.nm.iter().any(|nm| nm.value.contains("京阪本線")))
		})
	}

	let mut train_dep_info_from_neyagawa = TrainInfo {
		update_time: format!(
			"{}:{}",
			&movement_info.file_created_time[8..=9],
			&movement_info.file_created_time[10..=11]
		),
		yodoyabashi_direction: train_dep_info_list
			.iter()
			// 淀屋橋方面に絞る
			.filter(|train| train.train_direction == "1")
			.cloned()
			.collect::<Vec<DepartureInfo>>(),
		sanjo_direction: train_dep_info_list
			.iter()
			// 三条方面に絞る
			.filter(|train| train.train_direction == "0")
			.cloned()
			.collect::<Vec<DepartureInfo>>(),
		is_stopped,
	};

	train_dep_info_from_neyagawa.yodoyabashi_direction = train_dep_info_from_neyagawa
		.yodoyabashi_direction[..2usize.min(train_dep_info_from_neyagawa.yodoyabashi_direction.len())]
		.to_vec();

	train_dep_info_from_neyagawa.sanjo_direction = train_dep_info_from_neyagawa.sanjo_direction
		[..2usize.min(train_dep_info_from_neyagawa.sanjo_direction.len())]
		.to_vec();

	Ok(train_dep_info_from_neyagawa)
}
