// JSONを受け取るための型定義
type TRAIN_TYPE =
  | "快速特急 洛楽"
  | "特急"
  | "ライナー"
  | "快速急行"
  | "通勤快急"
  | "急行"
  | "深夜急行"
  | "準急"
  | "通勤準急"
  | "区間急行"
  | "普通"
  | "通勤急行"
  | "臨時列車";

type TRAVEl_MODE = "歩き" | "走り";

export interface TrainInfo {
  plan_departure_time: string;
  real_departure_time: string;
  arrival_time: string;
  train_type: TRAIN_TYPE;
  terminal_station: string;
  is_delayed: boolean;
  delay_time: string;
  travel_mode: TRAVEl_MODE;
}

export interface TrainInfoList {
  update_time: string;
  yodoyabashi_direction: TrainInfo[];
  sanjo_direction: TrainInfo[];
}

export interface StopTrainInfo {
  keihan: boolean;
}

export const GetTrainTypeColor = function (trainType: TRAIN_TYPE) {
  const cleanedTrainType = trainType.replace(/臨時\s+/g, '');
  switch (cleanedTrainType) {
    case "快速特急 洛楽":
      return "pink";
    case "特急":
      return "red";
    case "ライナー":
      return "red";
    case "快速急行":
      return "purple";
    case "通勤快急":
      return "purple";
    case "急行":
      return "orange";
    case "深夜急行":
      return "orange";
    case "準急":
      return "blue";
    case "通勤準急":
      return "blue";
    case "区間急行":
      return "green";
    case "普通":
      return "white";
    case "通勤急行":
      return "orange";
    case "臨時列車":
      return "white";
    default:
      return "white";
  }
};

export const GetTravelMode = function (mode: TRAVEl_MODE) {
  switch (mode) {
    case "歩き":
      return "歩いても間に合います";
    case "走り":
      return "走れば間に合います";
    default:
      return "";
  }
};

export const GetTravelModeColor = function (mode: TRAVEl_MODE) {
  switch (mode) {
    case "歩き":
      return "yellow";
    case "走り":
      return "red";
    default:
      return "white";
  }
};

export default TrainInfo;
