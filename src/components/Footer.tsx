import MySlider from './MySlider/MySlider';

const imageLinks = [
  "20221007_162315.JPG",
  "20240719_175509.jpg",
  "IMG_20230914_100009.jpg",
  "IMG_0505.jpg",
  "IMG_3238.jpg",
  "ProgrammingStudySession.jpg"
]

// ヘッダー部分のコンポーネント
function Footer() {
  return (
    <footer >
      <div className="ori" style={{fontSize:"1.25vh", lineHeight: "1", padding: "25px"}}>
        <p style={{wordWrap:"break-word"}}>このアプリはプログラミング研究会が作成しました。</p>
        <p>プログラミング研究会では、授業で教わる知識をもとに、ゲームを作ったり勉強会を開いたり様々なITにまつわる活動を行っています。</p>
        <p>部員募集中！！</p>
      </div>
      <MySlider size="62.5%" slideImageLinks={imageLinks} />
      <p style={{fontSize:"1.25vh", textAlign: 'right', marginRight: '0.25vw'}}>何かあれば proken0603@gmail.com まで</p>
      </footer>
  );
}

export default Footer;
