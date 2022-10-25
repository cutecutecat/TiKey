create fulltext index full_idx_name_author_publisher 
on book(name, author, publisher);

ALTER TABLE book 
DROP INDEX full_idx_name_author_publisher;