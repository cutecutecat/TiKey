CALL table_exists('new_feature','t1',@v_is_exists);

SELECT IF(@v_is_exists = '','Not exists!',@v_is_exists) AS 'result';