// フッター部分のコンポーネント
function Footer() {
	return (
		<footer>
			<div
				className="ori"
				style={{ fontSize: "2vmin", lineHeight: "2.5vh", padding: "1.1vmin",  marginLeft: "auto",  marginRight: "auto", width: "fit-content"}}
			>
				<p style={{ wordWrap: "break-word", margin: "0" }}>
					この掲示板はプログラミング研究会に所属する情報工学科の有志が作成しました。
				</p>
				<p style={{ wordWrap: "break-word", margin: "0" }}>
					プログラミング研究会では、授業で教わる知識をもとに、ゲームを作ったり勉強会を
					<br />
					開いたり様々なITにまつわる活動を行っています。
				</p>
				<p style={{ wordWrap: "break-word", margin: "0", textAlign:"right"}}>何かあれば proken0603@gmail.com まで</p>
				<p
					style={{
						wordWrap: "break-word",
						margin: "2vh 0",
						marginBottom:"0",
						textAlign: "center",
						fontSize: "4vmin",
						color: "#FF5760",
						fontWeight: "bold",
					}}
				>
					メンバー募集中！！
				</p>
			</div>
			{/* <MySlider size="24vh" slideImageLinks={imageLinks} /> */}
			<div style={{fontSize:"1.0vh", textAlign:"center", marginTop:"2.5vh"}}>
				<p>情報引用元</p>
				<p>京阪電車（京阪線） 列車走行位置 (https://www.keihan.co.jp/zaisen/)</p>
				<p>JR西日本 列車走行位置 (https://www.train-guide.westjr.co.jp/)</p>
      		</div>
		</footer>
	);
}

export default Footer;
