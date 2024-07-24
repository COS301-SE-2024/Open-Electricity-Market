use agent_manager::AgentManager;

pub mod agent_manager;

fn main() {
    let mut agent_manager = AgentManager {
        consumers: vec![],
        producers: vec![],
    };
    agent_manager.poll_database();
}
