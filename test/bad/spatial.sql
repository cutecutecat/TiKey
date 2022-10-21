CREATE TABLE `z_gis` (
  `id` varchar(45) NOT NULL,
  `name` varchar(10) NOT NULL COMMENT '姓名',
  `gis` geometry NOT NULL COMMENT '空间位置信息',
  `geohash` varchar(20) GENERATED ALWAYS AS (st_geohash(`gis`,8)) VIRTUAL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `id` (`id`),
  SPATIAL KEY `idx_gis` (`gis`),
  KEY `idx_geohash` (`geohash`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='空间位置信息';

insert into z_gis(id,name,gis) values
(replace(uuid(),'-',''),'张三',ST_GeometryFromText('point(108.9498710632 34.2588125935)')),
(replace(uuid(),'-',''),'李四',ST_GeometryFromText('point(108.9465236664 34.2598766768)')),
(replace(uuid(),'-',''),'王五',ST_GeometryFromText('point(108.9477252960 34.2590342786)')),
(replace(uuid(),'-',''),'赵六',ST_GeometryFromText('point(108.9437770844 34.2553719653)')),
(replace(uuid(),'-',''),'小七',ST_GeometryFromText('point(108.9443349838 34.2595663206)')),
(replace(uuid(),'-',''),'孙八',ST_GeometryFromText('point(108.9473497868 34.2643456798)')),
(replace(uuid(),'-',''),'十九',ST_GeometryFromText('point(108.9530360699 34.2599476152)'));

select name, astext(gis) gis from z_gis where name = '张三';
update z_gis set gis = geomfromtext('point(108.9465236664 34.2598766768)') where name = '张三';

select floor(st_distance_sphere(
    (select gis from z_gis where name= '张三'),
    gis
)) distance from z_gis where name= '李四';