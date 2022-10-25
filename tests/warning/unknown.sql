CREATE TABLE `lookups` (
  `Key` varchar(50) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL,
  `Value` varchar(200) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL,
  `Descriptions` varchar(100) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL,
  PRIMARY KEY (`Key`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;