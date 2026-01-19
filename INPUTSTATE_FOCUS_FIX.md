# InputState 焦点设置关键修复

## 问题
虽然输入法被激活，但文字无法输入到输入框中。这是因为 InputState 虽然被创建了，但它的 EntityInputHandler 没有被正确激活。

## 根本原因
`EntityInputHandler` 需要在对话框完全初始化后才能正确处理输入。如果焦点设置太早或在错误的时机，InputState 不会接收文本输入事件。

## 解决方案
在 `window.open_dialog()` 的闭包内部，创建 Form 实体后**立即设置焦点**：

```rust
window.open_dialog(cx, move |dialog, window, cx| {
    // 创建表单
    let form = cx.new(|cx| {
        AddTaskForm::new(window, cx)
    });

    // ✅ 关键：立即设置焦点
    // 这确保了 InputState 能在对话框初始化后正确接收键盘事件
    form.read(cx).title_input
        .read(cx)
        .focus_handle(cx)
        .focus(window, cx);

    // 构建对话框
    dialog
        .title("创建新任务")
        .child(form.clone())
});
```

## 修改的文件

### 1. `crates/dashboard_ui/src/dashboard_panel.rs`
- 在 `open_add_task_dialog()` 中，在创建 form 后立即设置焦点
- 位置：open_dialog 闭包内部，dialog 配置前

### 2. `crates/dashboard_ui/src/add_task_form.rs`
- 将 `title_input` 和 `description_input` 改为 `pub` 字段
- 这样 dashboard_panel 可以访问它们来设置焦点

## 为什么这个修复有效

### 事件处理链条
```
Window.open_dialog() 创建对话框上下文
  ↓
Form 实体在对话框上下文中创建
  ↓
focus_handle.focus(window, cx) 设置焦点
  ↓
Dialog 的焦点系统识别焦点位置
  ↓
键盘事件被路由到有焦点的 InputState
  ↓
InputState.EntityInputHandler.replace_text_in_range() 被调用
  ↓
文字被插入到 Rope 缓冲区
  ↓
Input 组件重新渲染显示新文字
```

### 关键时机
- ❌ **太早**：在 form 创建之前设置焦点 → 焦点对象不存在
- ❌ **太晚**：在 dialog 返回后设置焦点 → 对话框已关闭或重新渲染
- ✅ **正确**：form 创建直后，dialog 配置前 → 焦点在对话框初始化时立即生效

## 技术细节

### InputState 的 EntityInputHandler
```rust
impl EntityInputHandler for InputState {
    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        // 这个方法处理输入法输入的文字
    }
}
```

当 InputState 有焦点时，GPUI 框架会调用这个方法来处理文本输入。

### 焦点句柄链条
```rust
form.read(cx)                     // 获取 Form 引用
    .title_input                  // 获取 InputState 实体
    .read(cx)                     // 读取 InputState
    .focus_handle(cx)             // 获取焦点句柄
    .focus(window, cx)            // 设置焦点
```

## 测试方法

1. **打开对话框** - 点击任何 Kanban 列的"+"按钮
2. **观察输入法** - 输入法应该立即被激活
3. **输入文字** - 输入框应该立即显示输入的字符
4. **验证多行** - 在描述框中也应该能输入

## 对比修复前后

### 修复前（无法输入）
```rust
window.open_dialog(cx, move |dialog, window, cx| {
    let form = cx.new(|cx| AddTaskForm::new(window, cx));
    // ❌ 没有设置焦点
    dialog.child(form)
});
```
- 结果：输入法激活但无法输入

### 修复后（可以输入）
```rust
window.open_dialog(cx, move |dialog, window, cx| {
    let form = cx.new(|cx| AddTaskForm::new(window, cx));
    // ✅ 立即设置焦点
    form.read(cx).title_input.read(cx).focus_handle(cx).focus(window, cx);
    dialog.child(form)
});
```
- 结果：输入法激活，文字正确输入显示

## 编译状态
✅ 完全编译成功，无错误无警告

## 相关知识

### GPUI 的焦点系统
- 每个 Element 可以通过 `.track_focus(&focus_handle)` 跟踪焦点
- Dialog 自动跟踪其内部的焦点变化
- 有焦点的 Entity 如果实现了 `EntityInputHandler` 会接收文本输入

### Window 的输入处理
- `window.open_dialog()` 创建一个新的焦点域
- 对话框内的元素可以独立获得焦点
- 焦点需要在对话框初始化时设置，而不是之后

## 下一步
✅ 文本输入完全可用
✅ 表单验证工作
✅ 任务创建成功

现在用户可以完全正常地使用任务创建对话框！
