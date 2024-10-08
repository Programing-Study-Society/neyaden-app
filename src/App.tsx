import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { TrainInfoList, StopTrainInfo} from "./types/TrainInfo.d.tsx";
import StopRunningModal from "./components/StopRunningModal.tsx";
import ErrorModal from "./components/ErrorModal.tsx";
import TrainInfoComp from "./components/TrainInfoComp.tsx";
import StopTrain from "./components/StopTrain.tsx";
import Heder from "./components/Heder.tsx";
import Footer from "./components/Footer.tsx";
import "./App.css";
import { fontSize } from "@mui/system";

// メイン実装部分
function App() {
  // 変数定義
  const [trainInfoList, setTrainInfoList] = useState<TrainInfoList>();
  const [stopTrainInfo, setStopTrainInfo] = useState<StopTrainInfo>();
  const [errorOpen, setErrorOpen] = useState(false);
  const [stopRunningOpen, setStopRunningOpen] = useState(false);
  const ErrorHandleOpen = () => setErrorOpen(true);
  const ErrorHandleClose = () => setErrorOpen(false);
  const StopRunningHandleOpen = () => setStopRunningOpen(true);
  const StopRunningHandleClose = () => setStopRunningOpen(false);

  // 電車情報を取得する関数
  async function get() {
    try {
      await invoke<TrainInfoList>("get_train_info")
      .then((res) => {
        console.log(res);
        setTrainInfoList(res);
      })
      .catch((e) => {
        throw e;
      });

      await invoke<StopTrainInfo>("get_stop_train_info")
      .then((res) => {
        console.log(res);
        setStopTrainInfo(res);
        if (res.keihan) {
          StopRunningHandleOpen();
        } else {
          StopRunningHandleClose();
        }
      })
      .catch((e) => {
        throw e;
      });

      ErrorHandleClose();
    } catch(e) {
      console.log(e);
      StopRunningHandleClose();
      ErrorHandleOpen();
    }
  }

  return (
    <div className="content-wrapper" style={{ minHeight: "100%" }}>
      <div className="content">
        {/* ヘッダー */}
        <Heder updateTime={trainInfoList?.update_time} Get={get} />
        <span className="border"></span>
        <main>
          {/* 淀屋橋・中之島線方面 */}
          <div className="train-info-container">
            <div className="flex-right">
              <h1 className="direction-name">淀屋橋・中之島線</h1>
              <h3>方面</h3>
            </div>
            {/* 1電車目 */}
            <TrainInfoComp
              trainInfo={trainInfoList?.yodoyabashi_direction[0]}
            />

            {/* 2電車目 */}
            <TrainInfoComp
              trainInfo={trainInfoList?.yodoyabashi_direction[1]}
            />
          </div>
          <span className="border"></span>

          {/* 三条・出町柳方面 */}
          <div className="train-info-container">
            <div className="flex-right">
              <h1 className="direction-name">三条・出町柳</h1>
              <h3>方面</h3>
            </div>
            {/* 1電車目 */}
            <TrainInfoComp trainInfo={trainInfoList?.sanjo_direction[0]} />

            {/* 2電車目 */}
            <TrainInfoComp trainInfo={trainInfoList?.sanjo_direction[1]} />
          </div>
          <span className="border"></span>
          <div
            style={{
              margin: "2vh 0",
              padding: "15px",
              border: "1px solid #ccc",
              borderRadius: "8px",
            }}
          >
            <div style={{display: "flex", flexDirection: "column"}}>
              <StopTrain routeNeme={"京阪本線"} routeStatus={stopTrainInfo?.keihan} />
              <StopTrain routeNeme={"JR大阪環状線"} routeStatus={false} />
              <StopTrain routeNeme={"JR学研都市線"} routeStatus={true} />
              <StopTrain routeNeme={"大阪メトロ御堂筋線"} routeStatus={false} />
              <StopTrain routeNeme={"近鉄京都線"} routeStatus={false} />
            </div>
            <p
              style={{
                margin: "0",
                marginTop: "2vh",
                textAlign: "end",
                lineHeight: "1rem",
                fontSize: "2.25vmin",
              }}
            >
              {trainInfoList?.update_time} 更新　正確な情報は公式サイトを確認してください
            </p>
          </div>
          <StopRunningModal
            isOpen={stopRunningOpen}
            handleClose={StopRunningHandleClose}
          />
          <ErrorModal isOpen={errorOpen} handleClose={ErrorHandleClose} />
        </main>
      </div>
      {/* フッター */}
          <Footer />

    </div>
  );
}

export default App;
