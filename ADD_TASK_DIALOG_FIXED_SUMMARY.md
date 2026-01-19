# 添加任务对话框实现修复总结

## 问题分析

之前的实现存在以下问题：
1. **黑色背景**：手动渲染对话框背景，而不是使用系统的对话框管理
2. **闪退**：可能由于 InputState 初始化不当或事件处理不当导致
3. **输入失败**：手动管理的对话框没有正确处理焦点和事件

## 解决方案

使用 gpui-component 的 Dialog 系统和 WindowExt trait 正确处理弹出式对话框。

### 核心改变

#### 1. 使用 `window.open_dialog()` API（来自 WindowExt trait）
```rust
window.open_dialog(cx, move |dialog, window, cx| {
    // 构建和配置对话框
});
```

这个 API 自动处理：
- 对话框背景（淡化背景）
- 模态行为和焦点管理
- 事件处理和事件传播
- 对话框的生命周期管理

#### 2. 在对话框闭包中初始化 InputState
```rust
let title_input = cx.new(|cx| {
    gpui_component::input::InputState::new(window, cx)
});
```

InputState 需要 `&mut Window` 参数，在对话框闭包中可以获得。

#### 3. 使用 Dialog builder API
```rust
dialog
    .title("创建新任务")
    .w(px(600.0))
    .on_ok(|_, _window, cx| { ... })
    .on_cancel(|_, _window, _cx| true)
    .child(/* content */)
```

#### 4. 正确处理数据回调
使用 `Rc<RefCell<>>` 包装任务数据，在 on_ok 回调中更新并提交：
```rust
let task_data = Rc::new(RefCell::new(TaskData { ... }));

move |_, _window, cx| {
    let title = title_input.read(cx).value().to_string();
    task_data.borrow_mut().title = title;
    
    if let Some(handle) = this_handle.upgrade() {
        handle.update(cx, |panel, cx| {
            panel.add_task(task_data.borrow().clone(), cx);
        });
    }
    true  // 返回 true 关闭对话框
}
```

## 文件变更

### 删除的文件
- `crates/dashboard_ui/src/add_task_dialog.rs` （旧的静态对话框实现）
- `crates/dashboard_ui/src/add_task_modal.rs` （第一次改进尝试）

### 修改的文件

#### `crates/dashboard_ui/src/dashboard_panel.rs`
- 移除对 InputState 和对话框相关字段的引用
- 实现 `open_add_task_dialog()` 使用系统对话框 API
- 在对话框回调中创建 InputState 和处理用户输入

#### `crates/dashboard_ui/src/mission_control.rs`
- 更新调用 `open_add_task_dialog()` 的所有地方，传入 `window` 参数

#### `crates/dashboard_ui/src/lib.rs`
- 移除 `add_task_dialog` 和 `add_task_modal` 模块的导出

## 技术细节

### WindowExt trait
来自 `gpui_component::WindowExt`，提供：
- `open_dialog()` - 打开对话框
- `close_dialog()` - 关闭对话框
- `open_sheet()` - 打开侧面板
- 等等

### Dialog API 设计
Dialog 是一个 builder 样式的 API，可以链式调用方法：
```rust
dialog
    .title(...)      // 设置标题
    .w(...)          // 设置宽度
    .on_ok(...)      // 设置确认回调
    .on_cancel(...)  // 设置取消回调
    .child(...)      // 添加子组件
```

### 事件处理
- `on_ok(|_, window, cx| -> bool)` - 返回 true 关闭对话框
- `on_cancel(|_, window, cx| -> bool)` - 返回 true 关闭对话框
- 按 Escape 默认调用 `on_cancel`
- 按 Enter 默认调用 `on_ok`

## 关键学习点

1. **不要手动管理对话框UI** - 使用框架提供的 API
2. **背景管理** - Dialog 系统自动处理背景淡化和模态行为
3. **焦点管理** - Dialog 系统自动管理焦点转移
4. **InputState 生命周期** - 在对话框打开时创建，对话框关闭时销毁
5. **事件传播** - Dialog 系统处理事件阻止传播

## 编译状态
✅ 完全编译成功，无错误无警告

## 下一步改进

1. 添加字段验证和错误提示
2. 实现下拉菜单选择（Agent、优先级、分支）
3. 添加表单保存为草稿功能
4. 支持 Markdown 预览
