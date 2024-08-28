import { useEffect, useState } from "react";
import TimeFormat from "../methods/TimeFormat.tsx";

interface Props {
  Get: () => void;
}

// 現在時刻を表示するコンポーネント
function CurrentTimeDisplay({ Get }: Props) {
  const [currentTime, setCurrentTime] = useState(TimeFormat(new Date()));

  // 現在時刻を管理するカスタムフック
  function useCurrentTime() {
    useEffect(() => {
      const intervalId = setInterval(() => {
        const nowTime = TimeFormat(new Date());
        if (currentTime != nowTime) {
          setCurrentTime(nowTime);
        }
      }, 1000);
      return () => clearInterval(intervalId);
    }, []);
  }

  // 1分ごとにデータを取得・反映する関数
  useEffect(() => {
    console.log(currentTime);
    Get();
  }, [currentTime]);

  useCurrentTime();
  return <h1>{currentTime}</h1>;
}

export default CurrentTimeDisplay;
