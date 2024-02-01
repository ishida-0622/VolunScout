import { Navbar, Nav } from "react-bootstrap";

import styles from "./index.module.css";

import { joinClassnames } from "@/components/@joinClassnames";
import { URL_PATH_GROUP } from "@/consts";

type Props = {
  className?: string;
};

export const GroupHeader = ({ className }: Props) => {
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
            position: relative; /* 追加: 相対位置指定 */
          }

          /* 追加されたスタイル */
          .nav-link::after {
            content: '';
            position: absolute;
            bottom: 0;
            left: 0; /* 変更: 左端からスタート */
            width: 100%; /* 変更: 幅100% */
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
          <Nav.Link href={URL_PATH_GROUP.VOLUNTEER_CREATE} className="nav-link">
            <span className="text">
              <strong>ボランティア掲載</strong>
            </span>
          </Nav.Link>
          <Nav.Link href={URL_PATH_GROUP.VOLUNTEER} className="nav-link">
            <span className="text">
              <strong>掲載したボランティア</strong>
            </span>
          </Nav.Link>
        </Nav>
      </Navbar.Collapse>
    </>
  );
};
