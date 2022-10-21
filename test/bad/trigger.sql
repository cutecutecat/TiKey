CREATE TRIGGER T1
AFTER 
INSERT ON customer
FOR EACH ROW 
INSERT INTO note(日期,目标,操作) VALUES(NOW(),'customer','insert');