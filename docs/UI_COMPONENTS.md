# UI Components Guide

本项目基于 [longbridge/gpui-component](https://github.com/longbridge/gpui-component) 构建用户界面。

## gpui-component 特性

### 丰富组件库
- 60+ 跨平台桌面 UI 组件
- 受 macOS 和 Windows 控件启发
- 结合 shadcn/ui 设计理念

### 设计原则
- **无状态**: RenderOnce 组件,简单易用
- **可定制**: 内置主题系统,支持多主题和变量配置
- **灵活尺寸**: 支持 xs, sm, md, lg 等尺寸
- **Dock 布局**: 面板排列、调整大小和自由布局
- **高性能**: 虚拟化 Table 和 List 组件

### 内容渲染
- Markdown 原生支持
- 简单 HTML 支持
- 内置图表组件
- 高性能代码编辑器(支持 200K 行)
- LSP 支持(诊断、补全、悬停等)
- Tree Sitter 语法高亮

## 项目中使用的组件

### 布局组件
```rust
use ui::{h_flex, v_flex}; // 水平/垂直 flex 布局
use ui::card;             // 卡片容器
use ui::divider;          // 分割线
```

### 基础组件
```rust
use ui::label;            // 文本标签
use ui::button;           // 按钮
use ui::badge;            // 徽章
```

### 高级组件(待使用)
```rust
use ui::table;            // 虚拟化表格
use ui::list;             // 虚拟化列表
use ui::input;            // 输入框
use ui::select;           // 下拉选择
use ui::tab;              // 标签页
use ui::modal;            // 模态框
use ui::tooltip;          // 提示信息
```

## 设计风格指南

遵循 AGENTS.md 中定义的 UI 设计原则:

### 信息密度
- 紧凑但清晰的信息展示
- 避免不必要的空白
- 充分利用屏幕空间

### 颜色系统
```rust
// 语义化颜色
Success:   rgb(0x4ade80)  // 绿色
Warning:   rgb(0xfbbf24)  // 黄色
Error:     rgb(0xf87171)  // 红色
Info:      rgb(0x60a5fa)  // 蓝色

// 背景色
Primary:   rgb(0x1e1e1e)  // 主背景
Secondary: rgb(0x252525)  // 次级背景
Border:    rgb(0x2d2d2d)  // 边框

// 文字色
Text:      rgb(0xffffff)  // 主文字
Muted:     rgb(0x9ca3af)  // 次要文字
```

### 图标使用
- 最小化装饰性图标
- 仅用于功能性标识
- 保持简洁一致

### 字体
- 技术信息使用等宽字体
- 一般文本使用系统字体

### 动画
- 简洁的状态转换
- 避免过度动效
- 注重性能

## 示例代码

### 创建简单卡片
```rust
card()
    .child(
        v_flex()
            .gap_2()
            .child(label("标题").text_size(TextSize::Large))
            .child(divider())
            .child(label("内容"))
    )
```

### 创建按钮组
```rust
h_flex()
    .gap_2()
    .child(button("主要操作").primary())
    .child(button("次要操作").secondary())
    .child(button("取消").small())
```

### 创建状态徽章
```rust
h_flex()
    .gap_1()
    .child(badge("运行中").success())
    .child(badge("警告").warning())
    .child(badge("错误").error())
```

## 参考资源

- [gpui-component 官方文档](https://longbridge.github.io/gpui-component/)
- [gpui-component GitHub](https://github.com/longbridge/gpui-component)
- [示例应用: Longbridge Pro](https://longbridge.com)
