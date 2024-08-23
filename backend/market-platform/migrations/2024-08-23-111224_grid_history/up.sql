-- Your SQL goes here
Create table grid_history(
	created_at timestamptz PRIMARY KEY,
	grid_state jsonb
	);
