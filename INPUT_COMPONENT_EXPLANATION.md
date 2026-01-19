# Input 组件详细说明

## 概述

Input 组件是 gpui-component 提供的文本输入框，具有强大的功能：
- 单行和多行模式
- 焦点管理
- 事件系统
- 验证支持
- 掩码和格式化

## 三层架构

### 1. InputState (数据层)
```rust
pub struct AddTaskForm {
    title_input: Entity<InputState>,  // 输入框状态实体
}
```

**职责**：
- 管理文本缓冲区 (Rope)
- 处理键盘事件 (输入、删除、移动光标)
- 管理选择范围
- 处理焦点
- 发出 InputEvent

**生命周期**：
- 必须在 `Entity` 中创建和保存
- 必须持续存在直到对话框关闭
- 如果在闭包中创建会立即销毁

### 2. Input (渲染层)
```rust
Input::new(&self.title_input)
    .placeholder("输入任务标题...")
    .h(px(100.0))
```

**职责**：
- 渲染文本内容
- 显示光标
- 处理鼠标交互
- 提供样式选项

**关键特性**：
- `.h()` / `.h_full()` - 设置高度
- `.placeholder()` - 占位符文本
- `.cleanable()` - 显示清除按钮
- `.prefix()` / `.suffix()` - 前缀/后缀元素

### 3. 焦点管理 (交互层)
```rust
// 在 render 中自动获取焦点
if !self.title_input.read(cx).focus_handle(cx).is_focused(window) {
    self.title_input.read(cx).focus_handle(cx).focus(window, cx);
}
```

**为什么需要焦点管理**：
- InputState 本身实现了 `EntityInputHandler`
- 只有获得焦点的 InputState 才能接收键盘事件
- Dialog 的焦点系统需要知道哪个元素应该接收事件

## 为什么之前无法输入

### 问题1：InputState 生命周期丢失
```rust
// ❌ 错误：闭包结束后 InputState 被销毁
window.open_dialog(cx, move |dialog, window, cx| {
    let input = cx.new(|cx| InputState::new(window, cx));
    // 对话框渲染 → 闭包结束 → InputState 实体被销毁
});
```

**解决**：创建持久的 Form 实体
```rust
// ✅ 正确：InputState 在 Entity 中保存
pub struct AddTaskForm {
    title_input: Entity<InputState>,
}
```

### 问题2：焦点丢失
InputState 创建时没有自动获得焦点。Dialog 的焦点在对话框容器上，不在具体的输入框上。

**解决**：在 render 中主动设置焦点
```rust
fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    // 确保标题输入框获得焦点
    if !self.title_input.read(cx).focus_handle(cx).is_focused(window) {
        self.title_input.read(cx).focus_handle(cx).focus(window, cx);
    }
    
    v_flex().child(Input::new(&self.title_input))
}
```

### 问题3：事件处理链断裂
Input 需要通过以下链条接收事件：
```
Dialog.track_focus()
  ↓
InputState.focus_handle
  ↓
Input.on_action() 注册的事件处理
  ↓
InputState 处理键盘输入
```

如果焦点没有正确设置，事件不会到达 InputState。

## 正确的使用流程

### Step 1: 创建 Form 实体
```rust
pub struct AddTaskForm {
    title_input: Entity<InputState>,
    description_input: Entity<InputState>,
}

impl AddTaskForm {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let title_input = cx.new(|cx| {
            InputState::new(window, cx)
        });
        
        Self {
            title_input,
        }
    }
}
```

### Step 2: 在 render 中管理焦点
```rust
impl Render for AddTaskForm {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 设置焦点
        if !self.title_input.read(cx).focus_handle(cx).is_focused(window) {
            self.title_input.read(cx).focus_handle(cx).focus(window, cx);
        }
        
        v_flex().child(Input::new(&self.title_input))
    }
}
```

### Step 3: 在对话框中使用
```rust
window.open_dialog(cx, move |dialog, window, cx| {
    let form = cx.new(|cx| AddTaskForm::new(window, cx));
    
    dialog
        .title("创建新任务")
        .on_ok(move |_, _window, cx| {
            let title = form.read(cx).get_title(cx);
            true
        })
        .child(form.clone())
});
```

## 事件流详解

### 1. 用户按下键
```
键盘事件 (KeyDown)
  ↓
Dialog (track_focus) 检查焦点
  ↓
AddTaskForm (render) 渲染
  ↓
Input (on_action) 处理事件
  ↓
InputState.entity_input_handler 处理
  ↓
InputState 更新文本缓冲区
  ↓
InputState 发出 InputEvent::Change
```

### 2. InputState 发出事件
AddTaskForm 可以订阅 InputEvent：
```rust
cx.subscribe(&self.title_input, |this, input_state, event, cx| {
    match event {
        InputEvent::Change => {
            let text = input_state.read(cx).value();
            // 处理文本变化
        }
        InputEvent::PressEnter { secondary } => {
            // 处理回车键
        }
        _ => {}
    }
}).detach();
```

## 关键 API

### InputState 创建
```rust
InputState::new(window: &mut Window, cx: &mut Context<Self>) -> Self
```
- 必须在 `Context` 中创建
- 必须传入 `Window`
- 返回新的 InputState 实例

### 获取焦点 Handle
```rust
input_state.read(cx).focus_handle(cx)
```
- 获取焦点句柄
- 可用于检查焦点和设置焦点

### 设置焦点
```rust
focus_handle.focus(window: &mut Window, cx: &mut App)
```
- 主动将焦点设置给此输入框
- 键盘事件会发送给此输入框

### 检查焦点
```rust
focus_handle.is_focused(window: &Window) -> bool
```
- 检查此输入框是否有焦点

### 读取值
```rust
input_state.read(cx).value() -> RopeSlice
input_state.read(cx).value().to_string() -> String
```

## 常见问题

### Q: 为什么输入框显示了但无法输入？
A: 输入框没有获得焦点。确保在 render 中调用：
```rust
focus_handle.focus(window, cx)
```

### Q: 为什么输入框内的文字没有更新？
A: InputState 的值在其内部更新，但 UI 需要重新渲染。使用 `cx.notify()` 触发重新渲染。

### Q: 如何监听输入变化？
A: 使用 `cx.subscribe`：
```rust
cx.subscribe(&input_state, |this, state, event, cx| {
    if let InputEvent::Change = event {
        let value = state.read(cx).value();
    }
}).detach();
```

### Q: 如何设置初始值？
A: 在创建时使用 builder 方法：
```rust
InputState::new(window, cx)
    .default_value("初始值")
    .placeholder("占位符")
```

### Q: 如何禁用输入框？
A: Input 提供 `.disabled()` 方法，或通过 InputState：
```rust
input_state.update(cx, |state, _| {
    state.disabled = true;
});
```

## 总结

要让 Input 组件正常工作：

1. **创建** InputState 在 Entity 中并保存
2. **初始化** 时传入 Window 和 Context
3. **渲染** 时在每帧确保焦点正确设置
4. **验证** Dialog 的焦点系统正确跟踪此 Entity
5. **处理** 事件通过订阅 InputEvent

核心点：**焦点是输入的关键**。没有焦点，就没有事件；没有事件，就无法输入。
