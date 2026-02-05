//! Agent Execution Panel
//!
//! 主面板组件，显示 AI Agent 执行过程和结果

use super::model::*;
use gpui::*;
use gpui::prelude::FluentBuilder as _;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    h_flex,
    label::Label,
    v_flex, IconName, Icon, Sizable, StyledExt,
};

/// Agent 执行面板
pub struct AgentExecutionPanel {
    /// 当前会话
    session: AgentExecutionSession,
    /// 是否自动滚动到底部
    auto_scroll: bool,
    /// 滚动句柄
    scroll_handle: ScrollHandle,
    /// 订阅
    _subscriptions: Vec<Subscription>,
}

impl AgentExecutionPanel {
    /// 创建新面板
    pub fn new(window: &mut Window, cx: &mut Context<Self>, session: AgentExecutionSession) -> Self {
        let scroll_handle = ScrollHandle::new();
        
        Self {
            session,
            auto_scroll: true,
            scroll_handle,
            _subscriptions: Vec::new(),
        }
    }

    /// 从任务数据创建面板
    pub fn from_task(
        window: &mut Window,
        cx: &mut Context<Self>,
        task_id: u64,
        task_title: impl Into<String>,
    ) -> Self {
        let objective = MissionObjective::new(task_title, task_id)
            .with_description("分析目标无人机通信接口的注入漏洞")
            .with_description("重点关注遗留 PHP 端点");
        
        let session = AgentExecutionSession::new("PENLIGENT AGENT", objective);
        Self::new(window, cx, session)
    }

    /// 添加消息
    pub fn add_message(&mut self, message: AgentMessage, cx: &mut Context<Self>) {
        self.session.add_message(message);
        if self.auto_scroll {
            self.scroll_to_bottom(cx);
        }
        cx.notify();
    }

    /// 开始执行
    pub fn start_execution(&mut self, cx: &mut Context<Self>) {
        self.session.start();
        cx.notify();
    }

    /// 完成执行
    pub fn complete_execution(&mut self, cx: &mut Context<Self>) {
        self.session.complete();
        cx.notify();
    }

    /// 设置实时追踪
    pub fn set_live_trace(&mut self, enabled: bool, cx: &mut Context<Self>) {
        self.session.live_trace = enabled;
        cx.notify();
    }

    /// 滚动到底部
    fn scroll_to_bottom(&self, cx: &mut Context<Self>) {
        // 使用 scroll_handle 滚动到底部
        cx.defer(|_| {});
    }

    /// 渲染头部
    fn render_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let status = self.session.status;
        let status_color = status.color();
        
        h_flex()
            .px(px(16.0))
            .py(px(12.0))
            .border_b_1()
            .border_color(rgb(0xe5e7eb))
            .items_center()
            .justify_between()
            .child(
                h_flex()
                    .gap(px(12.0))
                    .items_center()
                    .child(
                        div()
                            .size(px(32.0))
                            .rounded_full()
                            .bg(rgb(0xe9d5ff))
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(Label::new("AI").text_sm().font_weight(FontWeight::MEDIUM))
                    )
                    .child(
                        v_flex()
                            .gap(px(2.0))
                            .child(
                                Label::new(&self.session.agent_name)
                                    .text_base()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0x7c3aed))
                            )
                            .child(
                                Label::new(format!("任务 #{}", self.session.objective.task_id))
                                    .text_xs()
                                    .text_color(rgb(0x6b7280))
                            )
                    )
            )
            .child(
                h_flex()
                    .gap(px(8.0))
                    .items_center()
                    .child(
                        // 状态指示器
                        h_flex()
                            .gap(px(6.0))
                            .items_center()
                            .child(
                                div()
                                    .size(px(8.0))
                                    .rounded_full()
                                    .bg(rgb(status_color))
                                    .when(status == AgentExecutionStatus::Running, |this| {
                                        // TODO: Add pulsing animation for running status
                                        this
                                    })
                            )
                            .child(
                                Label::new(status.as_str())
                                    .text_xs()
                                    .text_color(rgb(status_color))
                                    .font_weight(FontWeight::MEDIUM)
                            )
                    )
                    .child(
                        // Live Trace 开关
                        {
                            let btn = Button::new("toggle-live-trace")
                                .label(if self.session.live_trace { "LIVE TRACE" } else { "TRACE" })
                                .xsmall();
                            if self.session.live_trace {
                                btn.primary()
                            } else {
                                btn.ghost()
                            }
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.set_live_trace(!this.session.live_trace, cx);
                                }))
                        }
                    )
            )
    }

    /// 渲染任务目标
    fn render_objective(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .px(px(16.0))
            .py(px(12.0))
            .gap(px(8.0))
            .child(
                Label::new("MISSION OBJECTIVE")
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0x6b7280))
            )
            .child(
                v_flex()
                    .gap(px(8.0))
                    .p(px(12.0))
                    .rounded_md()
                    .bg(rgb(0xf9fafb))
                    .border_1()
                    .border_color(rgb(0xe5e7eb))
                    .children(self.session.objective.descriptions.iter().map(|desc| {
                        h_flex()
                            .gap(px(8.0))
                            .child(Label::new(">").text_sm().text_color(rgb(0x9ca3af)))
                            .child(Label::new(desc.clone()).text_sm())
                            .into_any_element()
                    }))
            )
    }

    /// 渲染时间线
    fn render_timeline(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        let messages = self.session.messages.clone();
        
        div()
            .flex_1()
            .overflow_hidden()
            .child(
                v_flex()
                    .h_full()
                    .px(px(16.0))
                    .py(px(12.0))
                    .gap(px(16.0))
                    .children(messages.iter().enumerate().map(|(_idx, msg)| {
                        Self::render_message_item_static(msg)
                    }))
            )
    }

    /// 渲染单个消息项（静态方法）
    fn render_message_item_static(msg: &AgentMessage) -> impl IntoElement {
        let (bg_color, text_color) = msg.message_type.tag_color();
        let time = msg.formatted_time();
        
        v_flex()
            .gap(px(8.0))
            .child(
                h_flex()
                    .gap(px(8.0))
                    .items_center()
                    // 时间线圆点
                    .child(
                        div()
                            .size(px(8.0))
                            .rounded_full()
                            .bg(rgb(0xd1d5db))
                    )
                    // 标签
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(2.0))
                            .rounded_sm()
                            .bg(rgb(bg_color))
                            .child(
                                Label::new(msg.message_type.as_str())
                                    .text_xs()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(text_color))
                            )
                    )
                    // 时间
                    .child(
                        Label::new(time)
                            .text_xs()
                            .text_color(rgb(0x9ca3af))
                    )
            )
            .child(
                div()
                    .pl(px(16.0))
                    .ml(px(3.0))
                    .border_l_1()
                    .border_color(rgb(0xe5e7eb))
                    .child(match &msg.metadata {
                        Some(AgentMessageMetadata::Tool { tool_name, command, output, status }) => {
                            Self::render_tool_execution_static(tool_name, command, output, *status)
                        }
                        Some(AgentMessageMetadata::Analysis { severity, findings, recommendations }) => {
                            Self::render_analysis_static(&msg.content, *severity, findings, recommendations)
                        }
                        _ => {
                            // 普通文本消息
                            Label::new(msg.content.clone())
                                .text_sm()
                                .into_any_element()
                        }
                    })
            )
    }

    /// 渲染工具执行（静态方法）
    fn render_tool_execution_static(
        tool_name: &str,
        command: &str,
        output: &str,
        status: ToolExecutionStatus,
    ) -> AnyElement {
        let status_color = match status {
            ToolExecutionStatus::Running => 0xf59e0b,
            ToolExecutionStatus::Success => 0x22c55e,
            ToolExecutionStatus::Failed => 0xef4444,
        };
        let status_text = match status {
            ToolExecutionStatus::Running => "Running",
            ToolExecutionStatus::Success => "Success",
            ToolExecutionStatus::Failed => "Failed",
        };

        v_flex()
            .gap(px(8.0))
            .child(
                h_flex()
                    .gap(px(8.0))
                    .items_center()
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded_md()
                            .bg(rgb(0x1f2937))
                            .child(
                                h_flex()
                                    .gap(px(6.0))
                                    .items_center()
                                    .child(Label::new(">" ).text_sm().text_color(rgb(0x9ca3af)))
                                    .child(Label::new(tool_name.to_string()).text_sm().text_color(rgb(0xa78bfa)))
                            )
                    )
                    .child(
                        div()
                            .px(px(6.0))
                            .py(px(2.0))
                            .rounded_sm()
                            .bg(rgb(status_color))
                            .child(
                                Label::new(status_text)
                                    .text_xs()
                                    .text_color(rgb(0xffffff))
                                    .font_weight(FontWeight::MEDIUM)
                            )
                    )
            )
            .child(
                div()
                    .w_full()
                    .rounded_md()
                    .bg(rgb(0x1f2937))
                    .child(
                        v_flex()
                            .child(
                                // 命令行
                                div()
                                    .px(px(12.0))
                                    .py(px(8.0))
                                    .border_b_1()
                                    .border_color(rgb(0x374151))
                                    .child(
                                        Label::new(format!("$ {}", command))
                                            .text_sm()
                                            .font_family(gpui::SharedString::from("monospace"))
                                            .text_color(rgb(0xe5e7eb))
                                    )
                            )
                            .child(
                                // 输出
                                div()
                                    .px(px(12.0))
                                    .py(px(8.0))
                                    .child(
                                        Label::new(output.to_string())
                                            .text_sm()
                                            .font_family(gpui::SharedString::from("monospace"))
                                            .text_color(rgb(0x9ca3af))
                                    )
                            )
                    )
            )
            .into_any_element()
    }

    /// 渲染分析结果（静态方法）
    fn render_analysis_static(
        content: &str,
        severity: u8,
        findings: &[String],
        _recommendations: &[String],
    ) -> AnyElement {
        let severity_color = if severity >= 8 {
            0xdc2626 // critical - red
        } else if severity >= 5 {
            0xf59e0b // high - amber
        } else {
            0x3b82f6 // medium/low - blue
        };

        let findings_owned: Vec<String> = findings.to_vec();

        v_flex()
            .gap(px(8.0))
            .child(
                h_flex()
                    .gap(px(8.0))
                    .items_center()
                    .child(
                        div()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded_md()
                            .bg(rgb(severity_color))
                            .child(
                                Label::new(format!("严重级别: {}/10", severity))
                                    .text_xs()
                                    .text_color(rgb(0xffffff))
                                    .font_weight(FontWeight::SEMIBOLD)
                            )
                    )
            )
            .child(
                Label::new(content.to_string())
                    .text_sm()
            )
            .children(if !findings_owned.is_empty() {
                Some(
                    v_flex()
                        .gap(px(4.0))
                        .mt(px(4.0))
                        .child(Label::new("发现:").text_xs().text_color(rgb(0x6b7280)))
                        .children(findings_owned.into_iter().map(|f| {
                            h_flex()
                                .gap(px(6.0))
                                .child(Label::new("•").text_sm().text_color(rgb(0x9ca3af)))
                                .child(Label::new(f).text_sm())
                                .into_any_element()
                        }))
                )
            } else {
                None
            })
            .into_any_element()
    }
}

impl Render for AgentExecutionPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(rgb(0xffffff))
            .rounded_md()
            .border_1()
            .border_color(rgb(0xe5e7eb))
            .child(self.render_header(cx))
            .child(self.render_objective(cx))
            .child(div().h(px(1.0)).bg(rgb(0xe5e7eb)))
            .child(self.render_timeline(cx))
    }
}

impl EventEmitter<AgentExecutionEvent> for AgentExecutionPanel {}

/// 示例数据生成（用于测试）
pub fn create_demo_session() -> AgentExecutionSession {
    let objective = MissionObjective::new("Analyze Flight Logs", 2)
        .with_description("分析目标无人机通信接口的注入漏洞")
        .with_description("重点关注遗留 PHP 端点");

    let mut session = AgentExecutionSession::new("PENLIGENT AGENT", objective);
    session.start();

    // 添加示例消息
    session.add_history("Initial reconnaissance completed. Target appears to be running OpenResty + PHP 5.6.40. Several potentially vulnerable parameters identified.");
    
    session.add_thought("Detected suspicious parameter `?ip=` in the URL. This pattern suggests a potential Command Injection vulnerability. The legacy PHP version (5.6.40) increases the likelihood of unpatched security flaws.");
    
    session.add_plan("1. Verify connection to target. 2. Fuzz the `ip` parameter with common injection payloads. 3. Analyze response time and content for execution indicators.", 1, 3);
    
    session.add_tool_execution(
        "curl",
        "curl -s -I -L --max-time 10 'http://target-drone-api:8080/?ip=127.0.0.1;id'",
        "uid=33(www-data) gid=33(www-data) groups=33(www-data)",
        ToolExecutionStatus::Success,
    );
    
    session.add_analysis(
        "Command execution confirmed. The server returned the output of the `id` command. This is a CRITICAL vulnerability allowing remote code execution.",
        10,
        vec!["Command Injection in `ip` parameter".into()],
        vec!["立即修补漏洞".into(), "升级 PHP 版本".into()],
    );

    session.complete();
    session
}
