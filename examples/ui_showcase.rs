// UI Component Showcase
// 用于测试和展示各个 UI 组件

use gpui::*;

fn main() {
    println!("UI Showcase Example");
    println!("====================");
    println!();
    println!("已实现的组件:");
    println!("1. NavigationBar - 顶部导航栏");
    println!("   - Tab 切换");
    println!("   - Target 显示");
    println!("   - AI Status 指示器");
    println!();
    println!("2. KanbanBoard - Kanban 看板");
    println!("   - To Do / In Progress / Done 列");
    println!("   - 任务卡片");
    println!("   - 优先级和类型标签");
    println!();
    println!("3. FindingsView - 安全发现列表");
    println!("   - 严重程度指示");
    println!("   - 过滤和搜索");
    println!("   - 状态标签");
    println!();
    println!("4. AgentPanel - AI Agent 面板");
    println!("   - 实时日志");
    println!("   - Mission Objective");
    println!("   - 代码执行显示");
    println!();
    println!("运行主应用查看完整 UI:");
    println!("cargo run");
}
