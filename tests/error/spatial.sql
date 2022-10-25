CREATE TABLE `z_gis` (
  `id` varchar(45) NOT NULL,
  `name` varchar(10) NOT NULL COMMENT '姓名',
  `gis` geometry NOT NULL COMMENT '空间位置信息'
);