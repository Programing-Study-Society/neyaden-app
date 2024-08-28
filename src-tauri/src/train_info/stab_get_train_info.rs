use super::get_train_info::{ArrivalInfo, TrainInfo};

#[tauri::command]
pub fn get_train_info() -> TrainInfo {
	TrainInfo {
		update_time: String::from("2024-08-26T09:30+09:00"),
		yodoyabashi_direction: vec![
			ArrivalInfo {
				plan_departure_time: String::from("2024-08-26T09:45:00+09:00"),
				real_departure_time: String::from("2024-08-26T09:45:00+09:00"),
				train_type: String::from("急行"),
				terminal_station: String::from("淀屋橋"),
				is_delayed: false,
				delay_time: String::from("00:00"),
				travel_mode: String::from("走り"),
			},
			ArrivalInfo {
				plan_departure_time: String::from("2024-08-26T09:50:00+09:00"),
				real_departure_time: String::from("2024-08-26T09:55:00+09:00"),
				train_type: String::from("準急"),
				terminal_station: String::from("中之島"),
				is_delayed: true,
				delay_time: String::from("00:05"),
				travel_mode: String::from("歩き"),
			},
		],
		sanjo_direction: vec![
			ArrivalInfo {
				plan_departure_time: String::from("2024-08-26T09:15:00+09:00"),
				real_departure_time: String::from("2024-08-26T09:42:00+09:00"),
				train_type: String::from("普通"),
				terminal_station: String::from("出町柳"),
				is_delayed: true,
				delay_time: String::from("00:27"),
				travel_mode: String::from("走り"),
			},
			ArrivalInfo {
				plan_departure_time: String::from("2024-08-26T09:45:00+09:00"),
				real_departure_time: String::from("2024-08-26T09:45:00+09:00"),
				train_type: String::from("普通"),
				terminal_station: String::from("三条"),
				is_delayed: false,
				delay_time: String::from("00:00"),
				travel_mode: String::from("走り"),
			},
		],
	}
}
