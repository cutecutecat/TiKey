DELIMITER // 
create
       function myfun_getMax(num1 int, num2 int)
       returns int        
begin
       declare res int;
       if(num1 > num2) then
       set res = num1;
       elseif (num1 < num2) then
       set res = num2;
       else
       set res = num1;
       end if;
       return res;
end//
DELIMITER ;