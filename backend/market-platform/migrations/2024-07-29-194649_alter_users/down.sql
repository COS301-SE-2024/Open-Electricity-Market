DROP TRIGGER IF EXISTS reactivate_user ON users;
DROP FUNCTION IF EXISTS set_deleted_at_null();
DROP TRIGGER IF EXISTS reactivate_node ON users;
DROP FUNCTION IF EXISTS set_nodes_active();
DROP TRIGGER IF EXISTS deactivate_node ON users;
DROP FUNCTION IF EXISTS set_nodes_inactive();
ALTER TABLE users
DROP COLUMN created_at;
