// TODO 現状1ファイルからの参照のみであるため、ファイル分けの必要がないかもしれない
// TODO 引数の型も1つで十分な可能性

// 時間をhh:mm形式に変換する関数
export const TimeFormat = function (date: Date | string | undefined) {
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
};

export default TimeFormat;
