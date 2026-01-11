import { AlertTriangle, Activity, Brain, Search, Filter, Clock, TrendingUp, Zap, Target, ChevronRight, Play, Pause, SkipForward, Terminal, Signal, Lightbulb, Shield, Bug, Network, Cpu, Link2, Code2, FileText, Layout, List } from 'lucide-react';
import { useState } from 'react';
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "../ui/resizable";
import { TasksKanban, Task } from '../dashboard/TasksKanban';
import { AgentDetails } from '../dashboard/AgentDetails';
import { FindingsPanel, Vulnerability } from '../dashboard/FindingsPanel';

interface TrafficItem {
  id: number;
  time: string;
  method: string;
  protocol: string;
  path: string;
  status: number;
  size: string;
  duration: string;
  source: string;
  destination: string;
  hasAnomaly?: boolean;
}

interface TaskProgress {
  id: string;
  name: string;
  asset: string;
  progress: number;
  status: 'running' | 'paused' | 'completed' | 'error';
  currentStep: string;
  eta: string;
  aiAgentId: string;
}

interface AIAgentDetail {
  id: string;
  taskId: string;
  currentAction: string;
  thoughtProcess: string[];
  findings: string[];
  confidence: number;
  nextSteps: string[];
}

interface DashboardViewProps {
  focusPanel: 'left' | 'center' | 'right';
  setFocusPanel: (panel: 'left' | 'center' | 'right') => void;
  onViewVulnerability?: (vulnId: number) => void;
  onViewTraffic?: (trafficId: number) => void;
  onViewAIAnalysis?: (assetName: string) => void;
}

export function DashboardView({ 
  focusPanel, 
  setFocusPanel,
  onViewVulnerability,
  onViewTraffic,
  onViewAIAnalysis
}: DashboardViewProps) {
  const [trafficQL, setTrafficQL] = useState('');
  const [selectedTaskId, setSelectedTaskId] = useState<string | null>(null);
  const [selectedVuln, setSelectedVuln] = useState<number | null>(1);
  const [activeView, setActiveView] = useState<'kanban' | 'findings'>('kanban');
  const [dynamicTabs, setDynamicTabs] = useState<string[]>([]);

  // Kanban Tasks
  const kanbanTasks: Task[] = [
    { id: '1', title: 'Full Security Scan on Mavic 3', status: 'in-progress', tag: 'SCAN', priority: 'high' },
    { id: '2', title: 'Analyze Flight Logs', status: 'todo', tag: 'ANALYSIS', priority: 'medium' },
    { id: '3', title: 'Verify Weak Auth', status: 'done', tag: 'PENTEST', priority: 'low' },
    { id: '4', title: 'Generate PoC for CVE-2024-1234', status: 'in-progress', tag: 'EXPLOIT', priority: 'high' },
    { id: '5', title: 'Check Firmware Version', status: 'todo', tag: 'RECON', priority: 'low' },
  ];

  // Mock data - 漏洞列表
  const vulnerabilities: Vulnerability[] = [
    {
      id: 1,
      title: 'MAVLink Buffer Overflow',
      severity: 'critical',
      asset: 'DJI Mavic 3',
      protocol: 'MAVLink',
      cve: 'CVE-2024-1234',
      cvss: 9.8,
      detectedAt: '2m',
      status: 'confirmed',
      aiConfidence: 95
    },
    {
      id: 2,
      title: 'DJI Auth Bypass',
      severity: 'critical',
      asset: 'DJI Mavic 3',
      protocol: 'DJI',
      cvss: 9.1,
      detectedAt: '5m',
      status: 'validating',
      aiConfidence: 88
    },
    {
      id: 3,
      title: 'MySQL Default Creds',
      severity: 'high',
      asset: 'GCS Primary',
      protocol: 'MySQL',
      cvss: 8.7,
      detectedAt: '8m',
      status: 'confirmed',
      aiConfidence: 98
    },
    {
      id: 4,
      title: 'RTSP Stream Injection',
      severity: 'high',
      asset: 'DJI Mavic 3',
      protocol: 'RTSP',
      cvss: 7.8,
      detectedAt: '12m',
      status: 'new',
      aiConfidence: 76
    },
    {
      id: 5,
      title: 'Telnet Weak Auth',
      severity: 'medium',
      asset: 'GCS Backup',
      protocol: 'Telnet',
      cvss: 6.5,
      detectedAt: '15m',
      status: 'new',
      aiConfidence: 82
    }
  ];

  // Mock data - 实时流量（增加更多数据填充空间）
  const trafficItems: TrafficItem[] = [
    {
      id: 1,
      time: '14:32:45.234',
      method: 'MAVLINK',
      protocol: 'UDP',
      path: 'COMMAND_LONG',
      status: 200,
      size: '1.2KB',
      duration: '12ms',
      source: '192.168.1.100:14550',
      destination: '10.0.1.50:14550',
      hasAnomaly: true
    },
    {
      id: 2,
      time: '14:32:45.189',
      method: 'POST',
      protocol: 'HTTPS',
      path: '/api/telemetry/upload',
      status: 200,
      size: '45KB',
      duration: '156ms',
      source: '10.0.1.50:443',
      destination: '192.168.1.100:8080'
    },
    {
      id: 3,
      time: '14:32:44.876',
      method: 'GET',
      protocol: 'HTTP',
      path: '/api/video/stream',
      status: 200,
      size: '2.3MB',
      duration: '8ms',
      source: '192.168.1.100:8080',
      destination: '10.0.1.50:8080'
    },
    {
      id: 4,
      time: '14:32:44.654',
      method: 'MAVLINK',
      protocol: 'UDP',
      path: 'HEARTBEAT',
      status: 200,
      size: '256B',
      duration: '3ms',
      source: '192.168.1.100:14550',
      destination: '10.0.1.50:14550'
    },
    {
      id: 5,
      time: '14:32:44.432',
      method: 'GET',
      protocol: 'HTTP',
      path: '/api/status',
      status: 404,
      size: '128B',
      duration: '5ms',
      source: '192.168.1.101:8080',
      destination: '10.0.1.50:8080'
    },
    {
      id: 6,
      time: '14:32:44.234',
      method: 'MAVLINK',
      protocol: 'UDP',
      path: 'ATTITUDE',
      status: 200,
      size: '512B',
      duration: '4ms',
      source: '192.168.1.100:14550',
      destination: '10.0.1.50:14550'
    },
    {
      id: 7,
      time: '14:32:44.012',
      method: 'DJI',
      protocol: 'TCP',
      path: 'AUTH_REQUEST',
      status: 200,
      size: '892B',
      duration: '23ms',
      source: '192.168.1.100:8899',
      destination: '10.0.1.50:8899',
      hasAnomaly: true
    },
    {
      id: 8,
      time: '14:32:43.876',
      method: 'POST',
      protocol: 'HTTPS',
      path: '/api/firmware/check',
      status: 200,
      size: '12KB',
      duration: '89ms',
      source: '10.0.1.50:443',
      destination: '192.168.1.100:8080'
    },
    {
      id: 9,
      time: '14:32:43.654',
      method: 'MAVLINK',
      protocol: 'UDP',
      path: 'GPS_RAW_INT',
      status: 200,
      size: '384B',
      duration: '5ms',
      source: '192.168.1.100:14550',
      destination: '10.0.1.50:14550'
    },
    {
      id: 10,
      time: '14:32:43.432',
      method: 'GET',
      protocol: 'HTTP',
      path: '/api/battery/status',
      status: 200,
      size: '256B',
      duration: '7ms',
      source: '192.168.1.100:8080',
      destination: '10.0.1.50:8080'
    },
    {
      id: 11,
      time: '14:32:43.234',
      method: 'RTSP',
      protocol: 'TCP',
      path: 'SETUP rtsp://10.0.1.50/stream1',
      status: 200,
      size: '1.8KB',
      duration: '34ms',
      source: '192.168.1.100:554',
      destination: '10.0.1.50:554'
    },
    {
      id: 12,
      time: '14:32:43.012',
      method: 'MAVLINK',
      protocol: 'UDP',
      path: 'SYSTEM_TIME',
      status: 200,
      size: '192B',
      duration: '3ms',
      source: '192.168.1.100:14550',
      destination: '10.0.1.50:14550'
    },
    {
      id: 13,
      time: '14:32:42.876',
      method: 'POST',
      protocol: 'HTTPS',
      path: '/api/logs/upload',
      status: 200,
      size: '128KB',
      duration: '234ms',
      source: '10.0.1.50:443',
      destination: '192.168.1.100:8080'
    },
    {
      id: 14,
      time: '14:32:42.654',
      method: 'GET',
      protocol: 'HTTP',
      path: '/api/mission/current',
      status: 200,
      size: '2.1KB',
      duration: '12ms',
      source: '192.168.1.101:8080',
      destination: '10.0.1.50:8080'
    },
    {
      id: 15,
      time: '14:32:42.432',
      method: 'MAVLINK',
      protocol: 'UDP',
      path: 'RC_CHANNELS',
      status: 200,
      size: '448B',
      duration: '4ms',
      source: '192.168.1.100:14550',
      destination: '10.0.1.50:14550'
    },
    {
      id: 16,
      time: '14:32:42.234',
      method: 'DJI',
      protocol: 'TCP',
      path: 'GIMBAL_CONTROL',
      status: 200,
      size: '672B',
      duration: '18ms',
      source: '192.168.1.100:8899',
      destination: '10.0.1.50:8899'
    },
    {
      id: 17,
      time: '14:32:42.012',
      method: 'GET',
      protocol: 'HTTP',
      path: '/api/camera/settings',
      status: 200,
      size: '4.2KB',
      duration: '15ms',
      source: '192.168.1.100:8080',
      destination: '10.0.1.50:8080'
    },
    {
      id: 18,
      time: '14:32:41.876',
      method: 'MAVLINK',
      protocol: 'UDP',
      path: 'VFR_HUD',
      status: 200,
      size: '288B',
      duration: '4ms',
      source: '192.168.1.100:14550',
      destination: '10.0.1.50:14550'
    },
    {
      id: 19,
      time: '14:32:41.654',
      method: 'POST',
      protocol: 'HTTPS',
      path: '/api/telemetry/realtime',
      status: 200,
      size: '34KB',
      duration: '124ms',
      source: '10.0.1.50:443',
      destination: '192.168.1.100:8080'
    },
    {
      id: 20,
      time: '14:32:41.432',
      method: 'MAVLINK',
      protocol: 'UDP',
      path: 'MISSION_CURRENT',
      status: 200,
      size: '176B',
      duration: '3ms',
      source: '192.168.1.100:14550',
      destination: '10.0.1.50:14550'
    },
    {
      id: 21,
      time: '14:32:41.234',
      method: 'GET',
      protocol: 'HTTP',
      path: '/api/weather/data',
      status: 200,
      size: '1.5KB',
      duration: '45ms',
      source: '192.168.1.101:8080',
      destination: '10.0.1.50:8080'
    },
    {
      id: 22,
      time: '14:32:41.012',
      method: 'MAVLINK',
      protocol: 'UDP',
      path: 'BATTERY_STATUS',
      status: 200,
      size: '352B',
      duration: '5ms',
      source: '192.168.1.100:14550',
      destination: '10.0.1.50:14550'
    },
    {
      id: 23,
      time: '14:32:40.876',
      method: 'DJI',
      protocol: 'TCP',
      path: 'FLIGHT_MODE',
      status: 200,
      size: '544B',
      duration: '16ms',
      source: '192.168.1.100:8899',
      destination: '10.0.1.50:8899'
    },
    {
      id: 24,
      time: '14:32:40.654',
      method: 'POST',
      protocol: 'HTTPS',
      path: '/api/waypoint/update',
      status: 200,
      size: '8.4KB',
      duration: '67ms',
      source: '10.0.1.50:443',
      destination: '192.168.1.100:8080'
    },
    {
      id: 25,
      time: '14:32:40.432',
      method: 'MAVLINK',
      protocol: 'UDP',
      path: 'POSITION_TARGET',
      status: 200,
      size: '416B',
      duration: '4ms',
      source: '192.168.1.100:14550',
      destination: '10.0.1.50:14550'
    }
  ];

  // Mock data - 任务进度
  const tasks: TaskProgress[] = [
    {
      id: 'task-1',
      name: 'Full Security Scan',
      asset: 'DJI Mavic 3 Pro',
      progress: 67,
      status: 'running',
      currentStep: 'Fuzzing MAVLink Protocol',
      eta: '4m 32s',
      aiAgentId: 'agent-1'
    },
    {
      id: 'task-2',
      name: 'Credential Bruteforce',
      asset: 'GCS Primary Station',
      progress: 43,
      status: 'running',
      currentStep: 'Testing MySQL combinations',
      eta: '8m 15s',
      aiAgentId: 'agent-2'
    },
    {
      id: 'task-3',
      name: 'Network Enumeration',
      asset: 'Backup GCS',
      progress: 23,
      status: 'running',
      currentStep: 'Port scanning 234/65535',
      eta: '12m 00s',
      aiAgentId: 'agent-3'
    },
    {
      id: 'task-4',
      name: 'PoC Generation',
      asset: 'DJI Mavic 3 Pro',
      progress: 100,
      status: 'completed',
      currentStep: 'Completed',
      eta: '0s',
      aiAgentId: 'agent-4'
    }
  ];

  // Mock data - AI Agent 详情
  const aiAgentDetails: Record<string, AIAgentDetail> = {
    'agent-1': {
      id: 'agent-1',
      taskId: 'task-1',
      currentAction: 'Generating malformed MAVLink COMMAND_LONG packets',
      thoughtProcess: [
        'Analyzing MAVLink protocol specification v2.0',
        'Identified potential buffer in command parameter handling',
        'Crafting oversized payload to trigger overflow',
        'Testing with incremental sizes: 256, 512, 1024 bytes'
      ],
      findings: [
        '✓ Buffer overflow confirmed at 1024 bytes',
        '✓ Crash signature matches CVE-2024-1234 pattern',
        '⚠ Testing RCE payload delivery mechanism',
        '⧗ Analyzing crash dump for exploitation path'
      ],
      confidence: 92,
      nextSteps: [
        'Verify code execution capability',
        'Generate weaponized PoC',
        'Test against firmware v3.2.1, v3.3.0'
      ]
    },
    'agent-2': {
      id: 'agent-2',
      taskId: 'task-2',
      currentAction: 'Attempting MySQL authentication with default credentials',
      thoughtProcess: [
        'Enumerated MySQL service on port 3306',
        'Detected version: MySQL 5.7.42',
        'Loading credential dictionary: top 10000 combos',
        'Current progress: 4,321 / 10,000 attempts'
      ],
      findings: [
        '✓ Found exposed MySQL port without rate limiting',
        '⚠ Testing: root / toor, admin / admin123',
        '⧗ Response times indicate valid username "root"',
        '⧗ Brute force in progress...'
      ],
      confidence: 85,
      nextSteps: [
        'Continue credential testing',
        'If successful: enumerate databases',
        'Attempt privilege escalation'
      ]
    },
    'agent-3': {
      id: 'agent-3',
      taskId: 'task-3',
      currentAction: 'TCP SYN scan across all ports',
      thoughtProcess: [
        'Target IP: 10.0.1.25 (Backup GCS)',
        'Scanning strategy: Fast SYN scan',
        'Current rate: 500 ports/sec',
        'Open ports detected: 22, 80, 3306, 8080'
      ],
      findings: [
        '✓ SSH (22) - OpenSSH 7.4',
        '✓ HTTP (80, 8080) - nginx 1.18.0',
        '✓ MySQL (3306) - Accessible',
        '⧗ Continuing scan...'
      ],
      confidence: 0,
      nextSteps: [
        'Complete port enumeration',
        'Perform service version detection',
        'Check for known vulnerabilities'
      ]
    },
    'agent-4': {
      id: 'agent-4',
      taskId: 'task-4',
      currentAction: 'PoC generation completed',
      thoughtProcess: [
        'Analyzed vulnerability CVE-2024-1234',
        'Crafted Python exploitation script',
        'Added automatic payload generation',
        'Included documentation and usage examples'
      ],
      findings: [
        '✓ PoC successfully tests buffer overflow',
        '✓ Achieves code execution with 95% success rate',
        '✓ Generated detailed report with remediation steps',
        '✓ Saved to: /poc/mavlink_buffer_overflow.py'
      ],
      confidence: 98,
      nextSteps: []
    }
  };

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case 'critical': return 'text-red-600';
      case 'high': return 'text-orange-600';
      case 'medium': return 'text-yellow-600';
      default: return 'text-blue-600';
    }
  };

  const getTaskStatusIcon = (status: string) => {
    switch (status) {
      case 'running': return <Play className="w-3 h-3 text-purple-600" />;
      case 'paused': return <Pause className="w-3 h-3 text-yellow-500" />;
      case 'completed': return <Target className="w-3 h-3 text-emerald-500" />;
      case 'error': return <AlertTriangle className="w-3 h-3 text-red-500" />;
      default: return null;
    }
  };

  const filteredTraffic = trafficQL 
    ? trafficItems.filter(t => 
        t.method.toLowerCase().includes(trafficQL.toLowerCase()) ||
        t.protocol.toLowerCase().includes(trafficQL.toLowerCase()) ||
        t.path.toLowerCase().includes(trafficQL.toLowerCase())
      )
    : trafficItems;

  return (
    <div className="flex-1 flex overflow-hidden bg-[#FAFAFA]">
      {/* Main Content Area */}
      <div className="flex-1 flex flex-col overflow-hidden">
        
        {/* Top Toolbar / View Toggle */}
        <div className="bg-[#F3F4F6] border-b border-slate-200 px-3 py-1.5 flex items-center gap-2">
           <div className="flex items-center bg-white p-0.5 rounded-md border border-slate-200 shadow-sm">
             <button
               onClick={() => setActiveView('kanban')}
               className={`px-3 py-1 rounded-sm text-[10px] font-medium transition-all flex items-center gap-1.5 ${
                 activeView === 'kanban' 
                   ? 'bg-purple-100 text-purple-700 shadow-sm' 
                   : 'text-slate-500 hover:text-slate-800'
               }`}
             >
               <Layout className="w-3 h-3" />
               Mission Control
             </button>
             <button
               onClick={() => setActiveView('findings')}
               className={`px-3 py-1 rounded-sm text-[10px] font-medium transition-all flex items-center gap-1.5 ${
                 activeView === 'findings' 
                   ? 'bg-purple-100 text-purple-700 shadow-sm' 
                   : 'text-slate-500 hover:text-slate-800'
               }`}
             >
               <Shield className="w-3 h-3" />
               Findings
               <span className="ml-1 px-1 rounded-full bg-slate-100 text-[8px] text-slate-600 border border-slate-200">
                 {vulnerabilities.length}
               </span>
             </button>
           </div>
        </div>

        {/* Main Content Area - Takes full height */}
        <div className="flex-1 flex overflow-hidden min-h-0">
          <ResizablePanelGroup direction="horizontal">
            <ResizablePanel defaultSize={selectedTaskId && activeView === 'kanban' ? 65 : 100} minSize={30}>
              {activeView === 'kanban' ? (
                <TasksKanban tasks={kanbanTasks} selectedTaskId={selectedTaskId || ''} onTaskClick={setSelectedTaskId} />
              ) : (
                <FindingsPanel vulnerabilities={vulnerabilities} />
              )}
            </ResizablePanel>
            {selectedTaskId && activeView === 'kanban' && (
              <>
                <ResizableHandle withHandle className="bg-slate-200" />
                <ResizablePanel defaultSize={35} minSize={20}>
                  <AgentDetails 
                    taskId={selectedTaskId} 
                    taskTitle={kanbanTasks.find(t => t.id === selectedTaskId)?.title || 'Task'} 
                    status={kanbanTasks.find(t => t.id === selectedTaskId)?.status === 'in-progress' ? 'running' : 'paused'}
                    onClose={() => setSelectedTaskId(null)}
                  />
                </ResizablePanel>
              </>
            )}
          </ResizablePanelGroup>
        </div>
      </div>
    </div>
  );
}