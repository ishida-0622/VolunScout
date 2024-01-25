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
      <Navbar.Toggle aria-controls="links" />
      <Navbar.Collapse id="links" className={joinClassnames(className)}>
        <Nav className={joinClassnames(styles.base, className)}>
          <Nav.Link href={URL_PATH_GROUP.VOLUNTEER_CREATE}>
            <span>ボランティア掲載</span>
          </Nav.Link>
          <Nav.Link href={URL_PATH_GROUP.VOLUNTEER}>
            <span>掲載したボランティア</span>
          </Nav.Link>
        </Nav>
      </Navbar.Collapse>
    </>
  );
};
