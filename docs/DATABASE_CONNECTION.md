# 数据库连接指南

本文档说明如何在 macOS 上使用 Navicat 等数据库工具连接到 UAV Red Team 的 SQLite 数据库。

## 数据库文件位置

UAV Red Team 的任务数据库存储在：

```
~/Library/Application Support/uavred/tasks.db
```

## 连接步骤

### 1. 打开 Navicat

启动 Navicat Premium 或 Navicat for SQLite。

### 2. 创建新连接

1. 点击 "连接" -> "SQLite"
2. 配置连接信息：
   - **连接名**: `uavred`（或任意名称）
   - **文件路径**: 输入完整路径
     ```
     /Users/your_username/Library/Application Support/uavred/tasks.db
     ```
   - 替换 `your_username` 为你的实际用户名

### 3. 快速查找路径

你可以通过以下方式快速获取完整路径：

```bash
# 打开终端
ls -l ~/Library/Application\ Support/uavred/tasks.db
```

复制输出的完整路径。

### 4. 连接

点击 "连接" 按钮建立连接。

## 数据库表结构

### tasks 表

| 列名 | 类型 | 说明 |
|--------|------|------|
| id | INTEGER | 任务 ID（主键） |
| title | TEXT | 任务标题 |
| task_type | TEXT | 任务类型 |
| priority | TEXT | 优先级 |
| status | TEXT | 任务状态（todo/in_progress/done） |
| created_at | TEXT | 创建时间（ISO 8601 格式） |
| updated_at | TEXT | 更新时间（ISO 8601 格式） |

## 常见操作

### 查看所有任务

```sql
SELECT * FROM tasks ORDER BY id ASC;
```

### 删除所有任务

```sql
DELETE FROM tasks;
```

### 删除特定状态的任务

```sql
DELETE FROM tasks WHERE status = 'todo';
DELETE FROM tasks WHERE status = 'in_progress';
DELETE FROM tasks WHERE status = 'done';
```

### 重置自增 ID

```sql
DELETE FROM tasks;
DELETE FROM sqlite_sequence WHERE name='tasks';
```

## 注意事项

⚠️ **重要提醒**:

1. **应用运行时不要修改**: 建议在应用关闭的状态下修改数据库，避免冲突
2. **备份数据**: 修改前建议备份 tasks.db 文件
   ```bash
   cp ~/Library/Application\ Support/uavred/tasks.db ~/Desktop/tasks.db.backup
   ```
3. **数据格式**:
   - status 必须是以下之一：`todo`, `in_progress`, `done`
   - id 为自增主键，插入时不需要指定
   - 时间使用 ISO 8601 格式

## 其他数据库工具

除了 Navicat，你也可以使用以下工具：

- **DB Browser for SQLite** (免费, 开源)
- **SQLiteStudio** (免费)
- **TablePlus** (付费，支持多种数据库)
- **DBeaver** (免费开源)
- **命令行 sqlite3**:
  ```bash
  sqlite3 ~/Library/Application\ Support/uavred/tasks.db
  ```

## 故障排除

### 找不到数据库文件

确认应用至少运行过一次，这样数据库文件才会被创建。

### Navicat 连接失败

1. 检查文件路径是否正确（注意用户名）
2. 确认文件权限：`chmod 644 tasks.db`
3. 尝试复制数据库到其他位置后再连接

### 数据丢失

如果误操作导致数据丢失：
```bash
# 检查是否有备份
ls -la ~/Library/Application\ Support/uavred/*.backup

# 从 Time Machine 或其他备份恢复
```
