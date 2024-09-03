-- Your SQL goes here
CREATE TABLE agent_history(
	created_at timestamptz PRIMARY KEY DEFAULT CURRENT_TIMESTAMP,
	agent_state jsonb
);

SELECT create_hypertable('agent_history', by_range('created_at'));