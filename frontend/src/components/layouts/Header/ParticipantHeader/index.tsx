import { Nav, Navbar } from "react-bootstrap";

import styles from "./index.module.css";

import { joinClassnames } from "@/components/@joinClassnames";
import { URL_PATH_PARTICIPANT } from "@/consts";

type Props = {
  className?: string;
};

export const ParticipantHeader = ({ className }: Props) => {
  return (
    <>
      <style>
        {`
          /* マウスカーソルを合わせた時のスタイルを変更 */
          .base .nav-link:hover,
          .base .nav-link:hover span,
          .text:hover {
            color: white !important; /* 白色に変更 */
          }

          /* 追加されたスタイル */
          .text:hover {
            color: red;
          }

          /* 追加されたスタイル */
          .nav-link {
            margin-right: 15px; /* リンク間の右マージンを追加 */
            position: relative; /* 追加: 線の基点を指定 */
            font-weight: bold; /* 太文字に変更 */
          }

          /* 追加: ナビゲーションリンクに対する左の余白を調整 */
          .text {
            margin: 0 25px;
          }

          /* 追加されたスタイル */
          .nav-link::after {
            content: '';
            position: absolute;
            bottom: 0;
            left: 10%;
            width: 80%;
            height: 2px;
            background: white;
            transition: all 0.3s;
            transform: scale(0, 1);
            transform-origin: center top;
          }

          /* 追加されたスタイル */
          .nav-link:hover::after {
            transform: scale(1, 1);
          }

          /* 追加: 特定のクラスを持つ要素に対するスタイル */
          .nav-link.no-underline::after {
            display: none;
          }
        `}
      </style>
      <Navbar.Toggle aria-controls="links" />
      <Navbar.Collapse id="links" className={joinClassnames(className)}>
        <Nav className={joinClassnames("me-auto", styles.base)}>
          <Nav.Link href={URL_PATH_PARTICIPANT.FAVORITE} className="text">
            <span>お気に入り</span>
          </Nav.Link>
          <Nav.Link href={URL_PATH_PARTICIPANT.APPLY_LIST} className="text">
            <span>応募したボランティア</span>
          </Nav.Link>
          <Nav.Link href={URL_PATH_PARTICIPANT.SCOUT} className="text">
            <span>スカウト</span>
          </Nav.Link>
        </Nav>
      </Navbar.Collapse>
    </>
  );
};
