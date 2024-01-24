import { Navbar, Nav } from "react-bootstrap";

import styles from "./index.module.css";

import { joinClassnames } from "@/components/@joinClassnames";
import { URL_PATH_PARTICIPANT } from "@/consts";

type Props = {
  className?: string;
};

export const ParticipantHeader = ({ className }: Props) => {
  return (
    <>
      <Navbar.Toggle aria-controls="links" />
      <Navbar.Collapse id="links" className={joinClassnames(className)}>
        <Nav className={joinClassnames("me-auto", styles.base)}>
          <Nav.Link href={URL_PATH_PARTICIPANT.FAVORITE}>
            <span>お気に入り</span>
          </Nav.Link>
          <Nav.Link href={URL_PATH_PARTICIPANT.APPLY_LIST}>
            <span>応募したボランティア</span>
          </Nav.Link>
          <Nav.Link href={URL_PATH_PARTICIPANT.SCOUT}>
            <span>スカウト</span>
          </Nav.Link>
        </Nav>
      </Navbar.Collapse>
    </>
  );
};
