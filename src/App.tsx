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
          {/* 1電車目 */}
          <TrainInfoComp trainInfo={trainInfoList?.yodoyabashi_direction[0]} />

          {/* 2電車目 */}
          <TrainInfoComp trainInfo={trainInfoList?.yodoyabashi_direction[1]} />
        </div>
        <span className="border"></span>

        {/* 三条・出町柳方面 */}
        <div>
          <div className="flex-right">
            <h1 className="direction-name">三条・出町柳</h1>
            <h3>方面</h3>
          </div>
          {/* 1電車目 */}
          <TrainInfoComp trainInfo={trainInfoList?.sanjo_direction[0]} />

          {/* 2電車目 */}
          <TrainInfoComp trainInfo={trainInfoList?.sanjo_direction[1]} />
        <span className="border"></span>
        </div>
        <div style={{
        padding: '10px',
        border: '1px solid #ccc',
        borderRadius: '8px',
        fontSize:"15px",
      }}>
          <p>現在３０分以上の遅れはございません。</p>
          <p>00:00 更新</p>
        </div>
        <ErrorModal isOpen={open} handleClose={handleClose} />
      </main>
      {/* フッター */}
      <Footer />
    </div>
  );
}

export default App;
