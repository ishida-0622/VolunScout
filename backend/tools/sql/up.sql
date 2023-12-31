USE `volunscout`;

CREATE TABLE IF NOT EXISTS `participant_account`
(
  `uid` CHAR(28),
  `name` VARCHAR(50) NOT NULL,
  `furigana` VARCHAR(50) NOT NULL,
  `gender` TINYINT NOT NULL DEFAULT 2,
  `birthday` DATE NOT NULL,
  `phone` VARCHAR(11) NOT NULL,
  `profile` TEXT NOT NULL,
  `is_deleted` BOOLEAN NOT NULL DEFAULT false,
  `deleted_at` DATETIME,
  PRIMARY KEY (`uid`)
);

CREATE TABLE IF NOT EXISTS `group_account`
(
  `gid` CHAR(28),
  `name` VARCHAR(50) NOT NULL,
  `furigana` VARCHAR(50) NOT NULL,
  `phone` VARCHAR(11) NOT NULL,
  `address` VARCHAR(100) NOT NULL,
  `contents` TEXT NOT NULL,
  `is_deleted` BOOLEAN NOT NULL DEFAULT false,
  `deleted_at` DATETIME,
  `is_paid` BOOLEAN NOT NULL DEFAULT false,
  PRIMARY KEY (`gid`)
);

CREATE TABLE IF NOT EXISTS `volunteer`
(
  `vid` CHAR(26),
  `gid` CHAR(28) NOT NULL,
  `title` VARCHAR(30) NOT NULL,
  `message` TEXT NOT NULL,
  `overview` TEXT NOT NULL,
  `recruited_num` INT NOT NULL,
  `place` VARCHAR(100) NOT NULL,
  `start_at` DATETIME NOT NULL,
  `finish_at` DATETIME NOT NULL,
  `as_group` BOOLEAN NOT NULL,
  `is_deleted` BOOLEAN NOT NULL DEFAULT false,
  `deleted_at` DATETIME,
  `deadline_on` DATE NOT NULL,
  `registered_at` DATETIME NOT NULL,
  `updated_at` DATETIME NOT NULL,
  PRIMARY KEY (`vid`),
  FOREIGN KEY(`gid`) REFERENCES `group_account`(`gid`)
);

CREATE TABLE IF NOT EXISTS `scout`
(
  `sid` CHAR(26),
  `vid` CHAR(26) NOT NULL,
  `uid` CHAR(28) NOT NULL,
  `message` TEXT NOT NULL,
  `is_succeed` BOOLEAN NOT NULL DEFAULT false,
  `is_read` BOOLEAN NOT NULL DEFAULT false,
  `send_at` DATETIME NOT NULL,
  `is_denied` BOOLEAN NOT NULL DEFAULT false,
  `denied_at` DATETIME,
  PRIMARY KEY (`sid`),
  FOREIGN KEY(`vid`) REFERENCES `volunteer`(`vid`),
  FOREIGN KEY(`uid`) REFERENCES `participant_account`(`uid`)
);

CREATE TABLE IF NOT EXISTS `apply`
(
  `aid` CHAR(26),
  `vid` CHAR(26) NOT NULL,
  `uid` CHAR(28) NOT NULL,
  `people_num` INT,
  `apply_at` DATETIME NOT NULL,
  `is_allowed` TINYINT NOT NULL DEFAULT 0,
  PRIMARY KEY (`aid`),
  FOREIGN KEY(`vid`) REFERENCES `volunteer`(`vid`),
  FOREIGN KEY(`uid`) REFERENCES `participant_account`(`uid`)
);

CREATE TABLE IF NOT EXISTS `favorite`
(
  `uid` CHAR(28) NOT NULL,
  `vid` CHAR(26) NOT NULL,
  `registered_at` DATETIME NOT NULL,
  PRIMARY KEY(`uid`, `vid`),
  FOREIGN KEY(`uid`) REFERENCES `participant_account`(`uid`),
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
  `eid` VARCHAR(255) NOT NULL,
  `is_need` BOOLEAN NOT NULL DEFAULT false,
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
  FOREIGN KEY(`uid`) REFERENCES `participant_account`(`uid`)
);

CREATE TABLE IF NOT EXISTS `review`
(
  `rid` CHAR(26),
  `from_id` VARCHAR(28) NOT NULL,
  `to_id` VARCHAR(28) NOT NULL,
  `user_type` TINYINT NOT NULL,
  `point` TINYINT NOT NULL,
  PRIMARY KEY (`rid`),
  FOREIGN KEY(`from_id`) REFERENCES `participant_account`(`uid`),
  FOREIGN KEY(`from_id`) REFERENCES `volunteer`(`vid`),
  FOREIGN KEY(`to_id`) REFERENCES `participant_account`(`uid`),
  FOREIGN KEY(`to_id`) REFERENCES `volunteer`(`vid`)
);

CREATE TABLE IF NOT EXISTS `group_volunteer_photo`
(
  `s3_key` VARCHAR(100),
  `gvid` VARCHAR(28) NOT NULL,
  PRIMARY KEY (`s3_key`)
)
