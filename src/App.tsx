import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { TrainInfoList } from "./types/TrainInfo.d.tsx";
import ErrorModal from "./components/ErrorModal.tsx";
import TrainInfoComp from "./components/TrainInfoComp.tsx";
import Heder from "./components/Heder.tsx";
import Footer from "./components/Footer.tsx";
import "./App.css";

// メイン実装部分
function App() {
  // 変数定義
  const [trainInfoList, setTrainInfoList] = useState<TrainInfoList>();
  const [open, setOpen] = useState(false);
  const handleOpen = () => setOpen(true);
  const handleClose = () => setOpen(false);

  // 電車情報を取得する関数
  function get() {
    invoke<TrainInfoList>("get_train_info")
      .then((res) => {
        console.log(res);
        setTrainInfoList(res);
        handleClose();
      })
      .catch((e) => {
        console.log(e);
        handleOpen();
      });
  }

  return (
    <div style={{ minHeight: "100%" }}>
      {/* ヘッダー */}
      <Heder updateTime={trainInfoList?.update_time} Get={get} />
      <span className="border"></span>
      <main>
        {/* 淀屋橋・中之島線方面 */}
        <div>
          <div className="flex-right">
            <h1 className="direction-name">淀屋橋・中之島線</h1>
            <h3>方面</h3>
          </div>
          {/* 電車情報 */}
          {trainInfoList?.yodoyabashi_direction.map((train) => (
            <TrainInfoComp trainInfo={train} />
          ))}
        </div>
        <span className="border"></span>

        {/* 三条・出町柳方面 */}
        <div>
          <div className="flex-right">
            <h1 className="direction-name">三条・出町柳</h1>
            <h3>方面</h3>
          </div>
          {/* 電車情報 */}
          {trainInfoList?.sanjo_direction.map((train) => (
            <TrainInfoComp trainInfo={train} />
          ))}
        </div>
        <span className="border"></span>
        <ErrorModal isOpen={open} handleClose={handleClose} />
      </main>
      {/* フッター */}
      <Footer />
    </div>
  );
}

export default App;
