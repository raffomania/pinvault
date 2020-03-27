-- Your SQL goes here
create table files (
  hash text primary key not null,
  title text not null,
  url text not null
)
