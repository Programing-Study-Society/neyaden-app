import { Typography, Modal, Box } from "@mui/material";
import ErrorIcon from "@mui/icons-material/Error";
import DelayQRCode from "../../public/delay_QRCode.png";

const style = {
  position: "absolute" as "absolute",
  top: "50%",
  left: "50%",
  transform: "translate(-50%, -50%)",
  width: 400,
  height: 480,
  display: "flex",
  alignItems: "center",
  justifyContent: "center", // 水平方向に中央揃え
  bgcolor: "background.paper",
  border: "2px solid #000",
  boxShadow: 24,
  p: 4,
};

interface Props {
  isOpen: boolean;
  handleClose: () => void;
}

// 京阪本線が大幅に遅延している際に表示するモーダル
function StopRunningModal({ isOpen, handleClose }: Props) {
  return (
    <Modal
      open={isOpen}
      onClose={handleClose}
      aria-labelledby="modal-modal-title"
      aria-describedby="modal-modal-description"
    >
      <Box sx={style}>
        <div>
          <ErrorIcon
            style={{
              fontSize: "100px",
              display: "block",
              margin: "0 auto",
              transform: "translateY(15%)",
            }}
            className="red"
          />

          <Typography
            style={{
              fontFamily: "notoSansJP-Regular",
              display: "block",
              fontSize: "1.5em",
              fontWeight: "bold",
            }}
            className="black"
            id="modal-modal-title"
            variant="h6"
            component="h2"
          >
            <p>
              現在、京阪本線で遅延が発生しています。詳しく公式ページを確認してください。
            </p>
            <img
              src={DelayQRCode}
              style={{
                display: "block",
                margin: "auto",
                transform: "translateY(-5%)",
              }}
            />
          </Typography>
        </div>
      </Box>
    </Modal>
  );
}

export default StopRunningModal;
