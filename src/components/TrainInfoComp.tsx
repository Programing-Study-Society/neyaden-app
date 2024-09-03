import {
  GetTrainTypeColor,
  GetTravelMode,
  GetTravelModeColor,
  TrainInfo,
} from "../types/TrainInfo.d.tsx";

// 各電車の情報を表示するコンポーネント
function TrainInfoComp({ trainInfo }: { trainInfo: TrainInfo | undefined }) {
  if (trainInfo === undefined) return;

  return (
    <div className="margin10">
      <div className="flex">
        <h2
          id="planTime"
          className={trainInfo.is_delayed ? "correction-line" : ""}
        >
          {trainInfo.plan_departure_time}
        </h2>
        <h2>発</h2>
        <h2 className={GetTrainTypeColor(trainInfo.train_type)}>
          {trainInfo.train_type}
        </h2>
        <h2>{trainInfo?.terminal_station}行き</h2>
        <h2 className={"red " + (trainInfo.is_delayed ? "" : "display-none")}>
          遅延{trainInfo.delay_time}分
        </h2>
      </div>
      <div className="flex">
        <h2
          className={"top red " + (trainInfo.is_delayed ? "" : "display-none")}
        >
          {trainInfo.real_departure_time}
        </h2>
        <h2 className={"right " + GetTravelModeColor(trainInfo.travel_mode)}>
          {`あと${trainInfo.arrival_time}分　` + GetTravelMode(trainInfo.travel_mode)}
        </h2>
      </div>
    </div>
  );
}

export default TrainInfoComp;
