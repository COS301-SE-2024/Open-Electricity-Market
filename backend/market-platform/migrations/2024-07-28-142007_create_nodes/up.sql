CREATE TABLE nodes(
    node_id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    node_owner uuid NOT NULL,
    location_x float8, -- Potentially replace with location type
    location_y float8,
    units_consumed float8 NOT NULL DEFAULT 0,
    units_generated float8 NOT NULL DEFAULT 0,
    CONSTRAINT fk_node_owner
        FOREIGN KEY(node_owner)
        REFERENCES users(user_id)
);
