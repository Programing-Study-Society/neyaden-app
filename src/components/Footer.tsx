// フッター部分のコンポーネント
function Footer() {
  return (
    <footer >
      <div className="ori" style={{fontSize:"2.25vmin", lineHeight: "1", padding: "1.1vmin"}}>
        <p style={{wordWrap:"break-word", margin: "0.75vh 0"}}>このアプリはプログラミング研究会が作成しました。</p>
        <p style={{wordWrap:"break-word", margin: "0.75vh 0"}}>プログラミング研究会では、授業で教わる知識をもとに、ゲームを作ったり勉強会を開いたり様々なITにまつわる活動を行っています。</p>
        <p style={{wordWrap:"break-word", margin: "0.75vh 0"}}>部員募集中！！</p>
      </div>
      {/* <MySlider size="24vh" slideImageLinks={imageLinks} /> */}
      <p style={{fontSize:"2vmin", textAlign: 'right', marginRight: '0.25vw'}}>何かあれば proken0603@gmail.com まで</p>
      </footer>
  );
}

export default Footer;
