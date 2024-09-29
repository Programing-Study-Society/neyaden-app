// フッター部分のコンポーネント
function Footer() {
	return (
		<footer>
			<div
				className="ori"
				style={{ fontSize: "2vmin", lineHeight: "2.5vh", padding: "1.1vmin" }}
			>
				<p style={{ wordWrap: "break-word", margin: "0" }}>
					この掲示板はプログラミング研究会に所属する情報工学科の有志が作成しました。
				</p>
				<p style={{ wordWrap: "break-word", margin: "0" }}>
					プログラミング研究会では、授業で教わる知識をもとに、ゲームを作ったり勉強会を
					<br />
					開いたり様々なITにまつわる活動を行っています。
				</p>
				<p
					style={{
						wordWrap: "break-word",
						margin: "5vh 0",
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
			<p
				style={{ fontSize: "2vmin", textAlign: "right", marginRight: "0.25vw" }}
			>
				何かあれば proken0603@gmail.com まで
			</p>
		</footer>
	);
}

export default Footer;
