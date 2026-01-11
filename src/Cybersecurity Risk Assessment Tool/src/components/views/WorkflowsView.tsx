import { useState } from 'react';
import { Cpu, Play, Edit3, Trash2, Save, Plus, CheckCircle, XCircle, Clock, Search, Filter, Copy, BarChart3, Settings, Zap, AlertTriangle, ArrowRight, ChevronRight, ChevronDown, Folder, FolderOpen, Box, GitBranch, Layers, Timer, Maximize2, Minimize2 } from 'lucide-react';

// 工作流层级类型
type WorkflowLevel = 'atomic' | 'composite' | 'task';

// 节点输入输出类型（用于自动推断依赖）
interface IOType {
  name: string;
  type: 'ip' | 'port' | 'url' | 'vulnerability' | 'credential' | 'file' | 'data';
  required: boolean;
}

// 原子工作流节点
interface WorkflowNode {
  id: string;
  type: 'scan' | 'exploit' | 'validate' | 'report' | 'fuzz' | 'ai-analyze';
  label: string;
  atomicWorkflowId?: string; // 关联的原子工作流 ID
  inputs: IOType[];
  outputs: IOType[];
  estimatedDuration: number; // 秒
  status?: 'pending' | 'running' | 'completed' | 'failed' | 'skipped';
  startTime?: string;
  endTime?: string;
  position?: { x: number; y: number }; // 用于可视化布局
}

// 依赖边（带条件）
interface WorkflowEdge {
  from: string;
  to: string;
  condition?: string; // 执行条件，如 "on_success", "if port==80", "if vuln_found"
  conditionType?: 'always' | 'on_success' | 'on_failure' | 'conditional';
}

interface Workflow {
  id: number;
  name: string;
  description: string;
  level: WorkflowLevel;
  category: string;
  status: 'active' | 'draft' | 'archived';
  nodes: WorkflowNode[];
  edges: WorkflowEdge[];
  parentId?: number; // 父工作流 ID（用于层级关系）
  appliedTo: number[];
  runs: number;
  lastRun: string;
  successRate: number;
  avgDuration: string;
  estimatedDuration: number;
  tags: string[];
  // 并行执行统计
  maxParallelism?: number; // 最大并行度
  criticalPath?: string[]; // 关键路径节点 ID
}

interface Asset {
  id: number;
  name: string;
  ip: string;
  tags: string[];
}

interface WorkflowsViewProps {
  workflows: Workflow[];
  selectedWorkflowIdx: number;
  setSelectedWorkflowIdx: (idx: number) => void;
  assets: Asset[];
  selectedAssets: number[];
  setSelectedAssets: (ids: number[]) => void;
  onApplyWorkflow: (workflowId: number) => void;
  onCreateWorkflow: () => void;
  focusPanel: 'left' | 'center' | 'right';
  setFocusPanel: (panel: 'left' | 'center' | 'right') => void;
}

// 扩展的工作流数据（包含层级和依赖）
const EXTENDED_WORKFLOWS: Workflow[] = [
  // === 原子工作流 ===
  {
    id: 101,
    name: '扫描单个端口',
    description: '扫描指定 IP 的指定端口是否开放',
    level: 'atomic',
    category: 'Network Scan',
    status: 'active',
    nodes: [
      {
        id: 'n1',
        type: 'scan',
        label: 'Port Scan',
        inputs: [
          { name: 'target_ip', type: 'ip', required: true },
          { name: 'port', type: 'port', required: true },
        ],
        outputs: [
          { name: 'port_status', type: 'data', required: true },
          { name: 'service_info', type: 'data', required: false },
        ],
        estimatedDuration: 2,
      },
    ],
    edges: [],
    appliedTo: [],
    runs: 1247,
    lastRun: '5分钟前',
    successRate: 98,
    avgDuration: '1.8s',
    estimatedDuration: 2,
    tags: ['atomic', 'network', 'port'],
    maxParallelism: 1,
  },
  {
    id: 102,
    name: 'Web 服务联通性测试',
    description: '测试 Web 服务是否可访问',
    level: 'atomic',
    category: 'Web Security',
    status: 'active',
    nodes: [
      {
        id: 'n1',
        type: 'validate',
        label: 'HTTP Check',
        inputs: [
          { name: 'url', type: 'url', required: true },
        ],
        outputs: [
          { name: 'http_status', type: 'data', required: true },
          { name: 'headers', type: 'data', required: false },
        ],
        estimatedDuration: 3,
      },
    ],
    edges: [],
    appliedTo: [],
    runs: 892,
    lastRun: '10分钟前',
    successRate: 95,
    avgDuration: '2.5s',
    estimatedDuration: 3,
    tags: ['atomic', 'web', 'http'],
    maxParallelism: 1,
  },
  {
    id: 103,
    name: 'SQL 注入测试',
    description: '对指定 URL 进行 SQL 注入漏洞测试',
    level: 'atomic',
    category: 'Web Security',
    status: 'active',
    nodes: [
      {
        id: 'n1',
        type: 'exploit',
        label: 'SQLi Test',
        inputs: [
          { name: 'url', type: 'url', required: true },
        ],
        outputs: [
          { name: 'vulnerability', type: 'vulnerability', required: false },
        ],
        estimatedDuration: 45,
      },
    ],
    edges: [],
    appliedTo: [],
    runs: 456,
    lastRun: '1小时前',
    successRate: 87,
    avgDuration: '42s',
    estimatedDuration: 45,
    tags: ['atomic', 'web', 'sqli'],
    maxParallelism: 1,
  },
  
  // === 组合工作流 ===
  {
    id: 201,
    name: 'Web 端口发现与测试',
    description: '扫描常见 Web 端口并测试服务',
    level: 'composite',
    category: 'Web Security',
    status: 'active',
    nodes: [
      { 
        id: 'n1', 
        type: 'scan', 
        label: '扫描 80 端口',
        atomicWorkflowId: '101',
        inputs: [{ name: 'target_ip', type: 'ip', required: true }, { name: 'port', type: 'port', required: true }],
        outputs: [{ name: 'port_status', type: 'data', required: true }],
        estimatedDuration: 2,
        position: { x: 50, y: 100 },
      },
      { 
        id: 'n2', 
        type: 'scan', 
        label: '扫描 443 端口',
        atomicWorkflowId: '101',
        inputs: [{ name: 'target_ip', type: 'ip', required: true }, { name: 'port', type: 'port', required: true }],
        outputs: [{ name: 'port_status', type: 'data', required: true }],
        estimatedDuration: 2,
        position: { x: 50, y: 200 },
      },
      { 
        id: 'n3', 
        type: 'scan', 
        label: '扫描 8080 端口',
        atomicWorkflowId: '101',
        inputs: [{ name: 'target_ip', type: 'ip', required: true }, { name: 'port', type: 'port', required: true }],
        outputs: [{ name: 'port_status', type: 'data', required: true }],
        estimatedDuration: 2,
        position: { x: 50, y: 300 },
      },
      { 
        id: 'n4', 
        type: 'validate', 
        label: 'Web 联通性测试',
        atomicWorkflowId: '102',
        inputs: [{ name: 'url', type: 'url', required: true }],
        outputs: [{ name: 'http_status', type: 'data', required: true }],
        estimatedDuration: 3,
        position: { x: 300, y: 150 },
      },
      { 
        id: 'n5', 
        type: 'exploit', 
        label: 'SQL 注入测试',
        atomicWorkflowId: '103',
        inputs: [{ name: 'url', type: 'url', required: true }],
        outputs: [{ name: 'vulnerability', type: 'vulnerability', required: false }],
        estimatedDuration: 45,
        position: { x: 550, y: 150 },
      },
      { 
        id: 'n6', 
        type: 'report', 
        label: '生成报告',
        inputs: [{ name: 'results', type: 'data', required: true }],
        outputs: [{ name: 'report', type: 'file', required: true }],
        estimatedDuration: 5,
        position: { x: 800, y: 200 },
      },
    ],
    edges: [
      { from: 'n1', to: 'n4', condition: 'port_open', conditionType: 'conditional' },
      { from: 'n2', to: 'n4', condition: 'port_open', conditionType: 'conditional' },
      { from: 'n3', to: 'n4', condition: 'port_open', conditionType: 'conditional' },
      { from: 'n4', to: 'n5', condition: 'http_200', conditionType: 'on_success' },
      { from: 'n5', to: 'n6', conditionType: 'always' },
    ],
    appliedTo: [1, 2],
    runs: 234,
    lastRun: '2小时前',
    successRate: 94,
    avgDuration: '52s',
    estimatedDuration: 52,
    tags: ['composite', 'web', 'scan'],
    maxParallelism: 3, // n1, n2, n3 可并行
    criticalPath: ['n1', 'n4', 'n5', 'n6'],
  },
  {
    id: 202,
    name: 'MAVLink 完整扫描流程',
    description: '针对无人机 MAVLink 协议的完整安全评估',
    level: 'composite',
    category: 'Protocol Analysis',
    status: 'active',
    nodes: [
      { 
        id: 'n1', 
        type: 'scan', 
        label: '端口扫描',
        inputs: [{ name: 'target_ip', type: 'ip', required: true }],
        outputs: [{ name: 'open_ports', type: 'data', required: true }],
        estimatedDuration: 10,
        position: { x: 50, y: 150 },
      },
      { 
        id: 'n2', 
        type: 'validate', 
        label: 'MAVLink 协议识别',
        inputs: [{ name: 'port', type: 'port', required: true }],
        outputs: [{ name: 'protocol_info', type: 'data', required: true }],
        estimatedDuration: 5,
        position: { x: 250, y: 150 },
      },
      { 
        id: 'n3', 
        type: 'fuzz', 
        label: 'MAVLink Fuzzing',
        inputs: [{ name: 'target', type: 'data', required: true }],
        outputs: [{ name: 'crashes', type: 'data', required: false }],
        estimatedDuration: 120,
        position: { x: 450, y: 100 },
      },
      { 
        id: 'n4', 
        type: 'exploit', 
        label: 'CVE 漏洞验证',
        inputs: [{ name: 'target', type: 'data', required: true }],
        outputs: [{ name: 'vulnerability', type: 'vulnerability', required: false }],
        estimatedDuration: 60,
        position: { x: 450, y: 200 },
      },
      { 
        id: 'n5', 
        type: 'report', 
        label: '安全报告',
        inputs: [{ name: 'results', type: 'data', required: true }],
        outputs: [{ name: 'report', type: 'file', required: true }],
        estimatedDuration: 10,
        position: { x: 700, y: 150 },
      },
    ],
    edges: [
      { from: 'n1', to: 'n2', conditionType: 'on_success' },
      { from: 'n2', to: 'n3', conditionType: 'on_success' },
      { from: 'n2', to: 'n4', conditionType: 'on_success' },
      { from: 'n3', to: 'n5', conditionType: 'always' },
      { from: 'n4', to: 'n5', conditionType: 'always' },
    ],
    appliedTo: [1],
    runs: 89,
    lastRun: '昨天',
    successRate: 76,
    avgDuration: '3m 25s',
    estimatedDuration: 205,
    tags: ['composite', 'mavlink', 'drone'],
    maxParallelism: 2, // n3 和 n4 可并行
    criticalPath: ['n1', 'n2', 'n3', 'n5'],
  },
  
  // === 任务级工作流 ===
  {
    id: 301,
    name: '渗透测试 - 无人机系统',
    description: '完整的无人机系统渗透测试任务',
    level: 'task',
    category: 'Full Assessment',
    status: 'active',
    nodes: [
      { 
        id: 'n1', 
        type: 'scan', 
        label: 'Web 端口发现',
        atomicWorkflowId: '201',
        inputs: [{ name: 'target_ip', type: 'ip', required: true }],
        outputs: [{ name: 'web_services', type: 'data', required: true }],
        estimatedDuration: 52,
        position: { x: 50, y: 100 },
      },
      { 
        id: 'n2', 
        type: 'scan', 
        label: 'MAVLink 扫描',
        atomicWorkflowId: '202',
        inputs: [{ name: 'target_ip', type: 'ip', required: true }],
        outputs: [{ name: 'protocol_vulns', type: 'data', required: true }],
        estimatedDuration: 205,
        position: { x: 50, y: 250 },
      },
      { 
        id: 'n3', 
        type: 'ai-analyze', 
        label: 'AI 综合分析',
        inputs: [{ name: 'scan_results', type: 'data', required: true }],
        outputs: [{ name: 'threat_analysis', type: 'data', required: true }],
        estimatedDuration: 180,
        position: { x: 350, y: 175 },
      },
      { 
        id: 'n4', 
        type: 'report', 
        label: '渗透测试报告',
        inputs: [{ name: 'all_results', type: 'data', required: true }],
        outputs: [{ name: 'final_report', type: 'file', required: true }],
        estimatedDuration: 30,
        position: { x: 600, y: 175 },
      },
    ],
    edges: [
      { from: 'n1', to: 'n3', conditionType: 'always' },
      { from: 'n2', to: 'n3', conditionType: 'always' },
      { from: 'n3', to: 'n4', conditionType: 'on_success' },
    ],
    appliedTo: [1, 2, 3],
    runs: 45,
    lastRun: '3天前',
    successRate: 82,
    avgDuration: '8m 15s',
    estimatedDuration: 495,
    tags: ['task', 'full-assessment', 'drone'],
    maxParallelism: 2, // n1 和 n2 可并行
    criticalPath: ['n2', 'n3', 'n4'],
  },
];

export function WorkflowsView({
  workflows: _workflows,
  selectedWorkflowIdx,
  setSelectedWorkflowIdx,
  assets,
  selectedAssets,
  setSelectedAssets,
  onApplyWorkflow,
  onCreateWorkflow,
  focusPanel,
  setFocusPanel
}: WorkflowsViewProps) {
  const workflows = EXTENDED_WORKFLOWS;
  const [searchQuery, setSearchQuery] = useState('');
  const [filterLevel, setFilterLevel] = useState<string>('all');
  const [filterCategory, setFilterCategory] = useState<string>('all');
  const [expandedFolders, setExpandedFolders] = useState<Set<string>>(new Set(['atomic', 'composite', 'task']));
  const [editMode, setEditMode] = useState(false);
  const [showExecutionStats, setShowExecutionStats] = useState(true);

  const currentWorkflow = workflows[selectedWorkflowIdx] || workflows[0];

  // 按层级分组工作流
  const groupedWorkflows = {
    atomic: workflows.filter(w => w.level === 'atomic'),
    composite: workflows.filter(w => w.level === 'composite'),
    task: workflows.filter(w => w.level === 'task'),
  };

  // 过滤工作流
  const filterWorkflows = (wfs: Workflow[]) => {
    return wfs.filter(wf => {
      const matchSearch = wf.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
                         wf.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
                         wf.tags.some(tag => tag.toLowerCase().includes(searchQuery.toLowerCase()));
      const matchLevel = filterLevel === 'all' || wf.level === filterLevel;
      const matchCategory = filterCategory === 'all' || wf.category === filterCategory;
      return matchSearch && matchLevel && matchCategory;
    });
  };

  const categories = Array.from(new Set(workflows.map(wf => wf.category)));

  const toggleFolder = (folder: string) => {
    const newExpanded = new Set(expandedFolders);
    if (newExpanded.has(folder)) {
      newExpanded.delete(folder);
    } else {
      newExpanded.add(folder);
    }
    setExpandedFolders(newExpanded);
  };

  const getLevelIcon = (level: WorkflowLevel) => {
    switch (level) {
      case 'atomic': return <Box className="w-3.5 h-3.5" />;
      case 'composite': return <Layers className="w-3.5 h-3.5" />;
      case 'task': return <GitBranch className="w-3.5 h-3.5" />;
    }
  };

  const getLevelColor = (level: WorkflowLevel) => {
    switch (level) {
      case 'atomic': return 'text-blue-600 bg-blue-50 border-blue-200';
      case 'composite': return 'text-purple-600 bg-purple-50 border-purple-200';
      case 'task': return 'text-emerald-600 bg-emerald-50 border-emerald-200';
    }
  };

  const getLevelName = (level: WorkflowLevel) => {
    switch (level) {
      case 'atomic': return '原子工作流';
      case 'composite': return '组合工作流';
      case 'task': return '任务';
    }
  };

  const getNodeColor = (type: string) => {
    switch (type) {
      case 'scan': return 'bg-blue-100 border-blue-300 text-blue-900';
      case 'exploit': return 'bg-red-100 border-red-300 text-red-900';
      case 'validate': return 'bg-emerald-100 border-emerald-300 text-emerald-900';
      case 'report': return 'bg-purple-100 border-purple-300 text-purple-900';
      case 'fuzz': return 'bg-yellow-100 border-yellow-300 text-yellow-900';
      case 'ai-analyze': return 'bg-pink-100 border-pink-300 text-pink-900';
      default: return 'bg-slate-100 border-slate-300 text-slate-900';
    }
  };

  const getEdgeColor = (conditionType?: string) => {
    switch (conditionType) {
      case 'always': return 'stroke-slate-400';
      case 'on_success': return 'stroke-emerald-500';
      case 'on_failure': return 'stroke-red-500';
      case 'conditional': return 'stroke-yellow-500';
      default: return 'stroke-slate-400';
    }
  };

  return (
    <div className="flex gap-3 h-full bg-[#FAFAFA] p-3">
      {/* Left Panel: Hierarchical Workflow Tree */}
      <div className="w-80 bg-white rounded-xl overflow-hidden shadow-sm border-2 border-slate-200 flex flex-col">
        {/* Header */}
        <div className="border-b border-slate-200 px-4 py-3 bg-slate-50">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-2">
              <GitBranch className="w-4 h-4 text-purple-600" />
              <h2 className="text-xs font-medium text-slate-800">工作流编排</h2>
            </div>
            <button
              onClick={onCreateWorkflow}
              className="px-2.5 py-1 bg-purple-600 hover:bg-purple-700 text-white rounded-lg text-xs transition-all font-medium"
            >
              + 新建
            </button>
          </div>

          {/* Search */}
          <div className="relative mb-2">
            <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-slate-400" />
            <input
              type="text"
              placeholder="搜索工作流..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full pl-8 pr-3 py-1.5 bg-white border border-slate-200 rounded-lg text-xs text-slate-700 placeholder:text-slate-400 focus:outline-none focus:ring-2 focus:ring-purple-500/20 focus:border-purple-400"
            />
          </div>

          {/* Filters */}
          <div className="flex gap-2">
            <select
              value={filterLevel}
              onChange={(e) => setFilterLevel(e.target.value)}
              className="flex-1 px-2 py-1 bg-white border border-slate-200 rounded text-[10px] text-slate-700 focus:outline-none focus:ring-2 focus:ring-purple-500/20"
            >
              <option value="all">所有层级</option>
              <option value="atomic">原子</option>
              <option value="composite">组合</option>
              <option value="task">任务</option>
            </select>
            <select
              value={filterCategory}
              onChange={(e) => setFilterCategory(e.target.value)}
              className="flex-1 px-2 py-1 bg-white border border-slate-200 rounded text-[10px] text-slate-700 focus:outline-none focus:ring-2 focus:ring-purple-500/20"
            >
              <option value="all">所有类别</option>
              {categories.map(cat => (
                <option key={cat} value={cat}>{cat}</option>
              ))}
            </select>
          </div>
        </div>

        {/* Hierarchical Tree */}
        <div className="flex-1 overflow-auto">
          {/* Atomic Workflows */}
          <div className="border-b border-slate-100">
            <div
              onClick={() => toggleFolder('atomic')}
              className="px-3 py-2 bg-slate-50 border-b border-slate-200 cursor-pointer hover:bg-slate-100 transition-colors flex items-center justify-between"
            >
              <div className="flex items-center gap-2">
                {expandedFolders.has('atomic') ? (
                  <ChevronDown className="w-3.5 h-3.5 text-slate-600" />
                ) : (
                  <ChevronRight className="w-3.5 h-3.5 text-slate-600" />
                )}
                <Box className="w-3.5 h-3.5 text-blue-600" />
                <span className="text-xs font-medium text-slate-700">原子工作流</span>
              </div>
              <span className="text-[10px] text-slate-500 bg-slate-200 px-1.5 py-0.5 rounded">
                {filterWorkflows(groupedWorkflows.atomic).length}
              </span>
            </div>
            {expandedFolders.has('atomic') && (
              <div className="bg-white">
                {filterWorkflows(groupedWorkflows.atomic).map((wf) => (
                  <div
                    key={wf.id}
                    onClick={() => setSelectedWorkflowIdx(workflows.indexOf(wf))}
                    className={`px-4 py-2 border-b border-slate-50 cursor-pointer transition-all ${
                      currentWorkflow?.id === wf.id ? 'bg-blue-50 border-l-4 border-l-blue-500' : 'hover:bg-slate-50 border-l-4 border-l-transparent'
                    }`}
                  >
                    <div className="text-xs font-medium text-slate-800 mb-0.5">{wf.name}</div>
                    <div className="flex items-center gap-2 text-[9px] text-slate-500">
                      <span className="font-mono">{wf.avgDuration}</span>
                      <span>•</span>
                      <span className={wf.successRate >= 95 ? 'text-emerald-600 font-medium' : ''}>{wf.successRate}%</span>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>

          {/* Composite Workflows */}
          <div className="border-b border-slate-100">
            <div
              onClick={() => toggleFolder('composite')}
              className="px-3 py-2 bg-slate-50 border-b border-slate-200 cursor-pointer hover:bg-slate-100 transition-colors flex items-center justify-between"
            >
              <div className="flex items-center gap-2">
                {expandedFolders.has('composite') ? (
                  <ChevronDown className="w-3.5 h-3.5 text-slate-600" />
                ) : (
                  <ChevronRight className="w-3.5 h-3.5 text-slate-600" />
                )}
                <Layers className="w-3.5 h-3.5 text-purple-600" />
                <span className="text-xs font-medium text-slate-700">组合工作流</span>
              </div>
              <span className="text-[10px] text-slate-500 bg-slate-200 px-1.5 py-0.5 rounded">
                {filterWorkflows(groupedWorkflows.composite).length}
              </span>
            </div>
            {expandedFolders.has('composite') && (
              <div className="bg-white">
                {filterWorkflows(groupedWorkflows.composite).map((wf) => (
                  <div
                    key={wf.id}
                    onClick={() => setSelectedWorkflowIdx(workflows.indexOf(wf))}
                    className={`px-4 py-2 border-b border-slate-50 cursor-pointer transition-all ${
                      currentWorkflow?.id === wf.id ? 'bg-purple-50 border-l-4 border-l-purple-500' : 'hover:bg-slate-50 border-l-4 border-l-transparent'
                    }`}
                  >
                    <div className="flex items-center justify-between mb-0.5">
                      <div className="text-xs font-medium text-slate-800">{wf.name}</div>
                      {wf.maxParallelism && wf.maxParallelism > 1 && (
                        <span className="text-[9px] bg-orange-100 text-orange-700 px-1.5 py-0.5 rounded border border-orange-200 font-medium">
                          ∥{wf.maxParallelism}
                        </span>
                      )}
                    </div>
                    <div className="flex items-center gap-2 text-[9px] text-slate-500">
                      <span>{wf.nodes.length} 节点</span>
                      <span>•</span>
                      <span className="font-mono">{wf.avgDuration}</span>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>

          {/* Task Workflows */}
          <div>
            <div
              onClick={() => toggleFolder('task')}
              className="px-3 py-2 bg-slate-50 border-b border-slate-200 cursor-pointer hover:bg-slate-100 transition-colors flex items-center justify-between"
            >
              <div className="flex items-center gap-2">
                {expandedFolders.has('task') ? (
                  <ChevronDown className="w-3.5 h-3.5 text-slate-600" />
                ) : (
                  <ChevronRight className="w-3.5 h-3.5 text-slate-600" />
                )}
                <GitBranch className="w-3.5 h-3.5 text-emerald-600" />
                <span className="text-xs font-medium text-slate-700">任务工作流</span>
              </div>
              <span className="text-[10px] text-slate-500 bg-slate-200 px-1.5 py-0.5 rounded">
                {filterWorkflows(groupedWorkflows.task).length}
              </span>
            </div>
            {expandedFolders.has('task') && (
              <div className="bg-white">
                {filterWorkflows(groupedWorkflows.task).map((wf) => (
                  <div
                    key={wf.id}
                    onClick={() => setSelectedWorkflowIdx(workflows.indexOf(wf))}
                    className={`px-4 py-2 border-b border-slate-50 cursor-pointer transition-all ${
                      currentWorkflow?.id === wf.id ? 'bg-emerald-50 border-l-4 border-l-emerald-500' : 'hover:bg-slate-50 border-l-4 border-l-transparent'
                    }`}
                  >
                    <div className="text-xs font-medium text-slate-800 mb-0.5">{wf.name}</div>
                    <div className="flex items-center gap-2 text-[9px] text-slate-500 mb-1">
                      <span>{wf.nodes.length} 步骤</span>
                      <span>•</span>
                      <span className="font-mono">~{Math.floor(wf.estimatedDuration / 60)}m</span>
                    </div>
                    <div className="flex items-center gap-1">
                      {wf.appliedTo.slice(0, 3).map((assetId) => {
                        const asset = assets.find(a => a.id === assetId);
                        return asset ? (
                          <span key={assetId} className="text-[8px] bg-emerald-100 text-emerald-700 px-1.5 py-0.5 rounded border border-emerald-200">
                            {asset.name}
                          </span>
                        ) : null;
                      })}
                      {wf.appliedTo.length > 3 && (
                        <span className="text-[8px] text-slate-500">+{wf.appliedTo.length - 3}</span>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>

        {/* Stats Footer */}
        <div className="border-t border-slate-200 px-4 py-2 bg-slate-50">
          <div className="grid grid-cols-3 gap-2 text-[9px]">
            <div className="text-center">
              <div className="text-blue-600 font-bold">{groupedWorkflows.atomic.length}</div>
              <div className="text-slate-500">原子</div>
            </div>
            <div className="text-center">
              <div className="text-purple-600 font-bold">{groupedWorkflows.composite.length}</div>
              <div className="text-slate-500">组合</div>
            </div>
            <div className="text-center">
              <div className="text-emerald-600 font-bold">{groupedWorkflows.task.length}</div>
              <div className="text-slate-500">任务</div>
            </div>
          </div>
        </div>
      </div>

      {/* Right Panel: DAG Visualization & Details */}
      {currentWorkflow && (
        <div className="flex-1 bg-white rounded-xl overflow-hidden shadow-sm border-2 border-slate-200 flex flex-col">
          {/* Header */}
          <div className="border-b border-slate-200 px-4 py-3 bg-slate-50">
            <div className="flex items-center justify-between mb-2">
              <div className="flex items-center gap-3">
                <div className={`p-1.5 rounded-lg border ${getLevelColor(currentWorkflow.level)}`}>
                  {getLevelIcon(currentWorkflow.level)}
                </div>
                <div>
                  <div className="flex items-center gap-2">
                    <h3 className="text-xs font-bold text-slate-800">{currentWorkflow.name}</h3>
                    <span className={`px-2 py-0.5 rounded text-[9px] font-medium border ${getLevelColor(currentWorkflow.level)}`}>
                      {getLevelName(currentWorkflow.level)}
                    </span>
                  </div>
                  <div className="text-[10px] text-slate-500 mt-0.5">{currentWorkflow.description}</div>
                </div>
              </div>
              <div className="flex items-center gap-2">
                <button
                  onClick={() => setShowExecutionStats(!showExecutionStats)}
                  className={`p-2 rounded-lg transition-all ${
                    showExecutionStats ? 'bg-blue-100 text-blue-700' : 'hover:bg-slate-200 text-slate-600'
                  }`}
                >
                  <BarChart3 className="w-4 h-4" />
                </button>
                <button
                  onClick={() => setEditMode(!editMode)}
                  className={`p-2 rounded-lg transition-all ${
                    editMode ? 'bg-purple-100 text-purple-700' : 'hover:bg-slate-200 text-slate-600'
                  }`}
                >
                  <Edit3 className="w-4 h-4" />
                </button>
                <button className="p-2 hover:bg-slate-200 rounded-lg transition-all text-slate-600">
                  <Copy className="w-4 h-4" />
                </button>
              </div>
            </div>

            {/* Execution Stats */}
            {showExecutionStats && (
              <div className="grid grid-cols-5 gap-2">
                <div className="bg-white rounded-lg p-2 border border-slate-200">
                  <div className="text-[9px] text-slate-500 mb-0.5">节点数</div>
                  <div className="text-sm font-bold text-slate-900">{currentWorkflow.nodes.length}</div>
                </div>
                <div className="bg-white rounded-lg p-2 border border-slate-200">
                  <div className="text-[9px] text-slate-500 mb-0.5">最大并行</div>
                  <div className="text-sm font-bold text-orange-600">∥{currentWorkflow.maxParallelism || 1}</div>
                </div>
                <div className="bg-white rounded-lg p-2 border border-slate-200">
                  <div className="text-[9px] text-slate-500 mb-0.5">预计耗时</div>
                  <div className="text-xs font-bold text-blue-600 font-mono">
                    {currentWorkflow.estimatedDuration >= 60 
                      ? `${Math.floor(currentWorkflow.estimatedDuration / 60)}m` 
                      : `${currentWorkflow.estimatedDuration}s`}
                  </div>
                </div>
                <div className="bg-white rounded-lg p-2 border border-slate-200">
                  <div className="text-[9px] text-slate-500 mb-0.5">成功率</div>
                  <div className={`text-sm font-bold ${
                    currentWorkflow.successRate >= 90 ? 'text-emerald-600' :
                    currentWorkflow.successRate >= 70 ? 'text-yellow-600' : 'text-red-600'
                  }`}>
                    {currentWorkflow.successRate}%
                  </div>
                </div>
                <div className="bg-white rounded-lg p-2 border border-slate-200">
                  <div className="text-[9px] text-slate-500 mb-0.5">运行次数</div>
                  <div className="text-sm font-bold text-slate-900">{currentWorkflow.runs}</div>
                </div>
              </div>
            )}
          </div>

          {/* Content: DAG Visualization */}
          <div className="flex-1 overflow-auto p-4 bg-slate-50">
            <div className="mb-3 flex items-center justify-between">
              <div className="text-xs text-slate-600 font-medium">依赖关系图 (DAG)</div>
              {editMode && (
                <button className="text-xs text-purple-600 hover:text-purple-700 font-medium flex items-center gap-1">
                  <Plus className="w-3 h-3" />
                  添加节点
                </button>
              )}
            </div>

            {/* DAG Visualization Area */}
            <div className="bg-white rounded-lg border-2 border-slate-200 p-6 min-h-[400px] relative">
              {/* SVG Canvas for Edges */}
              <svg className="absolute inset-0 w-full h-full pointer-events-none" style={{ zIndex: 0 }}>
                {currentWorkflow.edges.map((edge, idx) => {
                  const fromNode = currentWorkflow.nodes.find(n => n.id === edge.from);
                  const toNode = currentWorkflow.nodes.find(n => n.id === edge.to);
                  if (!fromNode?.position || !toNode?.position) return null;

                  const x1 = fromNode.position.x + 60;
                  const y1 = fromNode.position.y + 25;
                  const x2 = toNode.position.x;
                  const y2 = toNode.position.y + 25;

                  // Calculate control points for curved line
                  const midX = (x1 + x2) / 2;

                  return (
                    <g key={idx}>
                      <path
                        d={`M ${x1} ${y1} Q ${midX} ${y1}, ${midX} ${(y1 + y2) / 2} T ${x2} ${y2}`}
                        fill="none"
                        className={getEdgeColor(edge.conditionType)}
                        strokeWidth="2"
                        markerEnd="url(#arrowhead)"
                      />
                      {edge.condition && (
                        <text
                          x={midX}
                          y={(y1 + y2) / 2 - 5}
                          className="text-[9px] fill-slate-600"
                          textAnchor="middle"
                        >
                          {edge.condition}
                        </text>
                      )}
                    </g>
                  );
                })}
                <defs>
                  <marker id="arrowhead" markerWidth="10" markerHeight="10" refX="9" refY="3" orient="auto">
                    <polygon points="0 0, 10 3, 0 6" className="fill-slate-400" />
                  </marker>
                </defs>
              </svg>

              {/* Nodes */}
              {currentWorkflow.nodes.map((node) => (
                <div
                  key={node.id}
                  className={`absolute rounded-lg p-2.5 border-2 min-w-[120px] shadow-sm transition-all ${getNodeColor(node.type)} ${
                    editMode ? 'cursor-move hover:shadow-lg' : ''
                  } ${
                    currentWorkflow.criticalPath?.includes(node.id) ? 'ring-2 ring-red-400 ring-offset-2' : ''
                  }`}
                  style={{
                    left: node.position?.x || 0,
                    top: node.position?.y || 0,
                    zIndex: 1,
                  }}
                >
                  <div className="text-xs font-bold mb-1">{node.label}</div>
                  <div className="text-[9px] opacity-70 mb-1">{node.type}</div>
                  <div className="flex items-center gap-1 text-[8px] opacity-60">
                    <Timer className="w-2.5 h-2.5" />
                    <span>{node.estimatedDuration}s</span>
                  </div>
                  {node.atomicWorkflowId && (
                    <div className="mt-1 text-[8px] bg-white/50 px-1.5 py-0.5 rounded border border-current">
                      ID: {node.atomicWorkflowId}
                    </div>
                  )}
                  {editMode && (
                    <div className="flex gap-1 mt-2">
                      <button className="flex-1 px-2 py-0.5 bg-white/70 hover:bg-white rounded text-[9px] font-medium">
                        编辑
                      </button>
                      <button className="px-2 py-0.5 bg-red-100/70 hover:bg-red-100 rounded text-[9px] font-medium text-red-700">
                        删
                      </button>
                    </div>
                  )}
                </div>
              ))}

              {/* Critical Path Indicator */}
              {currentWorkflow.criticalPath && currentWorkflow.criticalPath.length > 0 && (
                <div className="absolute bottom-4 right-4 bg-red-50 border-2 border-red-300 rounded-lg p-2">
                  <div className="flex items-center gap-1.5 text-[10px] text-red-700 font-medium">
                    <AlertTriangle className="w-3 h-3" />
                    关键路径 ({currentWorkflow.criticalPath.length} 节点)
                  </div>
                </div>
              )}
            </div>

            {/* Legend */}
            <div className="mt-3 flex flex-wrap gap-3 text-[10px]">
              <div className="flex items-center gap-1.5">
                <div className="w-8 h-0.5 bg-slate-400" />
                <span className="text-slate-600">总是执行</span>
              </div>
              <div className="flex items-center gap-1.5">
                <div className="w-8 h-0.5 bg-emerald-500" />
                <span className="text-slate-600">成功时</span>
              </div>
              <div className="flex items-center gap-1.5">
                <div className="w-8 h-0.5 bg-yellow-500" />
                <span className="text-slate-600">条件</span>
              </div>
              <div className="flex items-center gap-1.5">
                <div className="w-3 h-3 border-2 border-red-400 rounded" />
                <span className="text-slate-600">关键路径</span>
              </div>
            </div>
          </div>

          {/* Footer Actions */}
          <div className="border-t border-slate-200 px-4 py-3 bg-slate-50">
            <div className="flex gap-2">
              <button
                onClick={() => onApplyWorkflow(currentWorkflow.id)}
                className="flex-1 flex items-center justify-center gap-2 px-4 py-2 bg-purple-600 hover:bg-purple-700 text-white rounded-lg text-xs transition-all font-medium shadow-sm"
              >
                <Play className="w-3.5 h-3.5" />
                运行工作流
              </button>
              <button className="flex items-center justify-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-xs transition-all font-medium shadow-sm">
                <Zap className="w-3.5 h-3.5" />
                模拟测试
              </button>
              {editMode && (
                <button className="flex items-center justify-center gap-2 px-4 py-2 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg text-xs transition-all font-medium shadow-sm">
                  <Save className="w-3.5 h-3.5" />
                  保存
                </button>
              )}
              <button className="p-2 hover:bg-slate-200 rounded-lg transition-all text-red-600">
                <Trash2 className="w-4 h-4" />
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
