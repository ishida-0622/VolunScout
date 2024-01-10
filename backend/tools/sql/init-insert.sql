INSERT INTO participant_account VALUES (
  "participant_account000000000",
  "ken",
  "ken",
  0,
  "2002-06-22",
  "08012345678",
  "ken",
  DEFAULT,
  NULL
);

INSERT INTO group_account VALUES (
  "group_account000000000000000",
  "ken",
  "ken",
  "08012345678",
  "Tokyo",
  "Hello",
  DEFAULT,
  NULL,
  DEFAULT
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
  "volunteer00000000000000000",
  "group_account000000000000000",
  "Title",
  "Message",
  "Overview",
  10,
  "Tokyo Sky Tree Soramachi 1F",
  "2023-12-24 11:00:00",
  "2023-12-24 17:00:00",
  false,
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
  "volunteer11111111111111111",
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
  "volunteer00000000000000000",
  1
);

INSERT INTO volunteer_region VALUES (
  "volunteer00000000000000000",
  2
);

INSERT INTO volunteer_region VALUES (
  "volunteer11111111111111111",
  3
);

INSERT INTO volunteer_region VALUES (
  "volunteer11111111111111111",
  4
);

INSERT INTO favorite VALUES (
  "participant_account000000000",
  "volunteer00000000000000000",
  NOW()
);

INSERT INTO favorite VALUES (
  "participant_account000000000",
  "volunteer11111111111111111",
  NOW()
);
