USE world;

SELECT 1 + 2, 10 - 11, 1 / 3, POW(2, 3), ROUND(1/3, 1), CEILING(0.9);

SELECT
  Name,
  ROUND(Population/1000000,1) AS 'Population (Millon)'
FROM city
WHERE CountryCode='MEX' AND Population>1000000;

SELECT Name FROM city WHERE LEFT(Name, 3) = 'New';

USE sakila;

SELECT 
  email,
  SUBSTRING_INDEX(email, "@", 1),
  SUBSTRING_INDEX(email, "@", -1)
FROM customer 
WHERE store_id=1 AND active=0;

SELECT LENGTH('Café'), CHAR_LENGTH('Café');

-- Expect time differences
SET TIMESTAMP=1574005513;
SELECT CURRENT_TIME(), CURRENT_DATE(), CURRENT_TIMESTAMP(), NOW();

-- Expect time differences
SET TIMESTAMP=1574005513;
SELECT CURRENT_TIME(6), CURRENT_DATE(), CURRENT_TIMESTAMP(6), NOW(6);

SELECT DATE_ADD('2010-01-01', INTERVAL 1 YEAR);

SELECT UNIX_TIMESTAMP('2030-01-01 00:00:00'), FROM_UNIXTIME(1573846979);