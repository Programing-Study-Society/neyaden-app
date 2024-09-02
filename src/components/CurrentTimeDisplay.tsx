import { useEffect, useState } from "react";

interface Props {
  Get: () => void;
}

// 時間をhh:mm形式に変換する関数
function TimeFormat(date: Date | string | undefined) {
  // stringの場合、時間に変換する
  if (typeof date == "string") {
    date = new Date(date);
  }

  // 24時間表記で秒を省略
  const formattedTime = date?.toLocaleTimeString("ja-JP", {
    hour: "2-digit",
    minute: "2-digit",
    hour12: false,
  });

  return formattedTime;
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
