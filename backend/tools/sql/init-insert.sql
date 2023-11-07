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

INSERT INTO volunteer VALUES (
  "volunteer00000000000000000",
  "group_account000000000000000",
  "Hello",
  10,
  NOW(),
  1,
  NOW(),
  1,
  false,
  DEFAULT,
  NULL,
  NOW()
);

INSERT INTO volunteer VALUES (
  "volunteer11111111111111111",
  "group_account000000000000000",
  "World",
  10,
  NOW(),
  0,
  NOW(),
  0,
  false,
  DEFAULT,
  NULL,
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
