//! Sandbox 管理器 - 管理安全测试的 sandbox 环境

use crate::intent_parser::security::ParsedSecurityIntent;


/// Sandbox 管理器
pub struct SandboxManager {
    /// Sandbox 后端类型
    backend: SandboxBackend,
    /// 默认配置
    default_config: SandboxConfig,
}

/// Sandbox 后端类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxBackend {
    /// BoxLite (microVM)
    BoxLite,
    /// Docker 容器
    Docker,
    /// 本地进程（仅用于开发）
    Process,
}

/// Sandbox 配置
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    /// 后端类型
    pub backend: SandboxBackend,
    /// 网络模式
    pub network_mode: NetworkMode,
    /// 内存限制（MB）
    pub memory_limit_mb: u64,
    /// CPU 限制（核心数）
    pub cpu_limit: f64,
    /// 超时时间（秒）
    pub timeout_seconds: u64,
    /// 环境变量
    pub environment: std::collections::HashMap<String, String>,
    /// 卷挂载
    pub volumes: Vec<VolumeMount>,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            backend: SandboxBackend::Docker,
            network_mode: NetworkMode::Bridge,
            memory_limit_mb: 2048,
            cpu_limit: 1.0,
            timeout_seconds: 3600,
            environment: std::collections::HashMap::new(),
            volumes: vec![VolumeMount {
                source: "/tmp/scans".to_string(),
                target: "/output".to_string(),
                read_only: false,
            }],
        }
    }
}

/// 网络模式
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkMode {
    /// 桥接模式
    Bridge,
    /// 主机模式
    Host,
    /// 无网络
    None,
    /// 自定义网络
    Custom(String),
}

impl NetworkMode {
    pub fn as_str(&self) -> &str {
        match self {
            NetworkMode::Bridge => "bridge",
            NetworkMode::Host => "host",
            NetworkMode::None => "none",
            NetworkMode::Custom(name) => name.as_str(),
        }
    }
}

/// 卷挂载
#[derive(Debug, Clone)]
pub struct VolumeMount {
    /// 宿主机路径
    pub source: String,
    /// 容器内路径
    pub target: String,
    /// 是否只读
    pub read_only: bool,
}

/// Sandbox 实例信息
#[derive(Debug, Clone)]
pub struct SandboxInstance {
    /// Sandbox ID
    pub id: String,
    /// 状态
    pub status: SandboxStatus,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 配置
    pub config: SandboxConfig,
}

/// Sandbox 状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxStatus {
    Creating,
    Running,
    Stopped,
    Error,
}

impl SandboxManager {
    /// 创建新的 sandbox 管理器
    pub fn new() -> Self {
        Self {
            backend: SandboxBackend::Docker,
            default_config: SandboxConfig::default(),
        }
    }

    /// 使用特定后端创建
    pub fn with_backend(backend: SandboxBackend) -> Self {
        Self {
            backend,
            default_config: SandboxConfig {
                backend,
                ..Default::default()
            },
        }
    }

    /// 创建 sandbox
    pub async fn create_sandbox(
        &self,
        parsed: &ParsedSecurityIntent,
    ) -> anyhow::Result<String> {
        let config = self.build_config(parsed);
        
        // 根据后端类型创建 sandbox
        match self.backend {
            SandboxBackend::BoxLite => {
                self.create_boxlite_sandbox(&config).await
            }
            SandboxBackend::Docker => {
                self.create_docker_sandbox(parsed, &config).await
            }
            SandboxBackend::Process => {
                self.create_process_sandbox(&config).await
            }
        }
    }

    /// 停止 sandbox
    pub async fn stop_sandbox(&self, sandbox_id: &str) -> anyhow::Result<()> {
        tracing::info!("Stopping sandbox: {}", sandbox_id);
        // TODO: 实现实际的停止逻辑
        Ok(())
    }

    /// 删除 sandbox
    pub async fn delete_sandbox(&self, sandbox_id: &str) -> anyhow::Result<()> {
        tracing::info!("Deleting sandbox: {}", sandbox_id);
        // TODO: 实现实际的删除逻辑
        Ok(())
    }

    /// 获取 sandbox 状态
    pub async fn get_sandbox_status(&self, sandbox_id: &str) -> anyhow::Result<SandboxStatus> {
        tracing::debug!("Getting sandbox status: {}", sandbox_id);
        // TODO: 实现实际的状态查询
        Ok(SandboxStatus::Running)
    }

    /// 为意图构建配置
    fn build_config(&self, parsed: &ParsedSecurityIntent) -> SandboxConfig {
        let mut config = self.default_config.clone();

        // 根据扫描强度调整资源
        let intent = &parsed.security_intent;
        match intent.scan_config.intensity {
            crate::intent_parser::ScanIntensity::Light => {
                config.memory_limit_mb = 512;
                config.cpu_limit = 0.5;
            }
            crate::intent_parser::ScanIntensity::Aggressive => {
                config.memory_limit_mb = 4096;
                config.cpu_limit = 2.0;
            }
            _ => {}
        }

        // 设置超时
        config.timeout_seconds = intent.scan_config.timeout_seconds;

        // 设置环境变量
        config.environment.insert(
            "SCAN_INTENSITY".to_string(),
            intent.scan_config.intensity.as_str().to_string(),
        );
        config.environment.insert(
            "DEEP_SCAN".to_string(),
            intent.scan_config.deep_scan.to_string(),
        );
        config.environment.insert(
            "THREADS".to_string(),
            intent.scan_config.threads.to_string(),
        );

        config
    }

    /// 创建 BoxLite sandbox
    async fn create_boxlite_sandbox(&self, config: &SandboxConfig) -> anyhow::Result<String> {
        let sandbox_id = format!("boxlite-{}", uuid::Uuid::new_v4());
        
        tracing::info!(
            "Creating BoxLite sandbox: {} with memory: {}MB, CPU: {}",
            sandbox_id,
            config.memory_limit_mb,
            config.cpu_limit
        );

        // TODO: 集成实际的 BoxLite 创建逻辑
        // 这里应该调用 sandbox crate 的 API

        Ok(sandbox_id)
    }

    /// 创建 Docker sandbox
    async fn create_docker_sandbox(
        &self,
        parsed: &ParsedSecurityIntent,
        config: &SandboxConfig,
    ) -> anyhow::Result<String> {
        let sandbox_id = format!("docker-{}", uuid::Uuid::new_v4());
        
        // 选择镜像
        let image = self.select_image(parsed);
        
        tracing::info!(
            "Creating Docker sandbox: {} with image: {}, memory: {}MB",
            sandbox_id,
            image,
            config.memory_limit_mb
        );

        // TODO: 集成实际的 Docker 创建逻辑
        // 这里应该调用 sandbox 或 agent crate 的 Docker API

        Ok(sandbox_id)
    }

    /// 创建进程 sandbox
    async fn create_process_sandbox(&self, config: &SandboxConfig) -> anyhow::Result<String> {
        let sandbox_id = format!("process-{}", uuid::Uuid::new_v4());
        
        tracing::info!(
            "Creating process sandbox: {} with timeout: {}s",
            sandbox_id,
            config.timeout_seconds
        );

        // TODO: 集成实际的进程隔离逻辑

        Ok(sandbox_id)
    }

    /// 选择合适的镜像
    fn select_image(&self, parsed: &ParsedSecurityIntent) -> String {
        use crate::intent_parser::SecurityTestType;
        
        let intent = &parsed.security_intent;
        match intent.test_type {
            SecurityTestType::NetworkScan | SecurityTestType::PortScan => {
                "uavred/agent:nmap".to_string()
            }
            SecurityTestType::VulnerabilityScan => "uavred/agent:openvas".to_string(),
            SecurityTestType::WebAppTest => "uavred/agent:burp".to_string(),
            SecurityTestType::ProtocolAnalysis => "uavred/agent:wireshark".to_string(),
            SecurityTestType::FirmwareAnalysis => "uavred/agent:binwalk".to_string(),
            SecurityTestType::ApiTest => "uavred/agent:postman".to_string(),
            _ => "uavred/agent:latest".to_string(),
        }
    }
}

impl Default for SandboxManager {
    fn default() -> Self {
        Self::new()
    }
}
