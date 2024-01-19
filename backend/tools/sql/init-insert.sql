INSERT INTO participant_account VALUES (
  "participant_account000000000",
  "ken",
  "ken",
  0,
  "2002-06-22",
  "08012345678",
  "kendesu",
  DEFAULT,
  NULL
);

INSERT INTO participant_account VALUES (
  "participant_account000000001",
  "taro",
  "taro",
  0,
  "2002-10-11",
  "08012341234",
  "yorodesu",
  DEFAULT,
  NULL
);

INSERT INTO group_account VALUES (
  "group_account000000000000000",
  "VolunScout",
  "boran sukauto",
  "08012345678",
  "Tokyo",
  "Hello",
  "katogi",
  "katogi",
  DEFAULT,
  DEFAULT,
  NULL
);

INSERT INTO volunteer(
  vid,
  gid,
  title,
  message,
  overview,
  recruited_num,
  place,
  start_at,
  finish_at,
  as_group,
  reward,
  deadline_on,
  registered_at,
  updated_at
) VALUES (
  "01HKXVVVKBR6G8240N7HWSPR7M",
  "group_account000000000000000",
  "Title",
  "Message",
  "Overview",
  10,
  "Tokyo Sky Tree Soramachi 1F",
  "2023-12-24 11:00:00",
  "2023-12-24 17:00:00",
  true,
  "1000 yen as transportation expenses",
  "2023-12-10",
  NOW(),
  NOW()
);

INSERT INTO volunteer(
  vid,
  gid,
  title,
  message,
  overview,
  recruited_num,
  place,
  start_at,
  finish_at,
  as_group,
  deadline_on,
  registered_at,
  updated_at
) VALUES (
  "01HKXZRM5AF35HHXJ8284PN0B7",
  "group_account000000000000000",
  "Title",
  "Message",
  "Overview",
  10,
  "front of Tokyo Station",
  "2023-12-24 11:00:00",
  "2023-12-24 17:00:00",
  false,
  "2023-12-10",
  NOW(),
  NOW()
);

INSERT INTO volunteer_region VALUES (
  "01HKXVVVKBR6G8240N7HWSPR7M",
  1
);

INSERT INTO volunteer_region VALUES (
  "01HKXVVVKBR6G8240N7HWSPR7M",
  2
);

INSERT INTO volunteer_region VALUES (
  "01HKXZRM5AF35HHXJ8284PN0B7",
  3
);

INSERT INTO volunteer_region VALUES (
  "01HKXZRM5AF35HHXJ8284PN0B7",
  4
);

INSERT INTO favorite VALUES (
  "participant_account000000000",
  "01HKXVVVKBR6G8240N7HWSPR7M",
  NOW()
);

INSERT INTO favorite VALUES (
  "participant_account000000000",
  "01HKXZRM5AF35HHXJ8284PN0B7",
  NOW()
);

INSERT INTO apply VALUES (
  "01HKXZS5TT1GMJD8PVC5RKJH42",
  "01HKXVVVKBR6G8240N7HWSPR7M",
  "participant_account000000000",
  "2024-1-12 12:00:00",
  true,
  1,
  "2024-1-12 16:20:00",
  true
);

INSERT INTO apply VALUES (
  "01HKXZSD39EDT7F528X87EPHX9",
  "01HKXVVVKBR6G8240N7HWSPR7M",
  "participant_account000000001",
  "2024-1-13 12:00:00",
  false,
  0,
  NULL,
  false
);

INSERT INTO group_participants VALUES (
  "01HKXZS5TT1GMJD8PVC5RKJH42",
  1,
  "first taro",
  "first furigana",
  0,
  21
);

INSERT INTO group_participants VALUES (
  "01HKXZS5TT1GMJD8PVC5RKJH42",
  2,
  "second hanako",
  "second furigana",
  1,
  19
);

INSERT INTO participant_region VALUES ("participant_account000000000", 1);
INSERT INTO participant_region VALUES ("participant_account000000000", 10);
INSERT INTO participant_region VALUES ("participant_account000000000", 15);

INSERT INTO participant_element VALUES("participant_account000000000", "theme_1", false);
INSERT INTO participant_element VALUES("participant_account000000000", "theme_2", true);

INSERT INTO participant_element VALUES("participant_account000000000", "condition_1", false);
INSERT INTO participant_element VALUES("participant_account000000000", "condition_2", true);

INSERT INTO scout VALUES(
  "01HMAWQDS2Y50QHNGB8GSWJQNW",
  "01HKXVVVKBR6G8240N7HWSPR7M",
  "participant_account000000000",
  "message",
  "2023-01-17 12:00:00",
  DEFAULT,
  DEFAULT,
  NULL,
  DEFAULT,
  NULL
);

INSERT INTO scout VALUES(
  "01HMAX08V2DSBK7KVPK92D6MSD",
  "01HKXVVVKBR6G8240N7HWSPR7M",
  "participant_account000000001",
  "message",
  "2023-01-16 16:00:00",
  true,
  true,
  "2023-01-16 17:00:01",
  true,
  "2023-01-17 09:00:00"
);

INSERT INTO volunteer_review VALUES (
  "participant_account000000000",
  "01HKXVVVKBR6G8240N7HWSPR7M",
  4,
  NULL
);

INSERT INTO volunteer_review VALUES (
  "participant_account000000001",
  "01HKXVVVKBR6G8240N7HWSPR7M",
  5,
  "verrrrrrrry goooooooooooooooooood"
);

INSERT INTO participant_review VALUES (
  "01HKXVVVKBR6G8240N7HWSPR7M",
  "participant_account000000000",
  4,
  "normal"
);

INSERT INTO participant_review VALUES (
  "01HKXVVVKBR6G8240N7HWSPR7M",
  "participant_account000000001",
  5,
  NULL
);
