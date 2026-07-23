-- Database: user_db
DEFINE DATABASE IF NOT EXISTS user_db;

-- Table: user_db.users
USE DATABASE user_db;
DEFINE TABLE IF NOT EXISTS users SCHEMAFULL;
DEFINE FIELD IF NOT EXISTS username              ON users TYPE string;
DEFINE FIELD IF NOT EXISTS nickname              ON users TYPE option<string>;
DEFINE FIELD IF NOT EXISTS password              ON users TYPE string;
DEFINE FIELD IF NOT EXISTS captcha_verification  ON users TYPE option<string>;
DEFINE FIELD IF NOT EXISTS user_id               ON users TYPE int;
DEFINE FIELD IF NOT EXISTS access_token          ON users TYPE string;
DEFINE FIELD IF NOT EXISTS refresh_token         ON users TYPE string;
DEFINE FIELD IF NOT EXISTS expires_time          ON users TYPE datetime;
DEFINE FIELD IF NOT EXISTS password_expired      ON users TYPE option<bool>;
DEFINE FIELD IF NOT EXISTS must_change_password  ON users TYPE option<bool>;
DEFINE FIELD IF NOT EXISTS create_time           ON users TYPE datetime;
DEFINE FIELD IF NOT EXISTS update_time           ON users TYPE datetime;
