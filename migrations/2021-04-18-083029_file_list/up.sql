-- Your SQL goes here
CREATE TABLE file_list (
  id SERIAL PRIMARY KEY NOT NULL,
  file_name VARCHAR NOT NULL,
  md5 TEXT NOT NULL,
  commit_message TEXT NOT NULL,
  version_number INTEGER NOT NULL,
  user_uploader VARCHAR NOT NULL,
  upload_time timestamp DEFAULT CURRENT_TIMESTAMP NOT NULL
);