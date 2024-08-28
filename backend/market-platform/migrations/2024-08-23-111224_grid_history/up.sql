-- Your SQL goes here
Create table grid_history(
	created_at timestamptz PRIMARY KEY DEFAULT CURRENT_TIMESTAMP,
	grid_state jsonb
	);
