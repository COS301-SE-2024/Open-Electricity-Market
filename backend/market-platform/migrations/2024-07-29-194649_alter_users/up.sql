ALTER TABLE users
ADD COLUMN created_at timestamptz NOT NULL DEFAULT now();

CREATE OR REPLACE FUNCTION set_nodes_inactive()
    RETURNS TRIGGER
AS $$
BEGIN
    IF (NEW.active = FALSE) THEN
        UPDATE nodes
        SET node_active = FALSE
        WHERE node_owner = NEW.user_id;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER deactivate_node
    AFTER UPDATE
    ON users
    FOR EACH ROW
EXECUTE FUNCTION set_nodes_inactive();

CREATE OR REPLACE FUNCTION set_nodes_active()
    RETURNS TRIGGER
AS $$
    BEGIN
        IF (OLD.active = FALSE) THEN
            UPDATE nodes
            SET node_active = TRUE
            WHERE node_owner = OLD.user_id;
        END IF;
        RETURN NEW;
    END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER reactivate_node
    AFTER UPDATE
    ON users
    FOR EACH ROW
EXECUTE FUNCTION set_nodes_active();
