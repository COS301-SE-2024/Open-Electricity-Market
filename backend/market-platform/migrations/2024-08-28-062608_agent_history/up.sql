-- Your SQL goes here
Create table agent_history(
	created_at timestamptz PRIMARY KEY DEFAULT CURRENT_TIMESTAMP,
	agent_state jsonb
	);
