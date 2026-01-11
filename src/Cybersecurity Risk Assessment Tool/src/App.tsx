import { useState, useEffect } from 'react';
import { 
  Shield, Target, Activity, Bug, Network, Cpu, Database, Settings, Zap, Plus, Search, 
  Play, Pause, Send, Filter, X, Trash2, Edit3, Copy, Download, RefreshCw, 
  AlertCircle, CheckCircle, XCircle, Clock, Terminal, ChevronRight, Save, Radio, Box
} from 'lucide-react';
import { TopBar } from './components/TopBar';
import { BottomBar } from './components/BottomBar';
import { DashboardView } from './components/views/DashboardView';
import { AssetsView } from './components/views/AssetsView';
import { ImagesView } from './components/views/ImagesView';
import { VulnerabilitiesView } from './components/views/VulnerabilitiesView';
import { TrafficView } from './components/views/TrafficView';
import { WorkflowsView } from './components/views/WorkflowsView';
import { DevicesView } from './components/views/DevicesView';
import { SettingsView } from './components/dashboard/SettingsView';
import { AIAssistantPanel } from './components/AIAssistantPanel';
import { KeyboardShortcutsHelp } from './components/KeyboardShortcutsHelp';
import { ExportDialog } from './components/ExportDialog';
import { Sparkline } from './components/MiniChart';
import { AIAnalysisViewer } from './components/AIAnalysisViewer';
import { AIConfigPanel } from './components/AIConfigPanel';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription } from './components/ui/dialog';
import { Input } from './components/ui/input';
import { Label } from './components/ui/label';
import { Checkbox } from './components/ui/checkbox';
import { Textarea } from './components/ui/textarea';
import { toast } from 'sonner@2.0.3';
import { Toaster } from './components/ui/sonner';

type ViewMode = 'dashboard' | 'assets' | 'images' | 'vulns' | 'traffic' | 'workflows' | 'devices' | 'settings';
type FocusPanel = 'left' | 'center' | 'right';
type ScanStatus = 'idle' | 'running' | 'paused' | 'completed' | 'failed';
type InterceptMode = 'off' | 'intercept';

interface Asset {
  id: number;
  name: string;
  ip: string;
  ports: number[];
  protocol: string;
  status: 'online' | 'offline' | 'scanning';
  scanProgress: number;
  scanPhase: string;
  vulns: number;
  risk: number;
  lastScan: string;
  services: string[];
  tags: string[];
  zone?: 'Z1' | 'Z2' | 'Z3' | 'Z4' | 'Z5';
  credentials?: {
    type: string;
    username?: string;
    status: 'valid' | 'expired' | 'weak' | 'unknown';
  };
  businessPurpose?: string;
  owner?: string;
  compliance?: string[];
}

interface ScanJob {
  id: number;
  name: string;
  assets: number[];
  status: ScanStatus;
  progress: number;
  startTime: string;
  endTime?: string;
  currentAsset?: number;
  logs: Array<{
    time: string;
    level: 'info' | 'success' | 'warning' | 'error';
    message: string;
  }>;
  results: {
    vulnsFound: number;
    portsScanned: number;
    servicesDetected: number;
  };
}

interface Vulnerability {
  id: string;
  severity: 'CRITICAL' | 'HIGH' | 'MEDIUM' | 'LOW';
  title: string;
  description: string;
  asset: string;
  assetId: number;
  cvss: number;
  aiScore: number;
  exploitability: number;
  impact: number;
  verified: boolean;
  pocAvailable: boolean;
  pocRequest: {
    method: string;
    path: string;
    headers: Record<string, string>;
    body: string;
    protocol: string;
  };
  mitreIds: string[];
  recommendation: string;
  detectedAt: string;
  cwe: string;
  affectedComponent: string;
}

interface TrafficPacket {
  id: number;
  time: string;
  src: string;
  dst: string;
  protocol: string;
  method: string;
  path: string;
  status: number;
  size: number;
  duration: string;
  anomaly: boolean;
  intercepted: boolean;
  request: string;
  response: string;
  vulnId?: string;
  assetId: number;
}

interface Workflow {
  id: number;
  name: string;
  description: string;
  icon: string;
  trafficQL: string;
  steps: Array<{
    type: 'request' | 'fuzz' | 'validate';
    config: any;
  }>;
  appliedTo: number[];
  runs: number;
  lastRun: string;
  successRate: number;
}

export default function App() {
  const [view, setView] = useState<ViewMode>('dashboard');
  const [focusPanel, setFocusPanel] = useState<FocusPanel>('left');
  const [currentTime, setCurrentTime] = useState(new Date());
  
  // Dialogs
  const [showNewAssetDialog, setShowNewAssetDialog] = useState(false);
  const [showEditAssetDialog, setShowEditAssetDialog] = useState(false);
  const [showScanConfigDialog, setShowScanConfigDialog] = useState(false);
  const [showFuzzDialog, setShowFuzzDialog] = useState(false);
  const [showSaveWorkflowDialog, setShowSaveWorkflowDialog] = useState(false);
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);
  const [showKeyboardHelp, setShowKeyboardHelp] = useState(false);
  const [showExportDialog, setShowExportDialog] = useState(false);
  const [showAIAnalysis, setShowAIAnalysis] = useState(false);
  const [showAIConfig, setShowAIConfig] = useState(false);
  const [exportData, setExportData] = useState<any>(null);
  const [exportType, setExportType] = useState<'assets' | 'vulnerabilities' | 'traffic' | 'scan-results' | 'full-report'>('assets');
  const [assetToDelete, setAssetToDelete] = useState<number | null>(null);
  const [assetToEdit, setAssetToEdit] = useState<Asset | null>(null);
  const [analysisAsset, setAnalysisAsset] = useState<Asset | null>(null);
  
  // States
  const [selectedAssets, setSelectedAssets] = useState<number[]>([]);
  const [selectedVulnIdx, setSelectedVulnIdx] = useState(0);
  const [selectedTrafficIdx, setSelectedTrafficIdx] = useState(0);
  const [selectedScanJobIdx, setSelectedScanJobIdx] = useState(0);
  const [selectedWorkflowIdx, setSelectedWorkflowIdx] = useState(0);
  
  // Traffic states
  const [trafficQL, setTrafficQL] = useState('');
  const [trafficCapturing, setTrafficCapturing] = useState(true);
  const [interceptMode, setInterceptMode] = useState<InterceptMode>('off');
  const [editingRequest, setEditingRequest] = useState(false);
  const [editedRequest, setEditedRequest] = useState('');
  
  // PoC editing
  const [editingPoc, setEditingPoc] = useState(false);
  const [pocBody, setPocBody] = useState('');
  
  // Filters
  const [vulnGroupBy, setVulnGroupBy] = useState<'severity' | 'asset' | 'mitre'>('severity');
  const [assetSearchQuery, setAssetSearchQuery] = useState('');
  const [vulnSearchQuery, setVulnSearchQuery] = useState('');
  
  // Forms
  const [newAsset, setNewAsset] = useState({
    name: '', ip: '', ports: '80,443,8080', protocol: 'HTTP', tags: ''
  });
  
  const [editAsset, setEditAsset] = useState({
    id: 0, name: '', ip: '', ports: '', protocol: 'HTTP', tags: ''
  });
  
  const [scanConfig, setScanConfig] = useState({
    mode: 'full' as 'quick' | 'full' | 'custom',
    modules: ['port_scan', 'service_detect', 'vuln_analysis', 'ai_poc_gen'],
    threads: 4,
    rateLimit: 100,
    aiEnabled: true,
  });
  
  const [fuzzConfig, setFuzzConfig] = useState({
    target: 'body',
    payloadType: 'sqli',
    threads: 10,
    delay: 100,
    customPayloads: ''
  });

  const [workflowForm, setWorkflowForm] = useState({
    name: '',
    description: '',
    icon: '🔥'
  });

  const [aiConfig, setAIConfig] = useState({
    provider: 'local' as 'local' | 'openai' | 'anthropic' | 'custom',
    model: 'qwen-2.5-72b',
    apiKey: '',
    endpoint: '',
    temperature: 0.7,
    maxTokens: 4096,
    enableAutoScan: true,
    enablePoCGeneration: true,
    enableTrafficAnalysis: true,
    aggressiveness: 6
  });

  useEffect(() => {
    const interval = setInterval(() => setCurrentTime(new Date()), 1000);
    return () => clearInterval(interval);
  }, []);

  // Mock Data
  const [assets, setAssets] = useState<Asset[]>([
    { 
      id: 1, name: 'DJI Mavic 3 Pro', ip: '192.168.1.100', 
      ports: [22, 80, 8080, 14550], protocol: 'DJI', 
      status: 'scanning', scanProgress: 67, scanPhase: 'AI PoC Generation',
      vulns: 3, risk: 95, lastScan: '2m ago', 
      services: ['SSH', 'HTTP', 'DJI-API', 'Telemetry'],
      tags: ['drone', 'critical'],
      zone: 'Z4',
      credentials: { type: 'SSH Key', username: 'root', status: 'weak' },
      businessPurpose: '侦察与监视任务',
      owner: '无人机作战小组 Alpha',
      compliance: ['MIL-STD-882E', 'DO-178C']
    },
    { 
      id: 2, name: 'GCS Primary Station', ip: '10.0.1.50',
      ports: [22, 443, 3306, 8080], protocol: 'HTTPS',
      status: 'online', scanProgress: 100, scanPhase: 'Completed',
      vulns: 1, risk: 35, lastScan: '2m ago',
      services: ['HTTPS', 'MySQL', 'API', 'SSH'],
      tags: ['gcs', 'low'],
      zone: 'Z1',
      credentials: { type: 'Certificate + Password', username: 'admin', status: 'valid' },
      businessPurpose: '地面指挥控制中心',
      owner: '指挥部运维团队',
      compliance: ['ISO 27001', 'NIST 800-53']
    },
    { 
      id: 3, name: 'Flight Controller', ip: '192.168.1.101',
      ports: [14550, 14551, 5760], protocol: 'MAVLink',
      status: 'online', scanProgress: 100, scanPhase: 'Completed',
      vulns: 0, risk: 25, lastScan: '5m ago',
      services: ['MAVLink', 'Telemetry', 'UDP'],
      tags: ['drone', 'low'],
      zone: 'Z4',
      credentials: { type: 'MAVLink Token', status: 'valid' },
      businessPurpose: '无人机飞行控制',
      owner: '飞控设备组',
      compliance: ['DO-178C', 'ARP4754A']
    },
    { 
      id: 4, name: 'Telemetry Server', ip: '10.0.1.51',
      ports: [8000, 8001, 9000], protocol: 'TCP',
      status: 'online', scanProgress: 100, scanPhase: 'Completed',
      vulns: 0, risk: 20, lastScan: '10m ago',
      services: ['TCP', 'HTTP', 'WebSocket'],
      tags: ['server', 'low'],
      zone: 'Z2',
      credentials: { type: 'API Key', status: 'valid' },
      businessPurpose: '遥测数据聚合处理',
      owner: '通信网关团队',
      compliance: ['ISO 27001']
    },
    {
      id: 5, name: 'Mission Control Server', ip: '10.0.1.52',
      ports: [443, 8080, 9090], protocol: 'HTTPS',
      status: 'online', scanProgress: 100, scanPhase: 'Completed',
      vulns: 2, risk: 55, lastScan: '3m ago',
      services: ['HTTPS', 'REST API', 'WebSocket'],
      tags: ['control', 'medium'],
      zone: 'Z3',
      credentials: { type: 'OAuth2 + MFA', username: 'mission_ctrl', status: 'valid' },
      businessPurpose: '任务规划与执行控制',
      owner: '任务控制中心',
      compliance: ['NIST 800-53', 'ISO 27001']
    },
    {
      id: 6, name: 'Emergency System', ip: '10.0.2.10',
      ports: [14550, 5760], protocol: 'MAVLink',
      status: 'online', scanProgress: 100, scanPhase: 'Completed',
      vulns: 0, risk: 15, lastScan: '1m ago',
      services: ['Emergency Protocol', 'Failsafe'],
      tags: ['safety', 'critical'],
      zone: 'Z5',
      credentials: { type: 'Hardware Token', status: 'valid' },
      businessPurpose: '紧急故障安全系统',
      owner: '安全保障组',
      compliance: ['MIL-STD-882E', 'DO-178C', 'ARP4761']
    },
    {
      id: 7, name: 'Data Gateway', ip: '10.0.1.60',
      ports: [443, 8443, 9443], protocol: 'HTTPS',
      status: 'online', scanProgress: 100, scanPhase: 'Completed',
      vulns: 0, risk: 30, lastScan: '8m ago',
      services: ['TLS Gateway', 'API Router', 'Load Balancer'],
      tags: ['network', 'low'],
      zone: 'Z2',
      credentials: { type: 'Certificate', status: 'valid' },
      businessPurpose: '数据路由与加密网关',
      owner: '网络安全团队',
      compliance: ['ISO 27001', 'PCI DSS']
    },
    {
      id: 8, name: 'Sensor Array', ip: '192.168.1.102',
      ports: [5000, 5001, 5002], protocol: 'UDP',
      status: 'online', scanProgress: 100, scanPhase: 'Completed',
      vulns: 0, risk: 18, lastScan: '12m ago',
      services: ['Camera Feed', 'LiDAR', 'GPS'],
      tags: ['sensor', 'low'],
      zone: 'Z4',
      credentials: { type: 'Token', status: 'valid' },
      businessPurpose: '传感器数据采集',
      owner: '传感器设备组',
      compliance: ['DO-178C']
    },
  ]);

  const [scanJobs, setScanJobs] = useState<ScanJob[]>([
    {
      id: 1,
      name: 'Full Fleet Scan',
      assets: [1, 2, 3, 4],
      status: 'running',
      progress: 56,
      startTime: '2024-11-05 14:30:00',
      currentAsset: 1,
      results: {
        vulnsFound: 12,
        portsScanned: 156,
        servicesDetected: 28,
      },
      logs: [
        { time: '14:30:00', level: 'info', message: 'Scan started on 4 assets' },
        { time: '14:30:15', level: 'success', message: 'Port scan completed on 192.168.1.100' },
        { time: '14:31:22', level: 'warning', message: 'AI detected buffer overflow vulnerability' },
        { time: '14:32:10', level: 'success', message: 'PoC generated for CVE-2024-1234' },
        { time: '14:32:45', level: 'info', message: 'Testing MAVLink protocol on 192.168.1.101' },
        { time: '14:33:12', level: 'error', message: 'Connection timeout on port 22 (10.0.1.50)' },
        { time: '14:33:45', level: 'success', message: 'Default credentials found on GCS Primary' },
      ],
    },
  ]);

  const vulnerabilities: Vulnerability[] = [
    {
      id: 'CVE-2024-1234', severity: 'CRITICAL',
      title: 'Buffer Overflow in Telemetry Parser',
      description: 'A buffer overflow vulnerability exists in the telemetry data parser allowing remote code execution through malformed telemetry packets.',
      asset: 'DJI Mavic 3 Pro', assetId: 1,
      cvss: 9.8, aiScore: 98, exploitability: 95, impact: 98, verified: false,
      pocAvailable: true,
      pocRequest: {
        method: 'POST', path: '/api/v1/telemetry',
        headers: { 'Content-Type': 'application/json' },
        body: '{"data":"' + 'A'.repeat(500) + '"}',
        protocol: 'HTTP'
      },
      mitreIds: ['T0806', 'T0868', 'T0885'],
      recommendation: 'Update firmware to v2.4.3 immediately. Implement input validation and bounds checking.',
      detectedAt: '2024-11-05 14:32:45', cwe: 'CWE-120',
      affectedComponent: 'telemetry_parser.c:247'
    },
    {
      id: 'SEC-UAV-002', severity: 'CRITICAL',
      title: 'Default Admin Credentials',
      description: 'Ground Control Station ships with default administrative credentials (admin/admin) that can be exploited for full system access.',
      asset: 'GCS Primary Station', assetId: 2,
      cvss: 9.1, aiScore: 100, exploitability: 100, impact: 95, verified: true,
      pocAvailable: true,
      pocRequest: {
        method: 'POST', path: '/api/auth/login',
        headers: { 'Content-Type': 'application/json' },
        body: '{"username":"admin","password":"admin"}',
        protocol: 'HTTPS'
      },
      mitreIds: ['T0817', 'T0859'],
      recommendation: 'Force password change on first login. Implement multi-factor authentication.',
      detectedAt: '2024-11-05 14:28:12', cwe: 'CWE-798',
      affectedComponent: 'auth_module'
    },
    {
      id: 'SEC-UAV-003', severity: 'HIGH',
      title: 'MAVLink Command Injection',
      description: 'The MAVLink command handler does not properly validate parameters, allowing injection of arbitrary flight commands.',
      asset: 'Flight Controller', assetId: 3,
      cvss: 8.2, aiScore: 91, exploitability: 78, impact: 92, verified: false,
      pocAvailable: true,
      pocRequest: {
        method: 'MAVLINK', path: 'MAV_CMD_NAV_WAYPOINT',
        headers: { 'MAVLink-Version': '2.0' },
        body: 'FD 09 00 00 01 01 16 00 00 00 00 00',
        protocol: 'MAVLink'
      },
      mitreIds: ['T0830', 'T0849'],
      recommendation: 'Implement command signature validation. Enable MAVLink 2.0 signing.',
      detectedAt: '2024-11-05 14:30:33', cwe: 'CWE-77',
      affectedComponent: 'mavlink_handler'
    },
    {
      id: 'SEC-UAV-004', severity: 'HIGH',
      title: 'SQL Injection in Flight Logs',
      description: 'Flight log query interface is vulnerable to SQL injection, allowing database access.',
      asset: 'Telemetry Server', assetId: 4,
      cvss: 7.5, aiScore: 89, exploitability: 82, impact: 85, verified: false,
      pocAvailable: true,
      pocRequest: {
        method: 'GET', path: '/logs?id=1\' OR \'1\'=\'1',
        headers: {}, body: '', protocol: 'HTTP'
      },
      mitreIds: ['T0868', 'T0802'],
      recommendation: 'Use parameterized queries. Sanitize all inputs. Implement WAF.',
      detectedAt: '2024-11-05 14:31:21', cwe: 'CWE-89',
      affectedComponent: 'log_query_service'
    },
  ];

  const [traffic, setTraffic] = useState<TrafficPacket[]>([
    {
      id: 1, time: '14:32:50.234', src: '192.168.1.100:8080', dst: '10.0.1.50:443',
      protocol: 'HTTP', method: 'POST', path: '/api/telemetry', status: 200,
      size: 1247, duration: '45ms', anomaly: false, intercepted: false,
      request: 'POST /api/telemetry HTTP/1.1\nHost: 10.0.1.50\nContent-Type: application/json\n\n{"lat":34.05,"lon":-118.25,"alt":120.5}',
      response: 'HTTP/1.1 200 OK\nContent-Type: application/json\n\n{"status":"ok","timestamp":"2024-11-05T14:32:50Z"}',
      assetId: 1
    },
    {
      id: 2, time: '14:32:51.123', src: '10.0.1.50:443', dst: '192.168.1.100:8080',
      protocol: 'HTTP', method: 'POST', path: '/cmd/emergency_stop', status: 200,
      size: 84, duration: '12ms', anomaly: true, intercepted: false, vulnId: 'CVE-2024-1234',
      request: 'POST /cmd/emergency_stop HTTP/1.1\nHost: 192.168.1.100:8080\n\n{"command":"stop"}',
      response: 'HTTP/1.1 200 OK\n\n{"result":"executed"}',
      assetId: 1
    },
    {
      id: 3, time: '14:32:52.567', src: '192.168.1.101:14550', dst: '10.0.1.50:14550',
      protocol: 'MAVLink', method: 'HEARTBEAT', path: 'MAV_TYPE_QUADROTOR', status: 0,
      size: 9, duration: '2ms', anomaly: false, intercepted: false,
      request: 'MAVLink 2.0 HEARTBEAT\nSystem ID: 1\nComponent ID: 1\nType: Quadrotor\nAutopilot: ArduPilot\nBase Mode: ARMED\nCustom Mode: 0\nSystem Status: Active',
      response: 'MAVLink 2.0 ACK\nCommand: HEARTBEAT\nResult: Accepted',
      assetId: 3
    },
    {
      id: 4, time: '14:32:53.234', src: '192.168.1.101:14550', dst: '10.0.1.50:14550',
      protocol: 'MAVLink', method: 'COMMAND_LONG', path: 'MAV_CMD_NAV_WAYPOINT', status: 0,
      size: 33, duration: '8ms', anomaly: false, intercepted: false,
      request: 'MAVLink 2.0 COMMAND_LONG\\nSystem ID: 1\\nComponent ID: 1\\nCommand: MAV_CMD_NAV_WAYPOINT (16)\\nParam1: Hold time (s): 5.0\\nParam2: Accept radius (m): 2.0\\nParam3: Pass through (0=no): 0\\nParam4: Yaw angle (deg): 0\\nParam5: Latitude: 34.052235\\nParam6: Longitude: -118.243683\\nParam7: Altitude (m): 100.0',
      response: 'MAVLink 2.0 COMMAND_ACK\\nCommand: MAV_CMD_NAV_WAYPOINT\\nResult: ACCEPTED (0)',
      assetId: 3
    },
    {
      id: 5, time: '14:32:54.123', src: '192.168.1.101:14550', dst: '10.0.1.50:14550',
      protocol: 'MAVLink', method: 'MISSION_ITEM', path: 'MAV_CMD_NAV_WAYPOINT', status: 0,
      size: 37, duration: '6ms', anomaly: false, intercepted: false,
      request: 'MAVLink 2.0 MISSION_ITEM\\nSystem ID: 1\\nComponent ID: 1\\nSequence: 0\\nFrame: MAV_FRAME_GLOBAL_RELATIVE_ALT\\nCommand: MAV_CMD_NAV_WAYPOINT\\nCurrent: 1\\nAutocontinue: 1\\nx (Lat): 34.052235\\ny (Lon): -118.243683\\nz (Alt): 100.0',
      response: 'MAVLink 2.0 MISSION_ACK\\nType: MAV_MISSION_ACCEPTED',
      assetId: 3
    },
    {
      id: 6, time: '14:32:55.567', src: '192.168.1.100:8080', dst: '10.0.1.50:443',
      protocol: 'HTTPS', method: 'GET', path: '/api/mission/status', status: 200,
      size: 543, duration: '23ms', anomaly: false, intercepted: false,
      request: 'GET /api/mission/status HTTP/1.1\\nHost: 10.0.1.50\\nAuthorization: Bearer eyJhbGc...\\nAccept: application/json',
      response: 'HTTP/1.1 200 OK\\nContent-Type: application/json\\n\\n{\"mission_id\":\"m-001\",\"status\":\"active\",\"current_waypoint\":1,\"total_waypoints\":5}',
      assetId: 1
    },
    {
      id: 7, time: '14:32:56.234', src: '192.168.1.101:14550', dst: '10.0.1.50:14550',
      protocol: 'MAVLink', method: 'GPS_RAW_INT', path: 'MAV_GPS_FIX_TYPE_3D_FIX', status: 0,
      size: 30, duration: '3ms', anomaly: false, intercepted: false,
      request: 'MAVLink 2.0 GPS_RAW_INT\\nSystem ID: 1\\nComponent ID: 1\\nTime (us): 1699194776000000\\nFix Type: 3D Fix\\nLat: 340522350\\nLon: -1182436830\\nAlt: 100000\\nEph: 100\\nEpv: 150\\nVel: 500\\nCog: 18000\\nSatellites visible: 12',
      response: '',
      assetId: 3
    },
    {
      id: 8, time: '14:32:57.123', src: '192.168.1.100:8080', dst: '10.0.1.50:443',
      protocol: 'HTTP', method: 'POST', path: '/api/camera/capture', status: 200,
      size: 234, duration: '156ms', anomaly: false, intercepted: false,
      request: 'POST /api/camera/capture HTTP/1.1\\nHost: 10.0.1.50\\nContent-Type: application/json\\n\\n{\"camera_id\":1,\"resolution\":\"4K\",\"format\":\"JPEG\"}',
      response: 'HTTP/1.1 200 OK\\nContent-Type: application/json\\n\\n{\"image_id\":\"img-12345\",\"url\":\"/images/img-12345.jpg\",\"size\":2457600}',
      assetId: 1
    },
    {
      id: 9, time: '14:32:58.567', src: '192.168.1.101:14550', dst: '10.0.1.50:14550',
      protocol: 'MAVLink', method: 'ATTITUDE', path: 'MAV_FRAME_BODY_NED', status: 0,
      size: 28, duration: '2ms', anomaly: false, intercepted: false,
      request: 'MAVLink 2.0 ATTITUDE\\nSystem ID: 1\\nComponent ID: 1\\nTime (ms): 1699194778567\\nRoll: 0.05\\nPitch: -0.03\\nYaw: 1.57\\nRollspeed: 0.01\\nPitchspeed: 0.02\\nYawspeed: 0.00',
      response: '',
      assetId: 3
    },
    {
      id: 10, time: '14:32:59.234', src: '192.168.1.101:14550', dst: '10.0.1.50:14550',
      protocol: 'MAVLink', method: 'COMMAND_LONG', path: 'MAV_CMD_NAV_WAYPOINT', status: 0,
      size: 33, duration: '9ms', anomaly: true, intercepted: false,
      request: 'MAVLink 2.0 COMMAND_LONG\\nSystem ID: 255\\nComponent ID: 255\\nCommand: MAV_CMD_NAV_WAYPOINT (16)\\nParam1: Hold time (s): 99999.0\\nParam2: Accept radius (m): 0.0\\nParam3: Pass through (0=no): 0\\nParam4: Yaw angle (deg): 0\\nParam5: Latitude: 0.0\\nParam6: Longitude: 0.0\\nParam7: Altitude (m): -1000.0',
      response: 'MAVLink 2.0 COMMAND_ACK\\nCommand: MAV_CMD_NAV_WAYPOINT\\nResult: DENIED (4)\\nReason: Invalid parameters',
      assetId: 3
    },
    {
      id: 11, time: '14:33:00.123', src: '192.168.1.100:8080', dst: '10.0.1.50:443',
      protocol: 'HTTPS', method: 'GET', path: '/api/battery/status', status: 200,
      size: 187, duration: '18ms', anomaly: false, intercepted: false,
      request: 'GET /api/battery/status HTTP/1.1\\nHost: 10.0.1.50\\nAuthorization: Bearer eyJhbGc...',
      response: 'HTTP/1.1 200 OK\\nContent-Type: application/json\\n\\n{\"voltage\":16.8,\"current\":12.5,\"remaining\":85,\"temperature\":28}',
      assetId: 1
    },
    {
      id: 12, time: '14:33:01.567', src: '192.168.1.102:8899', dst: '10.0.1.50:8899',
      protocol: 'DJI', method: 'AUTH_REQUEST', path: 'DJI_AUTH_V2', status: 200,
      size: 128, duration: '45ms', anomaly: false, intercepted: false,
      request: 'DJI Protocol V2\\nMessage Type: AUTH_REQUEST\\nDevice ID: A1B2C3D4E5F6\\nFirmware: v01.04.0300\\nApp ID: 123456\\nNonce: 0x1a2b3c4d',
      response: 'DJI Protocol V2\\nMessage Type: AUTH_RESPONSE\\nResult: SUCCESS\\nSession Token: 0xdeadbeef\\nEncryption Key: <encrypted>',
      assetId: 4
    },
    {
      id: 13, time: '14:33:02.234', src: '192.168.1.102:554', dst: '10.0.1.50:554',
      protocol: 'RTSP', method: 'SETUP', path: 'rtsp://10.0.1.50/stream1', status: 200,
      size: 256, duration: '67ms', anomaly: false, intercepted: false,
      request: 'SETUP rtsp://10.0.1.50/stream1/trackID=0 RTSP/1.0\\nCSeq: 2\\nTransport: RTP/AVP;unicast;client_port=8000-8001',
      response: 'RTSP/1.0 200 OK\\nCSeq: 2\\nTransport: RTP/AVP;unicast;client_port=8000-8001;server_port=9000-9001\\nSession: 12345678',
      assetId: 4
    },
    {
      id: 14, time: '14:33:03.123', src: '192.168.1.101:14550', dst: '10.0.1.50:14550',
      protocol: 'MAVLink', method: 'VFR_HUD', path: 'MAV_FRAME_GLOBAL', status: 0,
      size: 20, duration: '2ms', anomaly: false, intercepted: false,
      request: 'MAVLink 2.0 VFR_HUD\\nSystem ID: 1\\nComponent ID: 1\\nAirspeed: 5.2 m/s\\nGroundspeed: 5.1 m/s\\nHeading: 90 deg\\nThrottle: 65%\\nAlt: 100.0 m\\nClimb: 0.5 m/s',
      response: '',
      assetId: 3
    },
    {
      id: 15, time: '14:33:04.567', src: '192.168.1.100:8080', dst: '10.0.1.50:443',
      protocol: 'HTTP', method: 'POST', path: '/api/logs/upload', status: 500,
      size: 4567, duration: '234ms', anomaly: true, intercepted: false,
      request: 'POST /api/logs/upload HTTP/1.1\\nHost: 10.0.1.50\\nContent-Type: application/octet-stream\\nContent-Length: 4567\\n\\n<binary log data>',
      response: 'HTTP/1.1 500 Internal Server Error\\nContent-Type: application/json\\n\\n{\"error\":\"Buffer overflow in log parser\",\"code\":\"ERR_OVERFLOW\"}',
      assetId: 1
    },
  ]);

  const workflows: Workflow[] = [
    {
      id: 1, name: 'SQL Injection Test Suite', 
      description: 'Comprehensive SQLi testing with AI-generated payloads and validation',
      icon: '💉', trafficQL: 'path~="/api" AND method:POST',
      steps: [
        { type: 'request', config: { method: 'POST', path: '/api/query' } },
        { type: 'fuzz', config: { payloads: ['\'', '1=1', 'UNION SELECT'] } },
        { type: 'validate', config: { expectedStatus: 500 } }
      ],
      appliedTo: [1, 2, 3, 4], runs: 23, lastRun: '2h ago', successRate: 87
    },
    {
      id: 2, name: 'Auth Bypass Checker',
      description: 'Test for authentication vulnerabilities and default credentials',
      icon: '🔓', trafficQL: 'status:401 OR status:403',
      steps: [
        { type: 'request', config: { method: 'POST', path: '/api/auth/login' } },
        { type: 'fuzz', config: { payloads: ['admin:admin', 'root:root'] } }
      ],
      appliedTo: [2], runs: 15, lastRun: '1d ago', successRate: 100
    },
  ];

  const stats = {
    assets: { 
      total: assets.length, 
      online: assets.filter(a => a.status === 'online').length,
      scanning: assets.filter(a => a.status === 'scanning').length,
    },
    vulns: { 
      total: vulnerabilities.length,
      critical: vulnerabilities.filter(v => v.severity === 'CRITICAL').length,
      high: vulnerabilities.filter(v => v.severity === 'HIGH').length,
      verified: vulnerabilities.filter(v => v.verified).length,
    },
    traffic: { 
      total: traffic.length, 
      captured: traffic.length, 
      anomalies: traffic.filter(t => t.anomaly).length 
    },
  };

  const overallProgress = assets.length > 0 ? Math.floor(
    assets.reduce((sum, a) => sum + a.scanProgress, 0) / assets.length
  ) : 0;

  // Simulate scan progress
  useEffect(() => {
    if (scanJobs.length === 0 || scanJobs[0].status !== 'running') return;
    
    const interval = setInterval(() => {
      setScanJobs(prev => {
        const updated = [...prev];
        if (updated[0].progress < 100) {
          updated[0].progress += 1;
          
          // Add random logs
          if (Math.random() > 0.7) {
            const logMessages = [
              { level: 'info' as const, message: 'Scanning ports on ' + assets[0].ip },
              { level: 'success' as const, message: 'Service detected: SSH on port 22' },
              { level: 'warning' as const, message: 'Potential vulnerability detected' },
              { level: 'info' as const, message: 'AI analyzing response patterns...' },
            ];
            const randomLog = logMessages[Math.floor(Math.random() * logMessages.length)];
            updated[0].logs.push({
              time: new Date().toLocaleTimeString(),
              ...randomLog
            });
          }
        } else {
          updated[0].status = 'completed';
          updated[0].endTime = new Date().toISOString();
        }
        return updated;
      });

      // Update asset progress
      setAssets(prev => prev.map((a, idx) => {
        if (idx < 2 && a.scanProgress < 100) {
          return { ...a, scanProgress: Math.min(100, a.scanProgress + 1) };
        }
        return a;
      }));
    }, 1000);

    return () => clearInterval(interval);
  }, [scanJobs, assets]);

  // Simulate traffic capture
  useEffect(() => {
    if (!trafficCapturing) return;
    
    const interval = setInterval(() => {
      if (Math.random() > 0.8) {
        const newPacket: TrafficPacket = {
          id: traffic.length + 1,
          time: new Date().toLocaleTimeString() + '.' + Math.floor(Math.random() * 1000),
          src: `192.168.1.${100 + Math.floor(Math.random() * 10)}:${8000 + Math.floor(Math.random() * 100)}`,
          dst: '10.0.1.50:443',
          protocol: ['HTTP', 'MAVLink'][Math.floor(Math.random() * 2)],
          method: ['GET', 'POST', 'HEARTBEAT'][Math.floor(Math.random() * 3)],
          path: ['/api/status', '/api/telemetry', '/cmd/position'][Math.floor(Math.random() * 3)],
          status: [200, 401, 500][Math.floor(Math.random() * 3)],
          size: 100 + Math.floor(Math.random() * 1000),
          duration: Math.floor(Math.random() * 100) + 'ms',
          anomaly: Math.random() > 0.9,
          intercepted: false,
          request: 'GET /api/status HTTP/1.1\nHost: 10.0.1.50',
          response: 'HTTP/1.1 200 OK\n\n{"status":"ok"}',
          assetId: Math.floor(Math.random() * 4) + 1
        };
        setTraffic(prev => [...prev, newPacket].slice(-100)); // Keep last 100
      }
    }, 2000);

    return () => clearInterval(interval);
  }, [trafficCapturing, traffic.length]);

  // Actions
  const addAsset = () => {
    const ports = newAsset.ports.split(',').map(p => parseInt(p.trim()));
    const newAssetObj: Asset = {
      id: Math.max(...assets.map(a => a.id)) + 1,
      name: newAsset.name,
      ip: newAsset.ip,
      ports,
      protocol: newAsset.protocol,
      status: 'offline',
      scanProgress: 0,
      scanPhase: 'Not scanned',
      vulns: 0,
      risk: 0,
      lastScan: 'Never',
      services: [],
      tags: newAsset.tags.split(',').map(t => t.trim()).filter(t => t),
    };
    setAssets([...assets, newAssetObj]);
    setShowNewAssetDialog(false);
    setNewAsset({ name: '', ip: '', ports: '80,443', protocol: 'HTTP', tags: '' });
    toast.success(`Asset ${newAsset.name} added successfully`);
  };

  const openEditAsset = (asset: Asset) => {
    setEditAsset({
      id: asset.id,
      name: asset.name,
      ip: asset.ip,
      ports: asset.ports.join(','),
      protocol: asset.protocol,
      tags: asset.tags.join(',')
    });
    setShowEditAssetDialog(true);
  };

  const updateAsset = () => {
    const ports = editAsset.ports.split(',').map(p => parseInt(p.trim()));
    setAssets(prev => prev.map(a => 
      a.id === editAsset.id ? {
        ...a,
        name: editAsset.name,
        ip: editAsset.ip,
        ports,
        protocol: editAsset.protocol,
        tags: editAsset.tags.split(',').map(t => t.trim()).filter(t => t)
      } : a
    ));
    setShowEditAssetDialog(false);
    toast.success('Asset updated successfully');
  };

  const confirmDeleteAsset = (id: number) => {
    setAssetToDelete(id);
    setShowDeleteConfirm(true);
  };

  const deleteAsset = () => {
    if (assetToDelete) {
      const asset = assets.find(a => a.id === assetToDelete);
      setAssets(assets.filter(a => a.id !== assetToDelete));
      setSelectedAssets(selectedAssets.filter(aid => aid !== assetToDelete));
      setShowDeleteConfirm(false);
      toast.success(`Asset ${asset?.name} deleted`);
      setAssetToDelete(null);
    }
  };

  const openAIAnalysis = (asset: Asset) => {
    setAnalysisAsset(asset);
    setShowAIAnalysis(true);
  };

  // Mock AI analysis steps
  const getAIAnalysisSteps = (asset: Asset) => [
    {
      id: '1',
      timestamp: '14:32:15.234',
      phase: 'Port Enumeration',
      finding: `Discovered ${asset.ports.length} open ports on ${asset.ip}. Unusual port 14550 (MAVLink) detected - potential drone telemetry interface.`,
      confidence: 98,
      type: 'discovery' as const,
      relatedData: {
        pattern: 'Port 14550: MAVLink Protocol v2.0'
      },
      nextSteps: [
        'Enumerate MAVLink messages and commands',
        'Test for authentication bypass',
        'Check for buffer overflow vulnerabilities in packet handlers'
      ]
    },
    {
      id: '2',
      timestamp: '14:32:16.891',
      phase: 'Service Fingerprinting',
      finding: 'DJI proprietary API detected on port 8080. Version: 3.2.1 - Known vulnerable version with buffer overflow in video stream handler.',
      confidence: 95,
      type: 'hypothesis' as const,
      relatedData: {
        request: 'GET /api/version HTTP/1.1\nHost: 192.168.1.100:8080',
        response: 'HTTP/1.1 200 OK\n{\n  "version": "3.2.1",\n  "device": "DJI Mavic 3 Pro"\n}'
      },
      nextSteps: [
        'Test buffer overflow in video stream handler',
        'Attempt command injection in API parameters'
      ]
    },
    {
      id: '3',
      timestamp: '14:32:18.456',
      phase: 'Vulnerability Testing',
      finding: 'Buffer overflow confirmed in MAVLink COMMAND_LONG message handler. Sending oversized param7 field crashes the telemetry service.',
      confidence: 92,
      type: 'validation' as const,
      relatedData: {
        request: 'MAVLink COMMAND_LONG:\n  command: 176\n  param1-6: 0\n  param7: [4096 bytes of 0x41]',
        response: 'Telemetry service crashed\nCore dump available at /var/crash/mavlink_handler'
      },
      nextSteps: [
        'Generate exploit payload for RCE',
        'Test on other DJI drone models',
        'Create PoC demonstration'
      ]
    },
    {
      id: '4',
      timestamp: '14:32:20.123',
      phase: 'PoC Generation',
      finding: 'Successfully generated working exploit that achieves code execution via buffer overflow. Can execute arbitrary commands with root privileges.',
      confidence: 88,
      type: 'conclusion' as const,
      relatedData: {
        pattern: 'Exploit chain: Buffer Overflow → ROP Gadgets → Shellcode Execution'
      }
    }
  ];

  const startScan = () => {
    if (selectedAssets.length === 0) {
      toast.error('Please select at least one asset to scan');
      return;
    }
    
    const newScan: ScanJob = {
      id: scanJobs.length + 1,
      name: `Scan ${selectedAssets.length} asset${selectedAssets.length > 1 ? 's' : ''}`,
      assets: selectedAssets,
      status: 'running',
      progress: 0,
      startTime: new Date().toISOString(),
      currentAsset: selectedAssets[0],
      results: { vulnsFound: 0, portsScanned: 0, servicesDetected: 0 },
      logs: [
        { time: new Date().toLocaleTimeString(), level: 'info', message: `Scan started on ${selectedAssets.length} asset(s)` }
      ]
    };
    
    setScanJobs([newScan, ...scanJobs]);
    setAssets(prev => prev.map(a => 
      selectedAssets.includes(a.id) ? { ...a, status: 'scanning', scanProgress: 0 } : a
    ));
    
    setShowScanConfigDialog(false);
    // Scan initiated - will appear in Dashboard tasks
    toast.success(`Scan started on ${selectedAssets.length} asset(s)`);
  };

  const pauseScan = () => {
    setScanJobs(prev => prev.map((job, idx) => 
      idx === selectedScanJobIdx ? { ...job, status: job.status === 'running' ? 'paused' : 'running' } : job
    ));
    toast.info(scanJobs[selectedScanJobIdx].status === 'running' ? 'Scan paused' : 'Scan resumed');
  };

  const cancelScan = () => {
    setScanJobs(prev => prev.map((job, idx) => 
      idx === selectedScanJobIdx ? { ...job, status: 'failed', endTime: new Date().toISOString() } : job
    ));
    toast.error('Scan cancelled');
  };

  const sendPoCToTraffic = (vuln: Vulnerability) => {
    setPocBody(vuln.pocRequest.body);
    setView('traffic');
    setTrafficQL(`method:${vuln.pocRequest.method} AND path~="${vuln.pocRequest.path}"`);
    
    // Simulate sending request
    const newPacket: TrafficPacket = {
      id: traffic.length + 1,
      time: new Date().toLocaleTimeString() + '.000',
      src: 'localhost:9999',
      dst: assets.find(a => a.id === vuln.assetId)?.ip + ':' + (vuln.pocRequest.protocol === 'HTTPS' ? '443' : '80'),
      protocol: vuln.pocRequest.protocol,
      method: vuln.pocRequest.method,
      path: vuln.pocRequest.path,
      status: 200,
      size: vuln.pocRequest.body.length,
      duration: '156ms',
      anomaly: true,
      intercepted: false,
      request: `${vuln.pocRequest.method} ${vuln.pocRequest.path} HTTP/1.1\n${Object.entries(vuln.pocRequest.headers).map(([k, v]) => `${k}: ${v}`).join('\n')}\n\n${vuln.pocRequest.body}`,
      response: 'HTTP/1.1 500 Internal Server Error\n\n{"error":"Buffer overflow detected"}',
      vulnId: vuln.id,
      assetId: vuln.assetId
    };
    
    setTraffic(prev => [newPacket, ...prev]);
    setSelectedTrafficIdx(0);
    toast.success('PoC sent to traffic');
  };

  const replayTraffic = () => {
    const packet = traffic[selectedTrafficIdx];
    const newPacket = { ...packet, id: traffic.length + 1, time: new Date().toLocaleTimeString() + '.000' };
    setTraffic(prev => [newPacket, ...prev]);
    setSelectedTrafficIdx(0);
    toast.success('Request replayed');
  };

  const forwardIntercepted = () => {
    setInterceptMode('off');
    setEditingRequest(false);
    toast.success('Request forwarded');
  };

  const dropIntercepted = () => {
    setInterceptMode('off');
    setEditingRequest(false);
    toast.warning('Request dropped');
  };

  const saveAsWorkflow = () => {
    setShowSaveWorkflowDialog(true);
  };

  const createWorkflow = () => {
    toast.success(`Workflow "${workflowForm.name}" created`);
    setShowSaveWorkflowDialog(false);
    setWorkflowForm({ name: '', description: '', icon: '🔥' });
  };

  const applyWorkflow = (workflowId: number) => {
    const workflow = workflows.find(w => w.id === workflowId);
    if (!workflow) return;
    
    toast.info(`Applying workflow "${workflow.name}" to ${selectedAssets.length} asset(s)...`);
    
    setTimeout(() => {
      toast.success(`Workflow completed with ${workflow.successRate}% success rate`);
    }, 2000);
  };

  const getGroupedVulns = () => {
    const filtered = vulnerabilities.filter(v => 
      vulnSearchQuery === '' || 
      v.title.toLowerCase().includes(vulnSearchQuery.toLowerCase()) ||
      v.id.toLowerCase().includes(vulnSearchQuery.toLowerCase())
    );

    if (vulnGroupBy === 'severity') {
      return {
        'CRITICAL': filtered.filter(v => v.severity === 'CRITICAL'),
        'HIGH': filtered.filter(v => v.severity === 'HIGH'),
        'MEDIUM': filtered.filter(v => v.severity === 'MEDIUM'),
        'LOW': filtered.filter(v => v.severity === 'LOW'),
      };
    } else if (vulnGroupBy === 'asset') {
      const grouped: Record<string, Vulnerability[]> = {};
      assets.forEach(asset => {
        grouped[asset.name] = filtered.filter(v => v.assetId === asset.id);
      });
      return grouped;
    } else {
      const grouped: Record<string, Vulnerability[]> = {};
      filtered.forEach(v => {
        v.mitreIds.forEach(mitreId => {
          if (!grouped[mitreId]) grouped[mitreId] = [];
          if (!grouped[mitreId].includes(v)) grouped[mitreId].push(v);
        });
      });
      return grouped;
    }
  };

  const getFilteredTraffic = () => {
    if (!trafficQL) return traffic;
    
    // Simple TrafficQL parsing
    return traffic.filter(t => {
      const query = trafficQL.toLowerCase();
      if (query.includes('method:')) {
        const method = query.match(/method:(\w+)/)?.[1];
        if (method && !t.method.toLowerCase().includes(method)) return false;
      }
      if (query.includes('status:')) {
        const status = query.match(/status:(\d+)/)?.[1];
        if (status && t.status.toString() !== status) return false;
      }
      if (query.includes('protocol:')) {
        const protocol = query.match(/protocol:(\w+)/)?.[1];
        if (protocol && !t.protocol.toLowerCase().includes(protocol)) return false;
      }
      if (query.includes('anomaly:true') && !t.anomaly) return false;
      if (query.includes('path~="')) {
        const path = query.match(/path~="([^"]+)"/)?.[1];
        if (path && !t.path.includes(path)) return false;
      }
      return true;
    });
  };

  // AI Insights
  const aiInsights = [
    {
      type: 'risk' as const,
      title: 'Critical Buffer Overflow Detected',
      description: 'Asset DJI Mavic 3 Pro has a critical buffer overflow vulnerability in MAVLink packet handler. Immediate action required.',
      action: 'View Vulnerability',
      priority: 'high' as const
    },
    {
      type: 'recommendation' as const,
      title: 'Optimize Scan Configuration',
      description: 'Based on your asset types, enabling DJI-specific modules could improve detection rate by 34%.',
      action: 'Apply Optimization',
      priority: 'medium' as const
    },
    {
      type: 'alert' as const,
      title: 'Unusual Traffic Pattern',
      description: '127 unauthorized MAVLink commands detected in the last 5 minutes from 192.168.1.150.',
      action: 'Investigate Traffic',
      priority: 'high' as const
    },
    {
      type: 'success' as const,
      title: 'Workflow Completed Successfully',
      description: 'SQL Injection Test Suite completed on 5 assets with 2 new findings.',
      action: 'View Results',
      priority: 'low' as const
    }
  ];

  const handleAIAction = (insight: typeof aiInsights[0]) => {
    if (insight.action === 'View Vulnerability') {
      setView('vulns');
    } else if (insight.action === 'Investigate Traffic') {
      setView('traffic');
      setTrafficQL('anomaly:true');
    } else if (insight.action === 'View Results') {
      setView('workflows');
    }
    toast.success(`Action: ${insight.action}`);
  };

  const handleExport = (type: typeof exportType) => {
    let data;
    switch (type) {
      case 'assets':
        data = assets;
        break;
      case 'vulnerabilities':
        data = vulnerabilities;
        break;
      case 'traffic':
        data = traffic;
        break;
      case 'scan-results':
        data = scanJobs;
        break;
      case 'full-report':
        data = { assets, vulnerabilities, traffic, scanJobs, workflows };
        break;
    }
    setExportData(data);
    setExportType(type);
    setShowExportDialog(true);
  };

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyPress = (e: KeyboardEvent) => {
      // Don't trigger if in input
      if ((e.target as HTMLElement).tagName === 'INPUT' || (e.target as HTMLElement).tagName === 'TEXTAREA') return;
      
      if (e.key === '1') setView('dashboard');
      if (e.key === '2') setView('assets');
      if (e.key === '3') setView('images');
      if (e.key === '4') setView('vulns');
      if (e.key === '5') setView('traffic');
      if (e.key === '6') setView('workflows');
      if (e.key === '7') setView('devices');
      if (e.key === '?') setShowKeyboardHelp(true);
      if (e.key === 'Escape') {
        setShowKeyboardHelp(false);
        setShowExportDialog(false);
      }
      if (e.key === 'n' && view === 'assets') setShowNewAssetDialog(true);
      if (e.key === 's' && view === 'assets' && selectedAssets.length > 0) setShowScanConfigDialog(true);
      if (e.key === 'Tab') {
        e.preventDefault();
        setFocusPanel(f => f === 'left' ? 'center' : f === 'center' ? 'right' : 'left');
      }
      if (e.key === ' ' && view === 'traffic') {
        e.preventDefault();
        setTrafficCapturing(prev => !prev);
      }
    };
    window.addEventListener('keydown', handleKeyPress);
    return () => window.removeEventListener('keydown', handleKeyPress);
  }, [view]);

  // Panel style
  const getPanelStyle = (panel: FocusPanel) => `
    border-2 transition-all
    ${focusPanel === panel 
      ? 'border-orange-500/50 shadow-2xl shadow-orange-500/20' 
      : 'border-slate-700/30'
    }
  `;

  // Render Dashboard - 聚焦三大核心：关键漏洞、流量监控、AI分析
  const renderDashboard = () => (
    <DashboardView
      focusPanel={focusPanel}
      setFocusPanel={setFocusPanel}
      onViewVulnerability={(vulnId) => {
        const vulnIdx = vulnerabilities.findIndex(v => v.id === `VULN-${vulnId.toString().padStart(4, '0')}`);
        if (vulnIdx >= 0) setSelectedVulnIdx(vulnIdx);
        setView('vulns');
      }}
      onViewTraffic={(trafficId) => {
        setSelectedTrafficIdx(trafficId - 1);
        setView('traffic');
      }}
      onViewAIAnalysis={(assetName) => {
        const asset = assets.find(a => a.name === assetName);
        if (asset) openAIAnalysis(asset);
      }}
    />
  );

  // Continue with other render functions...
  // (Due to length, I'll include the critical ones - Scan, Traffic, Vulns, Workflows)

  return (
    <div className="min-h-screen bg-[#FAFAFA] text-slate-800 font-mono flex flex-col">
      <Toaster position="top-right" richColors />
      
      <TopBar
        currentTime={currentTime}
        view={view}
        setView={setView}
        stats={stats}
      />

      <div className={`flex-1 overflow-hidden ${view === 'settings' ? '' : 'p-3'}`}>
        {view === 'dashboard' && (
          <DashboardView
            focusPanel={focusPanel}
            setFocusPanel={setFocusPanel}
            onViewVulnerability={(vulnId) => {
              setView('vulns');
              const vuln = vulnerabilities.find(v => v.id === vulnId.toString());
              if (vuln) {
                setSelectedVulnIdx(vulnerabilities.indexOf(vuln));
              }
            }}
            onViewTraffic={(trafficId) => {
              setView('traffic');
              const trafficItem = traffic.find(t => t.id === trafficId);
              if (trafficItem) {
                setSelectedTrafficIdx(traffic.indexOf(trafficItem));
              }
            }}
            onViewAIAnalysis={(assetName) => {
              const asset = assets.find(a => a.name === assetName);
              if (asset) {
                openAIAnalysis(asset);
              }
            }}
          />
        )}
        {view === 'assets' && (
          <AssetsView
            assets={assets}
            selectedAssets={selectedAssets}
            setSelectedAssets={setSelectedAssets}
            focusPanel={focusPanel}
            setFocusPanel={setFocusPanel}
            assetSearchQuery={assetSearchQuery}
            setAssetSearchQuery={setAssetSearchQuery}
            onAddAsset={() => setShowNewAssetDialog(true)}
            onEditAsset={openEditAsset}
            onDeleteAsset={confirmDeleteAsset}
            onScanAssets={() => setShowScanConfigDialog(true)}
            onViewAIAnalysis={openAIAnalysis}
          />
        )}
        {view === 'images' && (
          <ImagesView
            focusPanel={focusPanel}
            setFocusPanel={setFocusPanel}
          />
        )}
        {view === 'vulns' && (
          <VulnerabilitiesView
            vulnerabilities={vulnerabilities}
            selectedVulnIdx={selectedVulnIdx}
            setSelectedVulnIdx={setSelectedVulnIdx}
            vulnGroupBy={vulnGroupBy}
            setVulnGroupBy={setVulnGroupBy}
            vulnSearchQuery={vulnSearchQuery}
            setVulnSearchQuery={setVulnSearchQuery}
            editingPoc={editingPoc}
            setEditingPoc={setEditingPoc}
            pocBody={pocBody}
            setPocBody={setPocBody}
            onSendPoC={sendPoCToTraffic}
            onFuzzTest={() => setShowFuzzDialog(true)}
            focusPanel={focusPanel}
            setFocusPanel={setFocusPanel}
            getGroupedVulns={getGroupedVulns}
          />
        )}
        {view === 'traffic' && (
          <TrafficView
            traffic={traffic}
            selectedTrafficIdx={selectedTrafficIdx}
            setSelectedTrafficIdx={setSelectedTrafficIdx}
            trafficQL={trafficQL}
            setTrafficQL={setTrafficQL}
            trafficCapturing={trafficCapturing}
            setTrafficCapturing={setTrafficCapturing}
            interceptMode={interceptMode}
            setInterceptMode={setInterceptMode}
            editingRequest={editingRequest}
            setEditingRequest={setEditingRequest}
            editedRequest={editedRequest}
            setEditedRequest={setEditedRequest}
            assets={assets}
            onReplayTraffic={replayTraffic}
            onForwardIntercepted={forwardIntercepted}
            onDropIntercepted={dropIntercepted}
            onFuzzTest={() => setShowFuzzDialog(true)}
            getFilteredTraffic={getFilteredTraffic}
            focusPanel={focusPanel}
            setFocusPanel={setFocusPanel}
          />
        )}
        {view === 'devices' && (
          <DevicesView
            focusPanel={focusPanel}
            setFocusPanel={setFocusPanel}
          />
        )}
        {view === 'workflows' && (
          <WorkflowsView
            workflows={workflows}
            selectedWorkflowIdx={selectedWorkflowIdx}
            setSelectedWorkflowIdx={setSelectedWorkflowIdx}
            assets={assets}
            selectedAssets={selectedAssets}
            setSelectedAssets={setSelectedAssets}
            onApplyWorkflow={applyWorkflow}
            onCreateWorkflow={() => setShowSaveWorkflowDialog(true)}
            focusPanel={focusPanel}
            setFocusPanel={setFocusPanel}
          />
        )}
        {view === 'settings' && (
          <SettingsView />
        )}
      </div>

      <BottomBar 
        view={view} 
        focusPanel={focusPanel} 
        stats={stats}
        onHelpClick={() => setShowKeyboardHelp(true)}
        onExportClick={() => handleExport('full-report')}
        aiOpsPerSecond={234}
      />

      {/* Dialogs */}
      <Dialog open={showNewAssetDialog} onOpenChange={setShowNewAssetDialog}>
        <DialogContent className="bg-slate-900 border-slate-700">
          <DialogHeader>
            <DialogTitle className="text-slate-100">Add New Asset</DialogTitle>
            <DialogDescription className="text-slate-400 text-xs">
              Add a new asset to your UAV security assessment project
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label className="text-slate-400 text-xs">Name</Label>
              <Input
                value={newAsset.name}
                onChange={(e) => setNewAsset({ ...newAsset, name: e.target.value })}
                placeholder="DJI Mavic 3 Pro"
                className="bg-slate-950 border-slate-700 text-slate-300"
              />
            </div>
            <div>
              <Label className="text-slate-400 text-xs">IP Address</Label>
              <Input
                value={newAsset.ip}
                onChange={(e) => setNewAsset({ ...newAsset, ip: e.target.value })}
                placeholder="192.168.1.100"
                className="bg-slate-950 border-slate-700 text-slate-300"
              />
            </div>
            <div>
              <Label className="text-slate-400 text-xs">Ports (comma-separated)</Label>
              <Input
                value={newAsset.ports}
                onChange={(e) => setNewAsset({ ...newAsset, ports: e.target.value })}
                placeholder="80,443,8080"
                className="bg-slate-950 border-slate-700 text-slate-300"
              />
            </div>
            <div>
              <Label className="text-slate-400 text-xs">Tags</Label>
              <Input
                value={newAsset.tags}
                onChange={(e) => setNewAsset({ ...newAsset, tags: e.target.value })}
                placeholder="drone, critical"
                className="bg-slate-950 border-slate-700 text-slate-300"
              />
            </div>
            <div className="flex gap-2">
              <button
                onClick={addAsset}
                className="flex-1 bg-orange-900/30 hover:bg-orange-900/50 border border-orange-700/50 rounded-lg px-4 py-2 text-sm text-orange-400 transition-all"
              >
                Add Asset
              </button>
              <button
                onClick={() => setShowNewAssetDialog(false)}
                className="flex-1 bg-slate-800 hover:bg-slate-700 border border-slate-700 rounded-lg px-4 py-2 text-sm text-slate-400 transition-all"
              >
                Cancel
              </button>
            </div>
          </div>
        </DialogContent>
      </Dialog>

      <Dialog open={showScanConfigDialog} onOpenChange={setShowScanConfigDialog}>
        <DialogContent className="bg-slate-900 border-slate-700 max-w-2xl">
          <DialogHeader>
            <DialogTitle className="text-slate-100">Configure Scan for {selectedAssets.length} asset(s)</DialogTitle>
            <DialogDescription className="text-slate-400 text-xs">
              Configure scan settings and modules for the selected assets
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label className="text-slate-400 text-xs mb-2 block">Scan Mode</Label>
              <div className="grid grid-cols-3 gap-2">
                {(['quick', 'full', 'custom'] as const).map(mode => (
                  <button
                    key={mode}
                    onClick={() => setScanConfig({ ...scanConfig, mode })}
                    className={`px-4 py-2 rounded-lg text-sm transition-all ${
                      scanConfig.mode === mode
                        ? 'bg-orange-900/30 text-orange-400 border border-orange-700/50'
                        : 'bg-slate-800 text-slate-400 border border-slate-700'
                    }`}
                  >
                    {mode.charAt(0).toUpperCase() + mode.slice(1)}
                  </button>
                ))}
              </div>
            </div>

            <div>
              <Label className="text-slate-400 text-xs mb-2 block">Scan Modules</Label>
              <div className="space-y-2">
                {[
                  { id: 'port_scan', label: 'Port Discovery' },
                  { id: 'service_detect', label: 'Service Detection' },
                  { id: 'vuln_analysis', label: 'Vulnerability Analysis' },
                  { id: 'ai_poc_gen', label: 'AI PoC Generation' },
                ].map(module => (
                  <div key={module.id} className="flex items-center gap-2">
                    <Checkbox
                      checked={scanConfig.modules.includes(module.id)}
                      onCheckedChange={(checked) => {
                        setScanConfig({
                          ...scanConfig,
                          modules: checked
                            ? [...scanConfig.modules, module.id]
                            : scanConfig.modules.filter(m => m !== module.id)
                        });
                      }}
                    />
                    <Label className="text-slate-300 text-sm">{module.label}</Label>
                  </div>
                ))}
              </div>
            </div>

            <div className="flex gap-2">
              <button
                onClick={startScan}
                className="flex-1 bg-orange-900/30 hover:bg-orange-900/50 border border-orange-700/50 rounded-lg px-4 py-2 text-sm text-orange-400 transition-all"
              >
                Start Scan
              </button>
              <button
                onClick={() => setShowScanConfigDialog(false)}
                className="flex-1 bg-slate-800 hover:bg-slate-700 border border-slate-700 rounded-lg px-4 py-2 text-sm text-slate-400 transition-all"
              >
                Cancel
              </button>
            </div>
          </div>
        </DialogContent>
      </Dialog>

      {/* Edit Asset Dialog */}
      <Dialog open={showEditAssetDialog} onOpenChange={setShowEditAssetDialog}>
        <DialogContent className="bg-slate-900 border-slate-700">
          <DialogHeader>
            <DialogTitle className="text-slate-100">Edit Asset</DialogTitle>
            <DialogDescription className="text-slate-400 text-xs">
              Update asset information and configuration
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label className="text-slate-400 text-xs">Name</Label>
              <Input
                value={editAsset.name}
                onChange={(e) => setEditAsset({ ...editAsset, name: e.target.value })}
                className="bg-slate-950 border-slate-700 text-slate-300"
              />
            </div>
            <div>
              <Label className="text-slate-400 text-xs">IP Address</Label>
              <Input
                value={editAsset.ip}
                onChange={(e) => setEditAsset({ ...editAsset, ip: e.target.value })}
                className="bg-slate-950 border-slate-700 text-slate-300"
              />
            </div>
            <div>
              <Label className="text-slate-400 text-xs">Ports (comma-separated)</Label>
              <Input
                value={editAsset.ports}
                onChange={(e) => setEditAsset({ ...editAsset, ports: e.target.value })}
                className="bg-slate-950 border-slate-700 text-slate-300"
              />
            </div>
            <div>
              <Label className="text-slate-400 text-xs">Tags</Label>
              <Input
                value={editAsset.tags}
                onChange={(e) => setEditAsset({ ...editAsset, tags: e.target.value })}
                className="bg-slate-950 border-slate-700 text-slate-300"
              />
            </div>
            <div className="flex gap-2">
              <button
                onClick={updateAsset}
                className="flex-1 bg-orange-900/30 hover:bg-orange-900/50 border border-orange-700/50 rounded-lg px-4 py-2 text-sm text-orange-400 transition-all"
              >
                Update Asset
              </button>
              <button
                onClick={() => setShowEditAssetDialog(false)}
                className="flex-1 bg-slate-800 hover:bg-slate-700 border border-slate-700 rounded-lg px-4 py-2 text-sm text-slate-400 transition-all"
              >
                Cancel
              </button>
            </div>
          </div>
        </DialogContent>
      </Dialog>

      {/* Delete Confirmation Dialog */}
      <Dialog open={showDeleteConfirm} onOpenChange={setShowDeleteConfirm}>
        <DialogContent className="bg-slate-900 border-slate-700">
          <DialogHeader>
            <DialogTitle className="text-slate-100">Delete Asset?</DialogTitle>
            <DialogDescription className="text-slate-400 text-xs">
              This action cannot be undone
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            <p className="text-sm text-slate-400">
              Are you sure you want to delete this asset? This action cannot be undone.
            </p>
            <div className="flex gap-2">
              <button
                onClick={deleteAsset}
                className="flex-1 bg-red-900/30 hover:bg-red-900/50 border border-red-700/50 rounded-lg px-4 py-2 text-sm text-red-400 transition-all"
              >
                Delete
              </button>
              <button
                onClick={() => setShowDeleteConfirm(false)}
                className="flex-1 bg-slate-800 hover:bg-slate-700 border border-slate-700 rounded-lg px-4 py-2 text-sm text-slate-400 transition-all"
              >
                Cancel
              </button>
            </div>
          </div>
        </DialogContent>
      </Dialog>

      {/* FUZZ Configuration Dialog */}
      <Dialog open={showFuzzDialog} onOpenChange={setShowFuzzDialog}>
        <DialogContent className="bg-slate-900 border-slate-700">
          <DialogHeader>
            <DialogTitle className="text-slate-100">FUZZ Configuration</DialogTitle>
            <DialogDescription className="text-slate-400 text-xs">
              Configure fuzzing parameters and payload types
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label className="text-slate-400 text-xs mb-2 block">Target</Label>
              <select
                value={fuzzConfig.target}
                onChange={(e) => setFuzzConfig({ ...fuzzConfig, target: e.target.value })}
                className="w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-300"
              >
                <option value="body">Request Body</option>
                <option value="headers">Headers</option>
                <option value="path">URL Path</option>
              </select>
            </div>

            <div>
              <Label className="text-slate-400 text-xs mb-2 block">Payload Type</Label>
              <select
                value={fuzzConfig.payloadType}
                onChange={(e) => setFuzzConfig({ ...fuzzConfig, payloadType: e.target.value })}
                className="w-full bg-slate-950 border border-slate-700 rounded-lg px-3 py-2 text-sm text-slate-300"
              >
                <option value="sqli">SQL Injection</option>
                <option value="xss">XSS Payloads</option>
                <option value="buffer">Buffer Overflow</option>
                <option value="custom">Custom Dictionary</option>
              </select>
            </div>

            {fuzzConfig.payloadType === 'custom' && (
              <div>
                <Label className="text-slate-400 text-xs mb-2 block">Custom Payloads (one per line)</Label>
                <Textarea
                  value={fuzzConfig.customPayloads}
                  onChange={(e) => setFuzzConfig({ ...fuzzConfig, customPayloads: e.target.value })}
                  placeholder="' OR '1'='1\n1' AND 1=1--\nUNION SELECT..."
                  className="bg-slate-950 border-slate-700 text-slate-300 font-mono text-xs h-32"
                />
              </div>
            )}

            <div className="grid grid-cols-2 gap-4">
              <div>
                <Label className="text-slate-400 text-xs">Threads</Label>
                <Input
                  type="number"
                  value={fuzzConfig.threads}
                  onChange={(e) => setFuzzConfig({ ...fuzzConfig, threads: parseInt(e.target.value) })}
                  className="bg-slate-950 border-slate-700 text-slate-300"
                />
              </div>
              <div>
                <Label className="text-slate-400 text-xs">Delay (ms)</Label>
                <Input
                  type="number"
                  value={fuzzConfig.delay}
                  onChange={(e) => setFuzzConfig({ ...fuzzConfig, delay: parseInt(e.target.value) })}
                  className="bg-slate-950 border-slate-700 text-slate-300"
                />
              </div>
            </div>

            <div className="flex gap-2">
              <button
                onClick={() => { setShowFuzzDialog(false); setView('traffic'); toast.success('FUZZ test started'); }}
                className="flex-1 bg-orange-900/30 hover:bg-orange-900/50 border border-orange-700/50 rounded-lg px-4 py-2 text-sm text-orange-400 transition-all"
              >
                Start FUZZ
              </button>
              <button
                onClick={() => setShowFuzzDialog(false)}
                className="flex-1 bg-slate-800 hover:bg-slate-700 border border-slate-700 rounded-lg px-4 py-2 text-sm text-slate-400 transition-all"
              >
                Cancel
              </button>
            </div>
          </div>
        </DialogContent>
      </Dialog>

      {/* Save Workflow Dialog */}
      <Dialog open={showSaveWorkflowDialog} onOpenChange={setShowSaveWorkflowDialog}>
        <DialogContent className="bg-slate-900 border-slate-700">
          <DialogHeader>
            <DialogTitle className="text-slate-100">Create New Workflow</DialogTitle>
            <DialogDescription className="text-slate-400 text-xs">
              Create a new automated testing workflow
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            <div>
              <Label className="text-slate-400 text-xs">Workflow Name</Label>
              <Input
                value={workflowForm.name}
                onChange={(e) => setWorkflowForm({ ...workflowForm, name: e.target.value })}
                placeholder="SQL Injection Test Suite"
                className="bg-slate-950 border-slate-700 text-slate-300"
              />
            </div>
            <div>
              <Label className="text-slate-400 text-xs">Description</Label>
              <Textarea
                value={workflowForm.description}
                onChange={(e) => setWorkflowForm({ ...workflowForm, description: e.target.value })}
                placeholder="Comprehensive SQL injection testing with AI analysis"
                className="bg-slate-950 border-slate-700 text-slate-300 h-20"
              />
            </div>
            <div>
              <Label className="text-slate-400 text-xs mb-2 block">Icon</Label>
              <div className="flex gap-2">
                {['💉', '🔓', '🔥', '⚡', '🎯', '🛡️'].map(icon => (
                  <button
                    key={icon}
                    onClick={() => setWorkflowForm({ ...workflowForm, icon })}
                    className={`text-2xl p-2 rounded-lg transition-all ${
                      workflowForm.icon === icon
                        ? 'bg-orange-900/30 border-2 border-orange-500/50'
                        : 'bg-slate-800 border border-slate-700 hover:border-slate-600'
                    }`}
                  >
                    {icon}
                  </button>
                ))}
              </div>
            </div>
            <div className="flex gap-2">
              <button
                onClick={createWorkflow}
                className="flex-1 bg-orange-900/30 hover:bg-orange-900/50 border border-orange-700/50 rounded-lg px-4 py-2 text-sm text-orange-400 transition-all"
              >
                Create Workflow
              </button>
              <button
                onClick={() => setShowSaveWorkflowDialog(false)}
                className="flex-1 bg-slate-800 hover:bg-slate-700 border border-slate-700 rounded-lg px-4 py-2 text-sm text-slate-400 transition-all"
              >
                Cancel
              </button>
            </div>
          </div>
        </DialogContent>
      </Dialog>

      {/* Keyboard Shortcuts Help */}
      <KeyboardShortcutsHelp
        show={showKeyboardHelp}
        onClose={() => setShowKeyboardHelp(false)}
      />

      {/* Export Dialog */}
      <ExportDialog
        show={showExportDialog}
        onClose={() => setShowExportDialog(false)}
        data={exportData}
        dataType={exportType}
      />

      {/* AI Analysis Viewer */}
      {showAIAnalysis && analysisAsset && (
        <AIAnalysisViewer
          assetName={analysisAsset.name}
          analysisSteps={getAIAnalysisSteps(analysisAsset)}
          onClose={() => setShowAIAnalysis(false)}
        />
      )}

      {/* AI Configuration Panel */}
      <AIConfigPanel
        show={showAIConfig}
        onClose={() => setShowAIConfig(false)}
        config={aiConfig}
        onSave={(newConfig) => {
          setAIConfig(newConfig);
          toast.success('AI configuration saved');
        }}
      />
    </div>
  );
}
