use crate::{Agent, AgentStatus};
use core::task::Task;
use std::collections::HashMap;
use uuid::Uuid;
use anyhow::Result;

pub struct AgentScheduler {
    agents: HashMap<Uuid, Agent>,
    task_queue: Vec<Task>,
}

impl AgentScheduler {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
            task_queue: Vec::new(),
        }
    }

    pub fn register_agent(&mut self, agent: Agent) {
        tracing::info!("Registering agent: {} ({})", agent.name, agent.id);
        self.agents.insert(agent.id, agent);
    }

    pub fn schedule_task(&mut self, task: Task) -> Result<()> {
        tracing::info!("Scheduling task: {}", task.name);
        self.task_queue.push(task);
        Ok(())
    }

    pub fn assign_tasks(&mut self) -> Result<()> {
        // Find available agents and assign tasks
        for agent in self.agents.values_mut() {
            if agent.status == AgentStatus::Idle && !self.task_queue.is_empty() {
                let task = self.task_queue.remove(0);
                tracing::info!("Assigning task {} to agent {}", task.name, agent.name);
                agent.status = AgentStatus::Running;
                // TODO: Actually execute the task
            }
        }
        Ok(())
    }

    pub fn get_active_agents(&self) -> Vec<&Agent> {
        self.agents
            .values()
            .filter(|a| a.status == AgentStatus::Running)
            .collect()
    }
}

impl Default for AgentScheduler {
    fn default() -> Self {
        Self::new()
    }
}
