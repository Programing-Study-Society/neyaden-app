// 各路線の遅延状況を表示するコンポーネント
function StopTrain({ routeNeme, routeStatus}: { routeNeme: String, routeStatus: Boolean | undefined}) {
  if (routeStatus === undefined) return (
    <p
      className={"white"}
      style={{ lineHeight: "1rem", margin: "1vh", fontSize: "3vmin" }}
    >
      {"情報取得中にエラーが発生しました。"}
    </p>
  );

  return (
    <div style={{display: "flex", flexDirection: "row", justifyContent: "space-between", width: "100%"}}>
      <p style={{fontSize: "3vmin"}}>{routeNeme}</p>
      <p
      className={routeStatus ? "red" : "white"}
      style={{ fontSize: "3vmin"}}
    >
      {routeStatus
        ? "遅延が発生しています"
        : "現在３０分以上の遅延はありません"}
    </p>
    </div>
  );
}

export default StopTrain;
