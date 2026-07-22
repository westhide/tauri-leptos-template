-- USER DB
DEFINE DATABASE IF NOT EXISTS user_db;

DEFINE TABLE IF NOT EXISTS users SCHEMAFULL;
DEFINE FIELD username              ON users TYPE string;
DEFINE FIELD nickname              ON users TYPE string;
DEFINE FIELD password              ON users TYPE string;
DEFINE FIELD captcha_verification  ON users TYPE option<string>;
DEFINE FIELD user_id               ON users TYPE int;
DEFINE FIELD access_token          ON users TYPE string;
DEFINE FIELD refresh_token         ON users TYPE string;
DEFINE FIELD expires_time          ON users TYPE datetime;
DEFINE FIELD password_expired      ON users TYPE option<bool>;
DEFINE FIELD must_change_password  ON users TYPE option<bool>;
DEFINE FIELD create_time           ON users TYPE datetime;
DEFINE FIELD update_time           ON users TYPE datetime;
