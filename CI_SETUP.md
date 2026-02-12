# UAVRed CI/CD 设置指南

## 概述

本文档介绍如何设置和使用 UAVRed 的 CI/CD 流水线。

## 文件结构

```
.github/workflows/ci.yml    # 主 CI/CD 配置
ci/
  ├── playwright.config.ts  # Playwright 测试配置
  ├── tests/
  │   └── dashboard.spec.ts # UI 测试用例
  └── package.json          # Node.js 依赖
```

## 工作流说明

### 1. 代码质量检查 (quality-check)
- **触发**: 每次 push 和 PR
- **任务**:
  - 代码格式化检查 (`cargo fmt`)
  - Clippy 静态分析
  - 依赖安全审计

### 2. 前端构建 (build-frontend)
- **触发**: push 到 main/develop
- **任务**:
  - 安装 wasm32-unknown-unknown 目标
  - 安装 trunk
  - 构建 WASM 前端
  - 上传 dist 产物

### 3. 后端测试 (test-backend)
- **触发**: push 和 PR
- **任务**:
  - 运行单元测试
  - 生成测试报告

### 4. Tauri 构建
- **触发**: push 到 main 或手动触发
- **平台**:
  - macOS (Universal Binary)
  - Linux (x86_64)
  - Windows (x86_64)
- **产物**: `.dmg`, `.AppImage`, `.msi`

### 5. UI 测试 (ui-test)
- **触发**: 前端构建成功后
- **工具**: Playwright
- **测试内容**:
  - 五栏布局验证
  - Agent 面板展开/关闭
  - 截图对比

### 6. 自动发布 (release)
- **触发**: 打 tag 时
- **任务**:
  - 收集所有平台构建产物
  - 创建 GitHub Release

## 使用方法

### 本地测试 CI 配置

```bash
# 安装 act 工具 (GitHub Actions 本地运行器)
brew install act

# 运行 CI 工作流
cd /Users/fk/Devlopment/uavred
act push
```

### 手动触发构建

1. 进入 GitHub Actions 页面
2. 选择 "CI/CD" 工作流
3. 点击 "Run workflow"
4. 选择构建类型:
   - `dev`: 仅测试
   - `release`: 构建发布版本
   - `all_platforms`: 构建所有平台

### 查看构建产物

构建完成后，产物可以在以下位置下载:

1. **GitHub Actions 页面** → Artifacts
2. **Releases 页面** (tag 触发时)

## 环境变量

在 GitHub 仓库设置中添加以下 Secrets:

| 名称 | 说明 | 必需 |
|------|------|------|
| `GITHUB_TOKEN` | 自动提供 | 是 |

## 故障排查

### 构建失败: WASM 目标未找到

```bash
# 本地安装 WASM 目标
rustup target add wasm32-unknown-unknown
```

### 构建失败: trunk 未找到

```bash
# 本地安装 trunk
cargo install --locked trunk
```

### UI 测试失败

```bash
# 本地运行 UI 测试
cd ci
npm install
npx playwright install
npx playwright test
```

## 性能优化

### 缓存策略

CI 使用 `Swatinem/rust-cache` 缓存:
- Cargo 依赖
- 编译产物
- trunk 构建缓存

### 并行构建

- 前端和后端并行构建
- 三个平台的 Tauri 构建并行

## 扩展建议

### 添加更多测试

在 `ci/tests/` 目录添加更多 `.spec.ts` 文件:

```typescript
// ci/tests/vulns.spec.ts
import { test, expect } from '@playwright/test';

test('漏洞列表页', async ({ page }) => {
  await page.goto('/vulns');
  await expect(page.getByText('Vulnerabilities')).toBeVisible();
});
```

### 添加代码覆盖率

```yaml
# 在 ci.yml 中添加
- name: 生成覆盖率报告
  run: |
    cd backend
    cargo tarpaulin --out Xml

- name: 上传覆盖率
  uses: codecov/codecov-action@v3
  with:
    files: backend/cobertura.xml
```

### 添加 Docker 构建

```yaml
# 添加新的 job
build-docker:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - name: 构建 Docker 镜像
      run: |
        docker build -t uavred:latest .
        docker save uavred:latest > uavred-docker.tar
```

## 参考链接

- [GitHub Actions 文档](https://docs.github.com/en/actions)
- [Tauri CI/CD 指南](https://tauri.app/v1/guides/building/cross-platform/)
- [Playwright 文档](https://playwright.dev/)
