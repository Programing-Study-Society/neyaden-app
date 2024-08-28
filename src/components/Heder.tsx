import CurrentTimeDisplay from "../components/CurrentTimeDisplay.tsx";

interface Props {
  updateTime: string | undefined;
  Get: () => void;
}

// ヘッダー部分のコンポーネント
function Heder({ updateTime, Get }: Props) {
  return (
    <header>
      <div className="flex border">
        <div>
          <div className="flex">
            <h2>寝屋川市駅</h2>
            <h2>電車状況</h2>
          </div>
          <div className="flex">
            <p>{updateTime}</p>
            <p>更新</p>
          </div>
        </div>
        <div className="flex right now-time">
          <h1>現在</h1>
          <CurrentTimeDisplay Get={Get} />
        </div>
      </div>
    </header>
  );
}

export default Heder;
