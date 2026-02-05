//! 执行服务 - 主服务实现

use super::{
    AgentScheduler, ExecutionConfig, ExecutionContext, ExecutionResult, ExecutionStatus,
    SandboxManager,
};
use crate::intent_parser::{
    error::{ExecutionResult as IntentExecResult, IntentExecutionError},
    executor::IntentExecutor,
    security::ParsedSecurityIntent,
};
use data::models::Task;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// 执行服务
pub struct ExecutionService {
    config: ExecutionConfig,
    executor: IntentExecutor,
    sandbox_manager: Option<SandboxManager>,
    agent_scheduler: Option<AgentScheduler>,
    /// 活跃的执行上下文
    active_executions: Arc<RwLock<std::collections::HashMap<Uuid, ExecutionContext>>>,
}

impl ExecutionService {
    /// 创建新的执行服务
    pub fn new() -> Self {
        Self {
            config: ExecutionConfig::default(),
            executor: IntentExecutor::new(),
            sandbox_manager: None,
            agent_scheduler: None,
            active_executions: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// 使用配置创建
    pub fn with_config(config: ExecutionConfig) -> Self {
        Self {
            config,
            executor: IntentExecutor::new(),
            sandbox_manager: None,
            agent_scheduler: None,
            active_executions: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// 设置 sandbox 管理器
    pub fn with_sandbox_manager(mut self, manager: SandboxManager) -> Self {
        self.sandbox_manager = Some(manager);
        self
    }

    /// 设置 agent 调度器
    pub fn with_agent_scheduler(mut self, scheduler: AgentScheduler) -> Self {
        self.agent_scheduler = Some(scheduler);
        self
    }

    /// 执行解析后的意图
    pub async fn execute_intent(
        &self,
        parsed: &ParsedSecurityIntent,
        user_id: Option<i64>,
    ) -> IntentExecResult<ExecutionContext> {
        // 1. 创建任务
        let task = self.executor.create_data_task(parsed, user_id)
            .map_err(|e| IntentExecutionError::execution_failed(format!("Failed to create task: {}", e)))?;

        let execution_id = Uuid::new_v4();
        let task_id = task.id;

        // 创建执行上下文
        let mut context = ExecutionContext {
            execution_id,
            task_id,
            user_id,
            status: ExecutionStatus::Pending,
            sandbox_id: None,
            agent_id: None,
            started_at: None,
            completed_at: None,
            result: None,
        };

        // 记录活跃执行
        {
            let mut executions = self.active_executions.write().await;
            executions.insert(execution_id, context.clone());
        }

        // 2. 创建 sandbox（如果配置了）
        if self.config.auto_create_sandbox {
            context.status = ExecutionStatus::Preparing;
            self.update_execution(&context).await;

            match self.create_sandbox(parsed).await {
                Ok(sandbox_id) => {
                    context.sandbox_id = Some(sandbox_id);
                }
                Err(e) => {
                    context.status = ExecutionStatus::Failed;
                    context.result = Some(ExecutionResult {
                        success: false,
                        exit_code: None,
                        output: String::new(),
                        error_message: Some(format!("Failed to create sandbox: {}", e)),
                        findings_count: 0,
                        report_path: None,
                    });
                    self.update_execution(&context).await;
                    return Err(IntentExecutionError::sandbox(e.to_string()));
                }
            }
        }

        // 3. 分配 agent（如果配置了）
        if self.config.auto_assign_agent {
            match self.assign_agent(parsed, context.sandbox_id.as_deref()).await {
                Ok(agent_id) => {
                    context.agent_id = Some(agent_id);
                }
                Err(e) => {
                    context.status = ExecutionStatus::Failed;
                    context.result = Some(ExecutionResult {
                        success: false,
                        exit_code: None,
                        output: String::new(),
                        error_message: Some(format!("Failed to assign agent: {}", e)),
                        findings_count: 0,
                        report_path: None,
                    });
                    self.update_execution(&context).await;
                    return Err(IntentExecutionError::agent(e.to_string()));
                }
            }
        }

        // 4. 开始执行
        context.status = ExecutionStatus::Running;
        context.started_at = Some(chrono::Utc::now());
        self.update_execution(&context).await;

        // TODO: 实际的执行任务逻辑
        // 这里应该调用 agent 或 sandbox 来执行实际的安全测试

        // 模拟执行完成
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        context.status = ExecutionStatus::Completed;
        context.completed_at = Some(chrono::Utc::now());
        context.result = Some(ExecutionResult {
            success: true,
            exit_code: Some(0),
            output: "Security test completed successfully".to_string(),
            error_message: None,
            findings_count: 0,
            report_path: Some(format!("/tmp/reports/{}.json", execution_id)),
        });

        self.update_execution(&context).await;

        Ok(context)
    }

    /// 从任务和意图执行（用于已有的任务）
    pub async fn execute_task_with_intent(
        &self,
        task: &Task,
        parsed: &ParsedSecurityIntent,
    ) -> IntentExecResult<ExecutionContext> {
        let user_id = task.assignee.parse().ok();
        self.execute_intent(parsed, user_id).await
    }

    /// 获取执行上下文
    pub async fn get_execution(&self, execution_id: Uuid) -> Option<ExecutionContext> {
        let executions = self.active_executions.read().await;
        executions.get(&execution_id).cloned()
    }

    /// 获取所有活跃执行
    pub async fn list_active_executions(&self) -> Vec<ExecutionContext> {
        let executions = self.active_executions.read().await;
        executions.values().filter(|e| e.status.is_active()).cloned().collect()
    }

    /// 取消执行
    pub async fn cancel_execution(&self, execution_id: Uuid) -> IntentExecResult<()> {
        let mut executions = self.active_executions.write().await;
        
        if let Some(context) = executions.get_mut(&execution_id) {
            if context.status.is_active() {
                context.status = ExecutionStatus::Cancelled;
                context.completed_at = Some(chrono::Utc::now());
                context.result = Some(ExecutionResult {
                    success: false,
                    exit_code: None,
                    output: String::new(),
                    error_message: Some("Execution cancelled by user".to_string()),
                    findings_count: 0,
                    report_path: None,
                });
                Ok(())
            } else {
                Err(IntentExecutionError::execution_failed(
                    "Execution is not active".to_string()
                ))
            }
        } else {
            Err(IntentExecutionError::execution_failed(
                "Execution not found".to_string()
            ))
        }
    }

    /// 生成执行计划（不实际执行）
    pub fn generate_plan(
        &self,
        parsed: &ParsedSecurityIntent,
    ) -> IntentExecResult<crate::intent_parser::executor::ExecutionPlan> {
        self.executor.generate_execution_plan(parsed)
    }

    /// 更新执行上下文
    async fn update_execution(&self, context: &ExecutionContext) {
        let mut executions = self.active_executions.write().await;
        executions.insert(context.execution_id, context.clone());
    }

    /// 创建 sandbox
    async fn create_sandbox(
        &self,
        parsed: &ParsedSecurityIntent,
    ) -> anyhow::Result<String> {
        if let Some(ref manager) = self.sandbox_manager {
            manager.create_sandbox(parsed).await
        } else {
            // 如果没有配置 sandbox 管理器，返回模拟 ID
            Ok(format!("sandbox-{}-mock", Uuid::new_v4()))
        }
    }

    /// 分配 agent
    async fn assign_agent(
        &self,
        parsed: &ParsedSecurityIntent,
        sandbox_id: Option<&str>,
    ) -> anyhow::Result<String> {
        if let Some(ref scheduler) = self.agent_scheduler {
            scheduler.assign_agent(parsed, sandbox_id).await
        } else {
            // 如果没有配置 agent 调度器，返回模拟 ID
            Ok(format!("agent-{}-mock", Uuid::new_v4()))
        }
    }
}

impl Default for ExecutionService {
    fn default() -> Self {
        Self::new()
    }
}
