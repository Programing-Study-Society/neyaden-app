// ヘッダー部分のコンポーネント
function Footer() {
  return (
    <footer className="flex">
      <div className="ori" style={{fontSize:"10px"}}>
        <p style={{wordWrap:"break-word"}}>このアプリはプログラミング研究会が作成しました。</p>
        <p>プログラミング研究会は情報通信工学部・情報工学科の融資が設立した団体です。</p>
        <p>プログラミング研究会では、授業で教わる知識をもとに、ゲームを作ったり勉強会を開いたり様々なITにまつわる活動を行っています。</p>
        <p>部員募集中！！</p>
      </div>
    </footer>
  );
}

export default Footer;
