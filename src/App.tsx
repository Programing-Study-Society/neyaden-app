import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const respons = invoke("greet");
  console.log(respons);
  (async () => {
    console.log(await invoke("get_train_info"));
  })();
  return (
    <div>
      {/* ヘッダー */}
      <header>
        <div className="flex border">
          <div>
            <div className="flex">
              <h2>寝屋川市駅</h2>
              <h2>電車状況</h2>
            </div>
            <div className="flex">
              <p>9:50</p>
              <p>更新</p>
            </div>
          </div>
          <div className="flex right now-time">
            <h1>現在</h1>
            <h1>9:53</h1>
          </div>
        </div>
      </header>

      <body>
        {/* 淀屋橋・中之島線方面 */}
        <div className="border">
          <div className="flex-right">
            <h1>淀屋橋・中之島線</h1>
            <h3>方面</h3>
          </div>
          {/* 1電車目 */}
          <div>
            <div className="flex">
              <h2 className="">10:00</h2>
              <h2>発</h2>
              <h2 className="orange">急行</h2>
              <h2>淀屋橋行き</h2>
              <h2 className="red display-none">遅延5分</h2>
            </div>
            <div className="flex">
              <h2 className="top red display-none">10:00</h2>
              <h2 className="right red">走れば間に合います</h2>
            </div>
          </div>

          {/* 2電車目 */}
          <div>
            <div className="flex margin">
              <h2 className="correction-line">10:05</h2>
              <h2>発</h2>
              <h2 className="blue">準急</h2>
              <h2>淀屋橋行き</h2>
              <h2 className="red">遅延5分</h2>
            </div>
            <div className="flex">
              <h2 className="top red">10:10</h2>
              <h2 className="right yellow">歩きで間に合います</h2>
            </div>
          </div>
        </div>

        {/* 三条・出町柳方面 */}
        <div>
          <div className="flex-right margin">
            <h1>三条・出町柳</h1>
            <h3>方面</h3>
          </div>
          {/* 1電車目 */}
          <div>
            <div className="flex">
              <h2 className="">9:55</h2>
              <h2>発</h2>
              <h2 className="white">普通</h2>
              <h2>三条行き</h2>
              <h2 className="red display-none">遅延5分</h2>
            </div>
            <div className="flex">
              <h2 className="top red display-none">10:00</h2>
              <h2 className="right yellow">歩きで間に合います</h2>
            </div>
          </div>

          {/* 2電車目 */}
          <div>
            <div className="flex margin">
              <h2 className="correction-line">9:45</h2>
              <h2>発</h2>
              <h2 className="white">普通</h2>
              <h2>淀屋橋行き</h2>
              <h2 className="red">遅延15分</h2>
            </div>
            <div className="flex">
              <h2 className="top red">10:00</h2>
              <h2 className="right red">走れば間に合います</h2>
            </div>
          </div>
        </div>
      </body>
    </div>
  );
}

function exampleApp() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
