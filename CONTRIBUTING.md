# Contributing to UAV Red Team

感谢你对 UAV Red Team 项目的兴趣!

## 开发指南

### 代码风格

- 遵循 Rust 标准代码风格
- 运行 `cargo fmt` 格式化代码
- 运行 `cargo clippy` 检查代码质量
- 为公共 API 添加文档注释

### 提交代码

1. Fork 项目
2. 创建特性分支: `git checkout -b feature/amazing-feature`
3. 提交更改: `git commit -m 'Add amazing feature'`
4. 推送到分支: `git push origin feature/amazing-feature`
5. 提交 Pull Request

### Commit Message 格式

```
<type>(<scope>): <subject>

<body>

<footer>
```

Type:
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式调整
- `refactor`: 重构
- `test`: 测试相关
- `chore`: 构建/工具相关

### 测试

- 为新功能添加单元测试
- 确保所有测试通过: `cargo test`
- 测试覆盖率目标: 80%+

### Agent 开发

参考 `AGENTS.md` 了解 Agent 系统架构。

添加新 Agent 的步骤:
1. 在 `AgentCapability` 枚举中定义能力
2. 实现执行逻辑
3. 在 Scheduler 中注册
4. 添加单元测试
5. 更新文档

## 行为准则

- 尊重所有贡献者
- 建设性反馈
- 专注技术讨论
- 遵守开源协议

## 安全披露

如果发现安全漏洞,请**不要**公开提交 issue。
请通过私密方式联系维护者。

## License

MIT License - 详见 LICENSE 文件
