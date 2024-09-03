-- Your SQL goes here
CREATE TABLE grid_history(
	created_at timestamptz PRIMARY KEY DEFAULT CURRENT_TIMESTAMP,
	grid_state jsonb
);

SELECT create_hypertable('grid_history', by_range('created_at'));
