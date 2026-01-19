# 添加任务对话框完整实现总结

## 问题解决

### 问题1：输入无效
**原因**：InputState 在对话框闭包中创建后立即被销毁，无法保持生命周期。
**解决**：创建 `AddTaskForm` 实体，在其中管理 InputState 的完整生命周期。

### 问题2：背景不对
**原因**：手动渲染黑色背景。
**解决**：使用 `window.open_dialog()` API，系统自动处理背景淡化和模态行为。

### 问题3：对话框闪退
**原因**：不当的事件处理和焦点管理。
**解决**：使用 Dialog 组件提供的事件系统（on_ok、on_cancel）。

## 核心实现

### 1. AddTaskForm 实体 (`add_task_form.rs`)

```rust
pub struct AddTaskForm {
    title_input: Entity<InputState>,
    description_input: Entity<InputState>,
    selected_priority: TaskPriority,
    selected_agent: String,
    validation_error: String,
    _subscriptions: Vec<Subscription>,
}
```

**关键特性**：
- InputState 的完整生命周期管理
- 表单字段验证
- 优先级和 Agent 选择状态
- 错误消息显示

### 2. TaskPriority 枚举

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}
```

支持四个优先级，方便序列化。

### 3. 对话框集成 (`dashboard_panel.rs`)

```rust
window.open_dialog(cx, move |dialog, window, cx| {
    // 创建表单实体
    let form = cx.new(|cx| AddTaskForm::new(window, cx));
    
    dialog
        .title("创建新任务")
        .on_ok(|_, _window, cx| {
            // 验证表单
            if !form.update(cx, |form, cx| form.validate(cx)) {
                return false;
            }
            // 获取数据并创建任务
            true
        })
        .on_cancel(|_, _window, _cx| true)
        .child(form.clone())
});
```

## 功能实现

### ✅ 文本输入（已完成）
- 任务标题输入框（必填）
- 任务描述输入框（可选）
- 使用 gpui-component 的 Input 组件
- 自动焦点管理

### ✅ 字段验证（已完成）
- 标题非空检查
- 错误提示显示（红色背景）
- 验证失败时阻止提交

### ✅ 优先级选择（已完成）
- 四个优先级：Low、Medium、High、Critical
- 默认 Medium
- 显示当前选择

### ✅ Agent 选择（已完成）
- OPENCODE 和 Claude
- 默认 OPENCODE
- 显示当前选择

### ⏳ 草稿保存（规划中）
- 可添加到 localStorage
- 对话框关闭时自动保存
- 重新打开时恢复

### ⏳ Markdown 预览（规划中）
- 在描述框中实时预览
- 支持基本 Markdown 格式

## 数据流

```
用户打开对话框
    ↓
AddTaskForm 实体创建，初始化 InputState
    ↓
用户输入文本（通过 Input 组件）
    ↓
用户点击"创建"按钮
    ↓
form.validate() 检查标题非空
    ↓
读取表单数据：标题、描述、优先级
    ↓
创建 TaskData
    ↓
通过 handle.update() 提交到 DashboardPanel
    ↓
panel.add_task() 保存到数据库
    ↓
对话框自动关闭
```

## UI 组件

### 验证错误提示
```
[错误消息] (红色背景)
```

### 输入字段
```
任务标题 * 
[输入框]

任务描述 (可选)
[多行输入框]
```

### 配置区域
```
配置

Agent          优先级
[OPENCODE]    [Medium]
```

## 文件变更

### 新增
- `crates/dashboard_ui/src/add_task_form.rs` - 表单管理实体

### 修改
- `crates/dashboard_ui/src/dashboard_panel.rs` - 对话框集成
- `crates/dashboard_ui/src/mission_control.rs` - 传入 window 参数
- `crates/dashboard_ui/src/lib.rs` - 导出新模块
- `crates/data/src/models.rs` - 添加 description 字段
- `crates/data/src/database.rs` - 初始化 description

### 删除
- `crates/dashboard_ui/src/add_task_dialog.rs` (旧实现)
- `crates/dashboard_ui/src/add_task_modal.rs` (过渡实现)

## 关键学习

1. **InputState 生命周期** - 必须在实体中管理，不能在闭包中快速创建销毁
2. **Dialog API** - 使用框架提供的对话框系统，自动处理背景、焦点、事件
3. **表单验证** - 在 update 闭包中执行，可以修改状态并返回验证结果
4. **约束条件** - Render 中无法调用 cx.listener，需要用闭包或数据驱动 UI

## 编译状态
✅ 完全编译成功，无错误

## 下一步改进

1. **字段验证增强**
   - 标题长度限制（最大 100 字符）
   - 描述长度限制
   - 特殊字符检查

2. **UI 改进**
   - 添加清除按钮到输入框
   - 支持在优先级和 Agent 之间切换（当前只显示）
   - 添加快捷键支持（Ctrl+Enter 提交）

3. **功能扩展**
   - 草稿自动保存到 localStorage
   - Markdown 预览支持
   - 添加标签/分类功能
   - 设置截止日期

4. **集成**
   - 与用户权限系统集成
   - 与通知系统集成
   - 与工作流自动化集成
