\set pass `echo "$OPEN_EM_PASS"`

CREATE ROLE open_em WITH CREATEDB CREATEROLE LOGIN PASSWORD :'pass';

CREATE DATABASE open_em WITH ENCODING='UTF8' OWNER=open_em CONNECTION LIMIT=-1;

GRANT ALL PRIVILEGES ON DATABASE open_em to open_em;
GRANT ALL ON LANGUAGE plpgsql TO open_em WITH GRANT OPTION;

\c open_em open_em
CREATE SCHEMA AUTHORIZATION open_em;
CREATE EXTENSION "uuid-ossp" WITH SCHEMA open_em;
GRANT ALL PRIVILEGES ON SCHEMA open_em to open_em;

ALTER ROLE open_em IN DATABASE open_em SET search_path=open_em, public;

\c open_em postgres
DROP EXTENSION timescaledb;
CREATE EXTENSION IF NOT EXISTS timescaledb SCHEMA open_em CASCADE;
DROP EXTENSION timescaledb_toolkit;
CREATE EXTENSION IF NOT EXISTS timescaledb_toolkit SCHEMA open_em CASCADE;