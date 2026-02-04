# AI Settings UI 测试指南

本文档介绍如何测试 AI 供应商设置界面的交互功能。

## 测试概述

AI 供应商设置界面 (`AiSettingsPanel`) 包含以下主要功能：

1. **供应商列表显示** - 左侧显示所有可用的 AI 供应商
2. **供应商选择** - 点击供应商卡片切换配置面板
3. **启用/禁用供应商** - 通过开关控制供应商激活状态
4. **API 配置** - 设置端点 URL 和 API Key
5. **连接测试** - 验证 API 连接是否正常
6. **模型获取** - 从供应商 API 获取可用模型列表
7. **模型管理** - 启用/禁用特定模型
8. **搜索过滤** - 支持供应商和模型的搜索过滤
9. **集成配置** - 显示 Claude Code 等工具的集成命令

## 运行测试

### 方法 1: 运行 GPUI 单元测试

```bash
# 运行 settings_ui crate 的所有测试
cargo test -p settings_ui

# 运行特定测试
cargo test -p settings_ui test_provider_selection
cargo test -p settings_ui test_provider_enable_toggle
cargo test -p settings_ui test_model_search_filter

# 运行所有 AI 设置相关测试
cargo test -p settings_ui ai_settings

# 查看测试输出详情
cargo test -p settings_ui -- --nocapture
```

### 方法 2: 手动测试指南

如果你想手动测试界面交互，请按以下步骤操作：

#### 基础功能测试

1. **启动应用**
   ```bash
   cargo run
   ```

2. **导航到 AI 设置**
   - 点击顶部导航栏的 "Settings"
   - 在左侧边栏点击 "AI" 类别

3. **测试供应商列表**
   - ✅ 验证显示 5 个供应商：Kimi, DeepSeek, OpenAI, Claude, Ollama
   - ✅ 每个供应商显示图标、名称、描述
   - ✅ 已启用模型数量显示为徽章
   - ✅ 供应商状态指示灯（绿色=启用，灰色=禁用）

4. **测试供应商选择**
   - ✅ 点击 DeepSeek，右侧显示 DeepSeek 配置面板
   - ✅ 点击 Kimi，右侧显示 Kimi 配置面板（包含 Kimi Code 集成区域）
   - ✅ 选中供应商卡片高亮显示

5. **测试启用/禁用开关**
   - ✅ 点击开关启用供应商，状态指示灯变绿
   - ✅ 再次点击禁用供应商，状态指示灯变灰
   - ✅ 设置自动保存

6. **测试 API 配置**
   - ✅ 端点输入框可编辑
   - ✅ API Key 输入框可编辑（密码样式显示）
   - ✅ 默认值根据供应商自动填充

7. **测试搜索过滤**
   - ✅ 在供应商搜索框输入 "deep"，只显示 DeepSeek
   - ✅ 清空搜索框，显示所有供应商
   - ✅ 在模型搜索框输入关键词，过滤模型列表

8. **测试连接功能**
   - ✅ 点击 "Test Connection" 按钮
   - ✅ 显示加载状态
   - ✅ 成功显示绿色提示，失败显示红色错误

9. **测试模型获取**
   - ✅ 输入有效的 API Key
   - ✅ 点击 "Fetch Models"
   - ✅ 成功获取后模型列表更新

10. **测试模型启用/禁用**
    - ✅ 点击模型旁边的开关
    - ✅ 供应商卡片上的启用数量徽章更新

11. **测试保存设置**
    - ✅ 点击 "Save Settings"
    - ✅ 显示绿色成功提示

## 测试文件说明

### 单元测试文件

`src/ai_settings_test.rs` 包含以下测试用例：

| 测试名称 | 描述 |
|---------|------|
| `test_provider_list_initialization` | 验证所有供应商正确加载 |
| `test_provider_selection` | 测试供应商选择切换 |
| `test_provider_enable_toggle` | 测试启用/禁用开关 |
| `test_provider_search_filter` | 测试供应商搜索过滤 |
| `test_default_endpoints` | 验证各供应商默认端点 |
| `test_model_toggle` | 测试模型启用/禁用 |
| `test_model_search_filter` | 测试模型搜索过滤 |
| `test_save_settings` | 测试设置保存功能 |
| `test_connection_test_state` | 验证连接测试状态管理 |
| `test_integration_section_rendering` | 测试集成配置区域渲染 |

## 调试技巧

### 启用详细日志

```bash
RUST_LOG=debug cargo run
```

### 检查配置文件

设置保存在用户配置目录：
- macOS: `~/Library/Application Support/uavred/settings.json`
- Linux: `~/.config/uavred/settings.json`

### 常见问题排查

1. **测试失败：无法创建窗口**
   - 确保在非图形环境使用 `cargo test` 而非运行完整应用
   - GPUI 测试需要图形环境，在 CI/无头环境中可能失败

2. **API 连接测试失败**
   - 检查网络连接
   - 验证 API Key 是否正确
   - 查看日志中的详细错误信息

3. **模型列表为空**
   - 确保已输入有效的 API Key
   - 检查供应商 API 是否可用

## 扩展测试

要添加新的测试用例，在 `ai_settings_test.rs` 中添加：

```rust
#[gpui::test]
async fn test_your_feature(cx: &mut TestAppContext) {
    let (panel, _window) = init_test(cx);
    
    cx.update(|cx| {
        panel.update(cx, |panel, cx| {
            // 你的测试逻辑
        });
    });
}
```

## 相关文件

- `src/ai_settings.rs` - AI 设置面板主实现
- `src/ai_settings_test.rs` - 测试文件
- `src/config.rs` - 设置配置结构
- `src/provider/` - 供应商实现
- `src/components/` - UI 组件
