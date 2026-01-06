use core::task::Task;
use anyhow::Result;
use tokio::sync::mpsc;

pub struct TaskExecutor {
    tx: mpsc::Sender<TaskResult>,
    rx: mpsc::Receiver<TaskResult>,
}

#[derive(Debug)]
pub struct TaskResult {
    pub task_name: String,
    pub success: bool,
    pub output: String,
}

impl TaskExecutor {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);
        Self { tx, rx }
    }

    pub async fn execute(&self, task: Task) -> Result<TaskResult> {
        tracing::info!("Executing task: {}", task.name);
        
        // Simulate task execution
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
        Ok(TaskResult {
            task_name: task.name.clone(),
            success: true,
            output: format!("Task {} completed successfully", task.name),
        })
    }

    pub async fn receive_result(&mut self) -> Option<TaskResult> {
        self.rx.recv().await
    }
}

impl Default for TaskExecutor {
    fn default() -> Self {
        Self::new()
    }
}
