import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { TrainInfoList } from "./types/TrainInfo.d.tsx";
import StopRunningModal from "./components/StopRunningModal.tsx";
import ErrorModal from "./components/ErrorModal.tsx";
import TrainInfoComp from "./components/TrainInfoComp.tsx";
import Heder from "./components/Heder.tsx";
import Footer from "./components/Footer.tsx";
import "./App.css";

// メイン実装部分
function App() {
  // 変数定義
  const [trainInfoList, setTrainInfoList] = useState<TrainInfoList>();
  const [errorOpen, setErrorOpen] = useState(false);
  const [stopRunningOpen, setStopRunningOpen] = useState(false);
  const ErrorHandleOpen = () => setErrorOpen(true);
  const ErrorHandleClose = () => setErrorOpen(false);
  const StopRunningHandleOpen = () => setStopRunningOpen(true);
  const StopRunningHandleClose = () => setStopRunningOpen(false);

  // 電車情報を取得する関数
  function get() {
    invoke<TrainInfoList>("get_train_info")
      .then((res) => {
        console.log(res);
        setTrainInfoList(res);

        // モーダル系の処理
        if (res.is_stopped) {
          StopRunningHandleOpen();
        } else {
          StopRunningHandleClose();
        }
        ErrorHandleClose();

        // モーダルテスト用
        // ErrorHandleOpen();
        // StopRunningHandleOpen();
      })
      .catch((e) => {
        console.log(e);
        StopRunningHandleClose();
        ErrorHandleOpen();
      });
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
              padding: "20px",
              border: "1px solid #ccc",
              borderRadius: "8px",
            }}
          >
            <p
              className={trainInfoList?.is_stopped ? "red" : "white"}
              style={{ lineHeight: "1rem", margin: "1vh", fontSize: "3vmin" }}
            >
              {trainInfoList?.is_stopped
                ? "現在遅延が発生しています。詳しくは公式サイトを確認してください。"
                : "現在３０分以上の遅延はありません。"}
            </p>
            <p
              style={{
                textAlign: "end",
                lineHeight: "1rem",
                margin: "1vh",
                fontSize: "2.25vmin",
              }}
            >
              {trainInfoList?.update_time} 更新
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
