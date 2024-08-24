-- Your SQL goes here
CREATE TABLE users (
  id text PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  github_user VARCHAR NOT NULL
)