#!/bin/bash
# UI Subagents 启动脚本
# 使用方式: ./scripts/start-ui-agents.sh [agent-name|list|prompt|check]
#
# 示例:
#   ./scripts/start-ui-agents.sh list           # 列出所有可用 agents
#   ./scripts/start-ui-agents.sh vulns          # 启动 vulns agent
#   ./scripts/start-ui-agents.sh prompt vulns   # 仅输出 prompt (不启动)
#   ./scripts/start-ui-agents.sh check          # 检查所有 crate 编译状态

PROJECT_ROOT="/Users/fk/Devlopment/uavred"
cd "$PROJECT_ROOT"

# Agent 配置 - 名称 -> crate 路径映射
declare -A AGENTS=(
    ["dashboard"]="crates/dashboard_ui"
    ["vulns"]="crates/vulns_ui"
    ["traffic"]="crates/traffic_ui"
    ["assets"]="crates/assets_ui"
    ["flows"]="crates/flows_ui"
    ["devices"]="crates/devices_ui"
    ["monitor"]="crates/monitor_ui"
    ["settings"]="crates/settings_ui"
)

# Agent 复杂度标记
declare -A COMPLEXITY=(
    ["dashboard"]="中"
    ["vulns"]="高"
    ["traffic"]="高"
    ["assets"]="高"
    ["flows"]="高"
    ["devices"]="中"
    ["monitor"]="低"
    ["settings"]="低"
)

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 通用 Agent Prompt 模板
generate_prompt() {
    local name=$1
    local crate=$2
    cat << EOF
你是 ${name}-ui-agent，专门负责 UAVRed 项目的 ${name} UI 模块开发。

## 你的职责范围
- 目录: ${crate}/
- 请先阅读 ${crate}/CLAUDE.local.md 了解详细指令

## 核心规则
1. 仅修改 ${crate}/ 目录下的文件
2. 禁止修改: ui/theme.rs, data/models.rs, workspace.rs
3. 使用 ui::theme::* 中的常量，不硬编码颜色
4. 遵循 GPUI 组件模式 (Entity, Render trait)
5. 状态变更后调用 cx.notify()

## 验证命令
完成任务后请运行:
- cargo check -p ${name}_ui
- cargo clippy -p ${name}_ui

## 参考文件
- GPUI 模式: 参考项目根目录 CLAUDE.md
- 主题常量: crates/ui/src/theme.rs
- 数据模型: crates/data/src/models.rs

请告诉我你的第一个任务是什么？
EOF
}

# 启动单个 agent
start_agent() {
    local name=$1
    local crate="${AGENTS[$name]}"

    if [ -z "$crate" ]; then
        echo -e "${RED}Unknown agent: $name${NC}"
        echo "Available agents: ${!AGENTS[@]}"
        exit 1
    fi

    # 检查 CLAUDE.local.md 是否存在
    if [ ! -f "${PROJECT_ROOT}/${crate}/CLAUDE.local.md" ]; then
        echo -e "${YELLOW}Warning: ${crate}/CLAUDE.local.md not found${NC}"
        echo "Agent will work with reduced context."
    fi

    echo -e "${GREEN}Starting ${name}-ui-agent...${NC}"
    echo -e "Working directory: ${BLUE}${PROJECT_ROOT}/${crate}${NC}"
    echo -e "Complexity: ${COMPLEXITY[$name]}"
    echo ""

    # 切换到 crate 目录并启动
    cd "${PROJECT_ROOT}/${crate}"

    # 生成 prompt 并启动 claude
    local prompt=$(generate_prompt "$name" "$crate")
    echo "$prompt" | claude
}

# 列出所有 agents
list_agents() {
    echo -e "${BLUE}Available UI Agents:${NC}"
    echo "===================="
    printf "  %-12s %-25s %-8s %s\n" "NAME" "CRATE" "复杂度" "STATUS"
    echo "  ------------------------------------------------------------"

    for name in dashboard vulns traffic assets flows devices monitor settings; do
        local crate="${AGENTS[$name]}"
        local complexity="${COMPLEXITY[$name]}"
        local status=""

        if [ -f "${PROJECT_ROOT}/${crate}/CLAUDE.local.md" ]; then
            status="${GREEN}[configured]${NC}"
        else
            status="${YELLOW}[needs config]${NC}"
        fi

        printf "  %-12s %-25s %-8s %b\n" "$name" "$crate" "$complexity" "$status"
    done

    echo ""
    echo "Usage:"
    echo "  $0 <agent-name>       Start an agent"
    echo "  $0 prompt <name>      Show agent prompt without starting"
    echo "  $0 check              Check all crates compile"
}

# 检查所有 crate 编译状态
check_crates() {
    echo -e "${BLUE}Checking UI crates compilation...${NC}"
    echo ""

    local failed=0
    for name in dashboard vulns traffic assets flows devices monitor settings; do
        local crate="${name}_ui"
        printf "  Checking %-15s ... " "$crate"

        if cargo check -p "$crate" 2>/dev/null; then
            echo -e "${GREEN}OK${NC}"
        else
            echo -e "${RED}FAILED${NC}"
            ((failed++))
        fi
    done

    echo ""
    if [ $failed -eq 0 ]; then
        echo -e "${GREEN}All crates compile successfully!${NC}"
    else
        echo -e "${RED}$failed crate(s) failed to compile.${NC}"
        exit 1
    fi
}

# 并行启动多个 agents (使用 tmux)
start_parallel() {
    if ! command -v tmux &> /dev/null; then
        echo -e "${RED}tmux is required for parallel mode${NC}"
        echo "Install with: brew install tmux"
        exit 1
    fi

    local session="uavred-ui-agents"

    # 创建新的 tmux session
    tmux new-session -d -s "$session" -n "dashboard"

    # 为每个 agent 创建窗口
    for name in vulns traffic assets flows devices monitor settings; do
        tmux new-window -t "$session" -n "$name"
    done

    echo -e "${GREEN}Created tmux session: $session${NC}"
    echo "Attach with: tmux attach -t $session"
    echo ""
    echo "In each window, run:"
    echo "  ./scripts/start-ui-agents.sh <window-name>"
}

# 主入口
case "${1:-}" in
    "")
        list_agents
        ;;
    "list")
        list_agents
        ;;
    "check")
        check_crates
        ;;
    "prompt")
        if [ -z "$2" ]; then
            echo "Usage: $0 prompt <agent-name>"
            exit 1
        fi
        generate_prompt "$2" "${AGENTS[$2]}"
        ;;
    "parallel")
        start_parallel
        ;;
    *)
        start_agent "$1"
        ;;
esac
