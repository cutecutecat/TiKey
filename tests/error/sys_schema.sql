SELECT statement, total, total_latency, rows_sent, rows_examined, 
rows_affected, full_scans FROM sys.host_summary_by_statement_type 
WHERE host='localhost' ORDER BY total DESC LIMIT 5;