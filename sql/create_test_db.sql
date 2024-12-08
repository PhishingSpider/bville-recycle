CREATE DATABASE IF NOT EXISTS test_bville_recycle;
DROP USER IF EXISTS 'user'@'localhost';
CREATE USER 'user'@'localhost';
SET PASSWORD FOR 'user'@'localhost' = PASSWORD('password');
GRANT ALL PRIVILEGES ON bville_recycle.* TO 'user'@'localhost';
FLUSH PRIVILEGES;