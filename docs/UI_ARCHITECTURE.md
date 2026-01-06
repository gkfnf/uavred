# UI 架构说明

## 当前代码结构

### 1. 整体架构

```
src/
├── main.rs              # 应用入口，初始化 GPUI 和窗口
├── app.rs               # 主应用组件，包含所有页面和导航逻辑
├── core/                # 核心业务逻辑
│   ├── task.rs          # 任务管理
│   └── vuln_db.rs       # 漏洞数据库
├── agent/               # AI Agent 相关
├── scanner/             # 扫描器模块
└── ui/                  # UI 样式和组件（旧代码，可迁移）
```

### 2. 主应用结构 (`app.rs`)

```rust
pub struct UavRedTeamApp {
    active_view: AppView,  // 当前激活的视图
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppView {
    Dashboard,  // 看板视图
    Assets,     // 资产管理
    Scan,       // 扫描页面
    Vulns,      // 漏洞列表
    Traffic,    // 流量监控
    Flows,      // 工作流
}
```

**特点：**
- 单一状态管理：使用 `active_view` 枚举跟踪当前页面
- 集中式渲染：所有页面在 `render_main_content` 中根据 `active_view` 切换
- 组件化设计：每个页面有独立的 `render_*` 方法

### 3. 渲染流程

```
main.rs
  └─> Application::new()
      └─> app.run()
          └─> Root::new(UavRedTeamApp)
              └─> UavRedTeamApp::render()
                  ├─> render_navigation()    # 顶部导航栏
                  └─> render_main_content()  # 主内容区
                      ├─> render_dashboard()
                      ├─> render_assets()
                      ├─> render_scan()
                      ├─> render_vulns()
                      ├─> render_traffic()
                      └─> render_flows()
```

## 交互处理机制

### 1. 当前实现的交互

#### 导航切换
```rust
fn nav_item(&mut self, cx: &mut Context<Self>, view: AppView, label: &str) -> impl IntoElement {
    // ...
    Button::new(format!("nav-{:?}", view))
        .on_click(cx.listener(move |this: &mut Self, _, _, _| {
            this.set_active_view(view);  // 切换视图
        }))
}
```

**工作原理：**
- 使用 `cx.listener()` 创建闭包监听器
- 闭包捕获 `view` 值（实现了 `Copy` trait）
- 点击时调用 `set_active_view()` 更新状态
- GPUI 自动触发重新渲染

#### 按钮点击
```rust
Button::new("start-scan")
    .primary()
    .label("Start Scan")
    .on_click(|_, _, _| {
        // 处理点击事件
    })
```

### 2. GPUI 交互系统

#### Context 和 Listener
```rust
// Context 提供状态管理和事件处理
cx: &mut Context<Self>

// Listener 用于创建事件处理器
cx.listener(|this: &mut Self, event, window, cx| {
    // this: 当前视图实例（可变引用）
    // event: 事件对象（如 ClickEvent）
    // window: 窗口对象
    // cx: 上下文（用于更新状态）
})
```

#### 状态更新流程
```
用户点击
  └─> Button.on_click() 触发
      └─> cx.listener() 闭包执行
          └─> 调用 set_active_view()
              └─> 更新 active_view 字段
                  └─> GPUI 检测到状态变化
                      └─> 自动调用 render() 重新渲染
                          └─> UI 更新显示新页面
```

## 未来交互处理方案

### 方案 1: 扩展状态管理（推荐）

#### 1.1 添加更多状态字段
```rust
pub struct UavRedTeamApp {
    active_view: AppView,
    // 添加页面特定状态
    dashboard_state: DashboardState,
    scan_state: ScanState,
    selected_asset: Option<String>,
    // ...
}
```

#### 1.2 使用 Entity 管理复杂状态
```rust
use gpui::Entity;

pub struct UavRedTeamApp {
    active_view: AppView,
    // 使用 Entity 管理有状态的组件
    input_state: Entity<InputState>,      // 输入框状态
    tree_state: Entity<TreeState>,        // 树形视图状态
    table_state: Entity<TableState>,      // 表格状态
}
```

#### 1.3 事件处理示例
```rust
impl UavRedTeamApp {
    // 处理扫描开始
    fn handle_start_scan(&mut self, target: String, cx: &mut Context<Self>) {
        // 1. 更新 UI 状态
        self.scan_state.status = ScanStatus::Running;
        
        // 2. 启动后台任务
        cx.spawn(async move {
            // 调用扫描器 API
            scanner::start_scan(&target).await?;
            Ok::<_, anyhow::Error>(())
        })
        .detach();
        
        // 3. 触发 UI 更新
        cx.notify();
    }
    
    // 处理任务卡片点击
    fn handle_task_click(&mut self, task_id: String, cx: &mut Context<Self>) {
        // 显示任务详情对话框
        self.show_task_detail(task_id, cx);
    }
}
```

### 方案 2: 使用 Action 系统

GPUI 支持 Action 系统，可以处理键盘快捷键和命令：

```rust
use gpui::Action;

#[derive(Clone, Action)]
enum AppAction {
    SwitchToDashboard,
    SwitchToAssets,
    StartScan,
    StopScan,
}

impl Render for UavRedTeamApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 注册快捷键
        cx.bind_keys([
            KeyBinding::new("cmd-1", AppAction::SwitchToDashboard, None),
            KeyBinding::new("cmd-2", AppAction::SwitchToAssets, None),
        ]);
        
        // 处理 Action
        cx.on_action(|this: &mut Self, action: &AppAction, _| {
            match action {
                AppAction::SwitchToDashboard => {
                    this.set_active_view(AppView::Dashboard);
                }
                AppAction::StartScan => {
                    this.handle_start_scan(cx);
                }
                // ...
            }
        });
        
        // 渲染 UI
        // ...
    }
}
```

### 方案 3: 组件化重构

将每个页面拆分为独立组件：

```rust
// src/ui/pages/dashboard.rs
pub struct Dashboard {
    kanban_state: Entity<KanbanState>,
}

impl Render for Dashboard {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Dashboard 特定的渲染逻辑
    }
}

// src/app.rs
impl UavRedTeamApp {
    fn render_main_content(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        match self.active_view {
            AppView::Dashboard => {
                // 使用独立的 Dashboard 组件
                self.dashboard.clone()
            }
            // ...
        }
    }
}
```

## 具体交互场景处理

### 1. 页面切换
```rust
// 当前实现：简单直接
fn nav_item(&mut self, cx: &mut Context<Self>, view: AppView, label: &str) -> impl IntoElement {
    Button::new(format!("nav-{:?}", view))
        .on_click(cx.listener(move |this, _, _, _| {
            this.set_active_view(view);
        }))
}
```

### 2. 表单提交
```rust
// 扫描配置表单
fn render_scan(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
    // ...
    Button::new("start-scan")
        .on_click(cx.listener(|this, _, _, cx| {
            // 获取输入值
            let target = this.scan_state.target.clone();
            // 启动扫描
            this.handle_start_scan(target, cx);
        }))
}
```

### 3. 列表项选择
```rust
// 漏洞卡片点击
fn render_vuln_card(&self, cve: &str, ...) -> impl IntoElement {
    GroupBox::new()
        .cursor_pointer()
        .on_click(cx.listener(move |this, _, _, cx| {
            // 显示漏洞详情
            this.show_vuln_detail(cve, cx);
        }))
}
```

### 4. 树形视图展开/折叠
```rust
// Assets 树形视图
let tree_state = cx.new(|cx| {
    TreeState::new(cx).items(vec![
        TreeItem::new("network", "Network")
            .expanded(true)  // 默认展开
            .children(vec![...])
    ])
});

// Tree 组件自动处理展开/折叠交互
tree(&tree_state, |ix, entry, selected, _, _| {
    ListItem::new(ix)
        .selected(selected)
        .on_click(cx.listener(|this, _, _, cx| {
            // 处理节点点击
            this.select_asset(entry.item().id.clone(), cx);
        }))
})
```

### 5. 对话框/模态框
```rust
use gpui_component::dialog::Dialog;

impl UavRedTeamApp {
    fn show_task_detail(&mut self, task_id: String, cx: &mut Context<Self>) {
        // 创建对话框状态
        let dialog = cx.new(|_| DialogState::new());
        self.active_dialog = Some(dialog);
        cx.notify();
    }
    
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .child(self.render_main_content(cx))
            .when_some(self.active_dialog, |this, dialog| {
                this.child(Dialog::new(&dialog, |_| {
                    // 对话框内容
                }))
            })
    }
}
```

## 最佳实践建议

### 1. 状态管理
- ✅ **简单状态**：直接存储在 `UavRedTeamApp` 结构体中
- ✅ **复杂状态**：使用 `Entity<T>` 管理（如 InputState, TreeState）
- ✅ **全局状态**：考虑使用 `AppContext` 或单例模式

### 2. 事件处理
- ✅ 使用 `cx.listener()` 创建事件处理器
- ✅ 在闭包中捕获需要的值（确保生命周期正确）
- ✅ 使用 `cx.notify()` 手动触发重新渲染（如需要）

### 3. 异步操作
```rust
// 启动后台任务
cx.spawn(async move {
    // 异步操作
    let result = api_call().await?;
    
    // 更新 UI（需要在主线程）
    cx.update(|this, _| {
        this.scan_result = Some(result);
    })?;
    
    Ok::<_, anyhow::Error>(())
})
.detach();
```

### 4. 性能优化
- ✅ 使用 `Entity` 管理大型列表/表格状态
- ✅ 利用虚拟化列表（VirtualList）处理大量数据
- ✅ 避免在 `render()` 中创建大量临时对象

## 下一步改进方向

1. **状态管理增强**
   - 添加页面特定的状态结构
   - 实现状态持久化（保存用户偏好）

2. **交互完善**
   - 实现拖拽功能（Kanban 卡片）
   - 添加右键菜单
   - 实现键盘快捷键

3. **数据绑定**
   - 连接后端 API
   - 实现实时数据更新（WebSocket）
   - 添加数据缓存机制

4. **用户体验**
   - 添加加载状态指示器
   - 实现错误提示和确认对话框
   - 添加操作历史记录
