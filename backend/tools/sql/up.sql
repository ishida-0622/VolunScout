USE `volunscout`;

CREATE TABLE IF NOT EXISTS `participant`
(
  `uid` CHAR(28),
  `name` VARCHAR(50) NOT NULL,
  `furigana` VARCHAR(50) NOT NULL,
  `gender` TINYINT NOT NULL DEFAULT 2,
  `birthday` DATE NOT NULL,
  `phone` VARCHAR(11) NOT NULL,
  `profile` TEXT NOT NULL,
  `is_delete` BOOLEAN NOT NULL DEFAULT false,
  `delete_date` DATE,
  PRIMARY KEY (`uid`)
);

CREATE TABLE IF NOT EXISTS `group`
(
  `gid` CHAR(28),
  `name` VARCHAR(50) NOT NULL,
  `furigana` VARCHAR(50) NOT NULL,
  `phone` VARCHAR(11) NOT NULL,
  `address` VARCHAR(100) NOT NULL,
  `contents` TEXT NOT NULL,
  `is_delete` BOOLEAN NOT NULL DEFAULT false,
  `delete_date` DATE,
  `is_paid` BOOLEAN NOT NULL DEFAULT false,
  PRIMARY KEY (`gid`)
);

CREATE TABLE IF NOT EXISTS `volunteer`
(
  `vid` CHAR(26),
  `gid` CHAR(28) NOT NULL,
  `overview` TEXT NOT NULL,
  `people_num` INT NOT NULL,
  `start_datetime` DATETIME NOT NULL,
  `start_day` TINYINT NOT NULL,
  `finish_datetime` DATETIME NOT NULL,
  `finish_day` TINYINT NOT NULL,
  `as_group` BOOLEAN NOT NULL,
  `is_delete` BOOLEAN NOT NULL DEFAULT false,
  PRIMARY KEY (`vid`),
  FOREIGN KEY(`gid`) REFERENCES `group`(`gid`)
);

CREATE TABLE IF NOT EXISTS `scout`
(
  `sid` CHAR(26),
  `vid` CHAR(26) NOT NULL,
  `uid` CHAR(28) NOT NULL,
  `message` TEXT NOT NULL,
  `is_succeed` BOOLEAN NOT NULL DEFAULT false,
  `is_read` BOOLEAN NOT NULL DEFAULT false,
  `send_date` DATE NOT NULL,
  `is_denied` BOOLEAN NOT NULL DEFAULT false,
  `denied_date` DATE,
  PRIMARY KEY (`sid`),
  FOREIGN KEY(`vid`) REFERENCES `volunteer`(`vid`),
  FOREIGN KEY(`uid`) REFERENCES `participant`(`uid`)
);

CREATE TABLE IF NOT EXISTS `apply`
(
  `aid` CHAR(26),
  `vid` CHAR(26) NOT NULL,
  `uid` CHAR(28) NOT NULL,
  `people_num` INT,
  `apply_date` DATE NOT NULL,
  `allowed` TINYINT NOT NULL DEFAULT 0,
  PRIMARY KEY (`aid`),
  FOREIGN KEY(`vid`) REFERENCES `volunteer`(`vid`),
  FOREIGN KEY(`uid`) REFERENCES `participant`(`uid`)
);

CREATE TABLE IF NOT EXISTS `favorite`
(
  `uid` CHAR(28) NOT NULL,
  `vid` CHAR(26) NOT NULL,
  `fav_date` DATE NOT NULL,
  PRIMARY KEY(`uid`, `vid`),
  FOREIGN KEY(`uid`) REFERENCES `participant`(`uid`),
  FOREIGN KEY(`vid`) REFERENCES `volunteer`(`vid`)
);

CREATE TABLE IF NOT EXISTS `participant_region`
(
  `uid` CHAR(28) NOT NULL,
  `rid` TINYINT NOT NULL,
  PRIMARY KEY (`uid`, `rid`)
);

CREATE TABLE IF NOT EXISTS `volunteer_region`
(
  `vid` CHAR(26) NOT NULL,
  `rid` TINYINT NOT NULL,
  PRIMARY KEY (`vid`, `rid`),
  FOREIGN KEY(`vid`) REFERENCES `volunteer`(`vid`)
);

CREATE TABLE IF NOT EXISTS `participant_element`
(
  `uid` CHAR(28) NOT NULL,
  `eid` TINYINT NOT NULL,
  PRIMARY KEY (`uid`, `eid`)
);

CREATE TABLE IF NOT EXISTS `volunteer_element`
(
  `vid` CHAR(28) NOT NULL,
  `eid` TINYINT NOT NULL,
  PRIMARY KEY (`vid`, `eid`)
);

CREATE TABLE IF NOT EXISTS `volunteer_dates`
(
  `did` CHAR(26),
  `uid` CHAR(28) NOT NULL,
  `dates` CHAR(3) NOT NULL,
  PRIMARY KEY (`did`),
  FOREIGN KEY(`uid`) REFERENCES `participant`(`uid`)
);

CREATE TABLE IF NOT EXISTS `review`
(
  `rid` CHAR(26),
  `from_id` VARCHAR(28) NOT NULL,
  `to_id` VARCHAR(28) NOT NULL,
  `user_type` TINYINT NOT NULL,
  `point` TINYINT NOT NULL,
  PRIMARY KEY (`rid`),
  FOREIGN KEY(`from_id`) REFERENCES `participant`(`uid`),
  FOREIGN KEY(`from_id`) REFERENCES `volunteer`(`vid`),
  FOREIGN KEY(`to_id`) REFERENCES `participant`(`uid`),
  FOREIGN KEY(`to_id`) REFERENCES `volunteer`(`vid`)
);
