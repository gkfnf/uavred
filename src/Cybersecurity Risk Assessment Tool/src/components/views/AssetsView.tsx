import { Plus, Zap, Edit3, Trash2, Settings, Brain, Network, Shield, Activity, ChevronDown, ChevronUp, AlertTriangle } from 'lucide-react';
import { useState, useEffect, useRef } from 'react';

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

interface AssetsViewProps {
  assets: Asset[];
  selectedAssets: number[];
  setSelectedAssets: (ids: number[]) => void;
  focusPanel: 'left' | 'center' | 'right';
  setFocusPanel: (panel: 'left' | 'center' | 'right') => void;
  assetSearchQuery: string;
  setAssetSearchQuery: (query: string) => void;
  onAddAsset: () => void;
  onEditAsset: (asset: Asset) => void;
  onDeleteAsset: (id: number) => void;
  onScanAssets: () => void;
  onViewAIAnalysis?: (asset: Asset) => void;
}

const ZONE_INFO = {
  Z1: { 
    name: '地面指挥中心', 
    color: 'bg-blue-50',
    borderColor: 'border-blue-300',
    textColor: 'text-blue-700',
    description: 'GCS' 
  },
  Z2: { 
    name: '通信网关层', 
    color: 'bg-green-50',
    borderColor: 'border-green-300',
    textColor: 'text-green-700',
    description: 'Gateway' 
  },
  Z3: { 
    name: '任务控制层', 
    color: 'bg-purple-50',
    borderColor: 'border-purple-300',
    textColor: 'text-purple-700',
    description: 'Control' 
  },
  Z4: { 
    name: '飞控设备层', 
    color: 'bg-orange-50',
    borderColor: 'border-orange-300',
    textColor: 'text-orange-700',
    description: 'Devices' 
  },
  Z5: { 
    name: '安全紧急系统', 
    color: 'bg-red-50',
    borderColor: 'border-red-300',
    textColor: 'text-red-700',
    description: 'Safety' 
  },
};

// Define network connections between assets
const ASSET_CONNECTIONS = [
  // GCS (Z1) connects to everything - it's the central hub
  { from: 2, to: 4 },   // GCS -> Telemetry Server
  { from: 2, to: 5 },   // GCS -> Mission Control
  { from: 2, to: 7 },   // GCS -> Data Gateway
  { from: 2, to: 1 },   // GCS -> DJI Mavic (direct control)
  { from: 2, to: 3 },   // GCS -> Flight Controller
  { from: 2, to: 6 },   // GCS -> Emergency System
  
  // Z2 Communication layer connections
  { from: 4, to: 5 },   // Telemetry -> Mission Control
  { from: 4, to: 7 },   // Telemetry -> Data Gateway
  { from: 7, to: 5 },   // Data Gateway -> Mission Control
  
  // Mission Control (Z3) to Devices (Z4)
  { from: 5, to: 1 },   // Mission Control -> DJI Mavic
  { from: 5, to: 3 },   // Mission Control -> Flight Controller
  { from: 5, to: 8 },   // Mission Control -> Sensor Array
  
  // Devices (Z4) internal connections
  { from: 1, to: 8 },   // DJI Mavic -> Sensor Array
  { from: 3, to: 8 },   // Flight Controller -> Sensor Array
  { from: 1, to: 3 },   // DJI Mavic -> Flight Controller
  
  // Emergency connections (Z5) - connected to critical devices
  { from: 6, to: 1 },   // Emergency -> DJI Mavic
  { from: 6, to: 3 },   // Emergency -> Flight Controller
  { from: 6, to: 5 },   // Emergency -> Mission Control
  { from: 6, to: 2 },   // Emergency -> GCS
];

// Get node color based on risk level
const getNodeColor = (risk: number) => {
  if (risk >= 80) return { fill: '#ef4444', stroke: '#dc2626' }; // Red - Critical
  if (risk >= 60) return { fill: '#f97316', stroke: '#ea580c' }; // Orange - High
  if (risk >= 40) return { fill: '#eab308', stroke: '#ca8a04' }; // Yellow - Medium
  return { fill: '#22c55e', stroke: '#16a34a' }; // Green - Low
};

export function AssetsView({
  assets,
  selectedAssets,
  setSelectedAssets,
  focusPanel,
  setFocusPanel,
  assetSearchQuery,
  setAssetSearchQuery,
  onAddAsset,
  onEditAsset,
  onDeleteAsset,
  onScanAssets,
  onViewAIAnalysis,
}: AssetsViewProps) {
  const [assetPositions, setAssetPositions] = useState<Record<number, { x: number; y: number }>>({});
  const [detailsExpanded, setDetailsExpanded] = useState(true);
  const [topologyExpanded, setTopologyExpanded] = useState(true);
  const [hoveredAsset, setHoveredAsset] = useState<number | null>(null);
  const assetRefs = useRef<Record<number, HTMLDivElement | null>>({});
  const containerRef = useRef<HTMLDivElement | null>(null);

  const selectedAsset = selectedAssets.length === 1 ? assets.find(a => a.id === selectedAssets[0]) : null;

  // Group assets by zone
  const assetsByZone = {
    Z1: assets.filter(a => a.zone === 'Z1'),
    Z2: assets.filter(a => a.zone === 'Z2'),
    Z3: assets.filter(a => a.zone === 'Z3'),
    Z4: assets.filter(a => a.zone === 'Z4'),
    Z5: assets.filter(a => a.zone === 'Z5'),
  };

  // Get connected assets (for attack path visualization)
  const getConnectedAssets = (assetId: number) => {
    const connected = new Set<number>();
    ASSET_CONNECTIONS.forEach(conn => {
      if (conn.from === assetId) connected.add(conn.to);
      if (conn.to === assetId) connected.add(conn.from);
    });
    return connected;
  };

  // Check if connection is part of attack path
  const isAttackPath = (fromId: number, toId: number) => {
    if (!selectedAsset || selectedAsset.risk < 60) return false;
    const connectedAssets = getConnectedAssets(selectedAsset.id);
    return (
      (fromId === selectedAsset.id && connectedAssets.has(toId)) ||
      (toId === selectedAsset.id && connectedAssets.has(fromId))
    );
  };

  // Calculate positions after render
  useEffect(() => {
    const updatePositions = () => {
      if (!containerRef.current) return;
      
      const containerRect = containerRef.current.getBoundingClientRect();
      const newPositions: Record<number, { x: number; y: number }> = {};

      Object.entries(assetRefs.current).forEach(([id, el]) => {
        if (el) {
          const rect = el.getBoundingClientRect();
          newPositions[Number(id)] = {
            x: rect.left - containerRect.left + rect.width / 2,
            y: rect.top - containerRect.top + rect.height / 2,
          };
        }
      });

      setAssetPositions(newPositions);
    };

    // Multiple updates to ensure proper calculation
    const timer1 = setTimeout(updatePositions, 100);
    const timer2 = setTimeout(updatePositions, 300);
    const timer3 = setTimeout(updatePositions, 500);
    
    window.addEventListener('resize', updatePositions);
    window.addEventListener('scroll', updatePositions, true);
    
    return () => {
      clearTimeout(timer1);
      clearTimeout(timer2);
      clearTimeout(timer3);
      window.removeEventListener('resize', updatePositions);
      window.removeEventListener('scroll', updatePositions, true);
    };
  }, [assets, detailsExpanded]);

  return (
    <div className="flex flex-col gap-3 h-full bg-[#FAFAFA] p-3">
      {/* Main: Network Topology Graph */}
      <div className={`bg-white rounded-xl overflow-hidden flex flex-col shadow-sm border-2 border-slate-200 transition-all ${
        !topologyExpanded ? 'h-12' : 
        selectedAsset && detailsExpanded ? 'flex-[0.65]' : 
        'flex-1'
      }`}>
        <div 
          className="border-b border-slate-200 px-4 py-2.5 bg-slate-50 flex items-center justify-between cursor-pointer hover:bg-slate-100 transition-colors"
          onClick={() => setTopologyExpanded(!topologyExpanded)}
        >
          <div className="flex items-center gap-3">
            <Network className="w-3.5 h-3.5 text-purple-600" />
            <span className="text-xs text-slate-700 font-medium">网络拓扑 - 业务层级视图</span>
            {topologyExpanded && selectedAsset && selectedAsset.risk >= 60 && (
              <div className="flex items-center gap-1.5 px-2 py-1 bg-red-50 border border-red-200 rounded text-[10px] text-red-700 font-medium">
                <AlertTriangle className="w-3 h-3" />
                <span>攻击路径分析模式</span>
              </div>
            )}
          </div>
          <div className="flex items-center gap-3">
            {topologyExpanded && (
              <>
                <div className="flex items-center gap-2 text-[9px] text-slate-500">
                  <div className="flex items-center gap-1">
                    <div className="w-2 h-2 rounded-full bg-emerald-500" />
                    <span>低危</span>
                  </div>
                  <div className="flex items-center gap-1">
                    <div className="w-2 h-2 rounded-full bg-yellow-500" />
                    <span>中危</span>
                  </div>
                  <div className="flex items-center gap-1">
                    <div className="w-2 h-2 rounded-full bg-orange-500" />
                    <span>高危</span>
                  </div>
                  <div className="flex items-center gap-1">
                    <div className="w-2 h-2 rounded-full bg-red-500" />
                    <span>严重</span>
                  </div>
                </div>
                <div className="text-[10px] text-slate-500">
                  {assets.length} 资产 · {ASSET_CONNECTIONS.length} 连接
                </div>
              </>
            )}
            {topologyExpanded ? (
              <ChevronDown className="w-4 h-4 text-slate-500" />
            ) : (
              <ChevronUp className="w-4 h-4 text-slate-500" />
            )}
          </div>
        </div>
        
        {topologyExpanded && (
          <div className="flex-1 overflow-auto relative bg-slate-50/30">
            {/* Zone columns */}
            <div className="flex h-full relative min-h-[500px]" ref={containerRef}>
              {/* SVG for connections - now inside the same container as nodes */}
              <svg className="absolute inset-0 w-full h-full pointer-events-none" style={{ zIndex: 1 }}>
                <defs>
                  <marker
                    id="arrowhead-normal"
                    markerWidth="6"
                    markerHeight="6"
                    refX="5"
                    refY="3"
                    orient="auto"
                  >
                    <polygon points="0 0, 6 3, 0 6" fill="#94a3b8" />
                  </marker>
                  <marker
                    id="arrowhead-attack"
                    markerWidth="8"
                    markerHeight="8"
                    refX="6"
                    refY="4"
                    orient="auto"
                  >
                    <polygon points="0 0, 8 4, 0 8" fill="#ef4444" />
                  </marker>
                  <marker
                    id="arrowhead-selected"
                    markerWidth="6"
                    markerHeight="6"
                    refX="5"
                    refY="3"
                    orient="auto"
                  >
                    <polygon points="0 0, 6 3, 0 6" fill="#a855f7" />
                  </marker>
                </defs>
                {ASSET_CONNECTIONS.map((conn, idx) => {
                  const fromPos = assetPositions[conn.from];
                  const toPos = assetPositions[conn.to];
                  if (!fromPos || !toPos) return null;

                  const isSelected = selectedAssets.includes(conn.from) || selectedAssets.includes(conn.to);
                  const isAttack = isAttackPath(conn.from, conn.to);
                  const isHovered = hoveredAsset === conn.from || hoveredAsset === conn.to;

                  return (
                    <g key={idx}>
                      <line
                        x1={fromPos.x}
                        y1={fromPos.y}
                        x2={toPos.x}
                        y2={toPos.y}
                        stroke={isAttack ? '#ef4444' : isSelected ? '#a855f7' : '#94a3b8'}
                        strokeWidth={isAttack ? '3' : isSelected ? '2.5' : '2'}
                        strokeDasharray={isAttack ? '0' : '5,3'}
                        markerEnd={`url(#arrowhead-${isAttack ? 'attack' : isSelected ? 'selected' : 'normal'})`}
                        opacity={isAttack ? 1 : isSelected ? 0.9 : 0.6}
                        className="transition-all duration-300"
                      />
                    </g>
                  );
                })}
              </svg>

              {(['Z1', 'Z2', 'Z3', 'Z4', 'Z5'] as const).map((zone, zoneIdx) => {
                return (
                  <div
                    key={zone}
                    className={`flex-1 border-r border-slate-200 last:border-r-0 ${ZONE_INFO[zone].color} flex flex-col`}
                  >
                    {/* Zone Header */}
                    <div className={`px-3 py-2 border-b ${ZONE_INFO[zone].borderColor} bg-white/60 backdrop-blur-sm sticky top-0 z-10`}>
                      <div className="flex items-center justify-between mb-1">
                        <div className="flex items-center gap-1.5">
                          <Shield className={`w-3 h-3 ${ZONE_INFO[zone].textColor}`} />
                          <span className={`text-xs font-bold ${ZONE_INFO[zone].textColor}`}>{zone}</span>
                        </div>
                        <button
                          onClick={onAddAsset}
                          className={`p-0.5 rounded hover:bg-white/80 transition-colors ${ZONE_INFO[zone].textColor}`}
                          title="添加资产"
                        >
                          <Plus className="w-3.5 h-3.5" />
                        </button>
                      </div>
                      <div className="text-[9px] text-slate-600 font-medium truncate" title={ZONE_INFO[zone].name}>
                        {ZONE_INFO[zone].name}
                      </div>
                      <div className="text-[8px] text-slate-500 mt-0.5">
                        {assetsByZone[zone].length} 资产
                      </div>
                    </div>

                    {/* Assets as nodes */}
                    <div className="flex-1 p-4 flex flex-col justify-around items-center overflow-y-auto">
                      {assetsByZone[zone].map((asset) => {
                        const isSelected = selectedAssets.includes(asset.id);
                        const isHovered = hoveredAsset === asset.id;
                        const isConnectedToSelected = selectedAsset && getConnectedAssets(selectedAsset.id).has(asset.id);
                        const colors = getNodeColor(asset.risk);
                        
                        return (
                          <div
                            key={asset.id}
                            ref={(el) => assetRefs.current[asset.id] = el}
                            className="relative group cursor-pointer"
                            onClick={() => setSelectedAssets([asset.id])}
                            onMouseEnter={() => setHoveredAsset(asset.id)}
                            onMouseLeave={() => setHoveredAsset(null)}
                          >
                            {/* Node circle */}
                            <div
                              className={`relative transition-all duration-300 ${
                                isSelected ? 'scale-150' : isHovered ? 'scale-125' : 'scale-100'
                              }`}
                              style={{
                                width: '32px',
                                height: '32px',
                              }}
                            >
                              <svg width="32" height="32" viewBox="0 0 32 32">
                                {/* Outer glow for selected/attack path */}
                                {(isSelected || isConnectedToSelected) && (
                                  <circle
                                    cx="16"
                                    cy="16"
                                    r="14"
                                    fill="none"
                                    stroke={isSelected ? '#a855f7' : '#ef4444'}
                                    strokeWidth="2"
                                    opacity="0.3"
                                    className="animate-pulse"
                                  />
                                )}
                                
                                {/* Main node */}
                                <circle
                                  cx="16"
                                  cy="16"
                                  r="10"
                                  fill={colors.fill}
                                  stroke={isSelected ? '#a855f7' : colors.stroke}
                                  strokeWidth={isSelected ? '3' : '2'}
                                  className="transition-all duration-300"
                                />
                                
                                {/* Status indicator */}
                                <circle
                                  cx="22"
                                  cy="10"
                                  r="3"
                                  fill={
                                    asset.status === 'scanning' ? '#a855f7' :
                                    asset.status === 'online' ? '#22c55e' : '#94a3b8'
                                  }
                                  stroke="white"
                                  strokeWidth="1.5"
                                />
                              </svg>
                            </div>

                            {/* Label */}
                            <div className={`absolute left-1/2 -translate-x-1/2 top-full mt-1 whitespace-nowrap transition-all duration-200 ${
                              isSelected || isHovered ? 'opacity-100 scale-100' : 'opacity-0 scale-90'
                            }`}>
                              <div className="bg-slate-900/90 backdrop-blur-sm text-white px-2 py-1 rounded text-[10px] font-medium shadow-lg">
                                <div className="font-bold">{asset.name}</div>
                                <div className="text-[9px] text-slate-300 font-mono">{asset.ip}</div>
                                <div className="text-[9px] text-slate-300 mt-0.5">
                                  风险: {asset.risk} · {asset.vulns} 漏洞
                                </div>
                              </div>
                            </div>

                            {/* Always visible minimal label */}
                            <div className={`absolute left-1/2 -translate-x-1/2 top-full mt-1 text-[9px] font-medium text-slate-700 whitespace-nowrap transition-opacity duration-200 ${
                              isSelected || isHovered ? 'opacity-0' : 'opacity-100'
                            }`}>
                              {asset.name.length > 12 ? asset.name.substring(0, 12) + '...' : asset.name}
                            </div>

                            {/* Attack path indicator */}
                            {selectedAsset && selectedAsset.id !== asset.id && isConnectedToSelected && selectedAsset.risk >= 60 && (
                              <div className="absolute -top-1 -right-1">
                                <div className="w-4 h-4 bg-red-500 rounded-full flex items-center justify-center animate-pulse">
                                  <AlertTriangle className="w-2.5 h-2.5 text-white" />
                                </div>
                              </div>
                            )}
                          </div>
                        );
                      })}
                      
                      {assetsByZone[zone].length === 0 && (
                        <div className="text-center text-[10px] text-slate-400 italic">
                          暂无资产
                        </div>
                      )}
                    </div>
                  </div>
                );
              })}
            </div>
          </div>
        )}
      </div>

      {/* Bottom: Asset Details Panel */}
      {selectedAsset && (
        <div className={`bg-white rounded-xl overflow-hidden shadow-sm border-2 border-slate-200 transition-all ${
          detailsExpanded ? 'flex-[0.35]' : 'h-12'
        }`}>
          <div 
            className="border-b border-slate-200 px-4 py-2.5 bg-slate-50 flex items-center justify-between cursor-pointer hover:bg-slate-100 transition-colors"
            onClick={() => setDetailsExpanded(!detailsExpanded)}
          >
            <div className="flex items-center gap-3">
              <span className="text-xs text-slate-700 font-medium">资产详情</span>
              <div className="flex items-center gap-2">
                <div 
                  className="w-3 h-3 rounded-full" 
                  style={{ backgroundColor: getNodeColor(selectedAsset.risk).fill }}
                />
                <span className="text-xs text-slate-900 font-bold">{selectedAsset.name}</span>
                <span className="text-[10px] text-slate-500 font-mono">{selectedAsset.ip}</span>
              </div>
              {selectedAsset.risk >= 60 && (
                <div className="flex items-center gap-1 px-2 py-0.5 bg-red-50 border border-red-200 rounded text-[10px] text-red-700 font-medium">
                  <AlertTriangle className="w-3 h-3" />
                  <span>高风险节点 - 潜在攻击起点</span>
                </div>
              )}
            </div>
            <div className="flex items-center gap-2">
              <div className="flex gap-1">
                <button
                  onClick={(e) => { e.stopPropagation(); onEditAsset(selectedAsset); }}
                  className="p-1.5 hover:bg-slate-200 rounded transition-all text-slate-500 hover:text-slate-800"
                >
                  <Edit3 className="w-3.5 h-3.5" />
                </button>
                <button
                  onClick={(e) => { e.stopPropagation(); onDeleteAsset(selectedAsset.id); }}
                  className="p-1.5 hover:bg-red-50 rounded transition-all text-slate-400 hover:text-red-600"
                >
                  <Trash2 className="w-3.5 h-3.5" />
                </button>
              </div>
              {detailsExpanded ? (
                <ChevronDown className="w-4 h-4 text-slate-500" />
              ) : (
                <ChevronUp className="w-4 h-4 text-slate-500" />
              )}
            </div>
          </div>
          
          {detailsExpanded && (
            <div className="p-4 overflow-auto h-[calc(100%-42px)]">
              <div className="grid grid-cols-5 gap-4">
                {/* Column 1: Zone & Risk */}
                <div className="space-y-3">
                  {selectedAsset.zone && (
                    <div className={`rounded-lg border-2 p-3 ${ZONE_INFO[selectedAsset.zone].color} ${ZONE_INFO[selectedAsset.zone].borderColor} shadow-sm`}>
                      <div className="flex items-center gap-2 mb-1">
                        <Shield className="w-4 h-4" />
                        <span className={`font-bold ${ZONE_INFO[selectedAsset.zone].textColor}`}>
                          {selectedAsset.zone}
                        </span>
                      </div>
                      <div className="text-[10px] font-medium text-slate-700">{ZONE_INFO[selectedAsset.zone].name}</div>
                    </div>
                  )}

                  <div className="bg-purple-50 rounded-lg border border-purple-100 p-3">
                    <div className="flex items-center justify-between mb-2">
                      <span className="text-[10px] text-purple-900 font-medium">风险评分</span>
                      <span className="text-xl font-bold text-purple-900">{selectedAsset.risk}</span>
                    </div>
                    <div className="relative h-2 bg-white rounded-full overflow-hidden border border-purple-100">
                      <div
                        className={`h-full transition-all ${
                          selectedAsset.risk >= 80 ? 'bg-gradient-to-r from-red-500 to-red-400' :
                          selectedAsset.risk >= 60 ? 'bg-gradient-to-r from-orange-500 to-orange-400' :
                          'bg-gradient-to-r from-yellow-500 to-yellow-400'
                        }`}
                        style={{ width: `${selectedAsset.risk}%` }}
                      />
                    </div>
                  </div>

                  <div className="grid grid-cols-1 gap-2">
                    <div className="bg-slate-50 rounded-lg p-2 border border-slate-200">
                      <div className="text-[9px] text-slate-500 mb-1 font-medium">状态</div>
                      <div className={`text-xs font-medium ${
                        selectedAsset.status === 'online' ? 'text-emerald-600' :
                        selectedAsset.status === 'scanning' ? 'text-purple-600' : 'text-slate-500'
                      }`}>
                        {selectedAsset.status.toUpperCase()}
                      </div>
                    </div>
                  </div>
                </div>

                {/* Column 2: Ports */}
                <div className="space-y-3">
                  <div>
                    <div className="text-[10px] text-slate-500 mb-2 font-medium">开放端口</div>
                    <div className="flex flex-wrap gap-1.5">
                      {selectedAsset.ports.map(port => (
                        <span key={port} className="px-2 py-0.5 rounded bg-slate-100 text-[10px] text-slate-700 border border-slate-200 font-mono">
                          {port}
                        </span>
                      ))}
                    </div>
                  </div>
                  <div className="text-[9px] text-slate-500">
                    <span className="font-medium">协议：</span>
                    <span className="font-mono">{selectedAsset.protocol}</span>
                  </div>
                  <div className="text-[9px] text-slate-500">
                    <span className="font-medium">最后扫描：</span>
                    <span className="font-mono">{selectedAsset.lastScan}</span>
                  </div>
                </div>

                {/* Column 3: Services */}
                <div className="space-y-3">
                  <div>
                    <div className="text-[10px] text-slate-500 mb-2 font-medium">检测到的服务</div>
                    <div className="space-y-1">
                      {selectedAsset.services.map((svc, i) => (
                        <div key={i} className="px-2 py-1 rounded bg-slate-50 text-[10px] text-slate-700 border border-slate-200 font-medium">
                          {svc}
                        </div>
                      ))}
                    </div>
                  </div>
                </div>

                {/* Column 4: Credentials & Business Info */}
                <div className="space-y-3">
                  {/* Credentials */}
                  {selectedAsset.credentials && (
                    <div className="bg-slate-50 rounded-lg border border-slate-200 p-3">
                      <div className="text-[10px] text-slate-500 mb-2 font-medium">认证凭证</div>
                      <div className="space-y-1.5">
                        <div className="text-[10px] text-slate-700">
                          <span className="font-medium">类型：</span>
                          <span className="ml-1">{selectedAsset.credentials.type}</span>
                        </div>
                        {selectedAsset.credentials.username && (
                          <div className="text-[10px] text-slate-700 font-mono bg-white px-2 py-1 rounded border border-slate-200">
                            {selectedAsset.credentials.username}
                          </div>
                        )}
                        <div className="flex items-center gap-1.5 mt-2">
                          <div className={`w-2 h-2 rounded-full ${
                            selectedAsset.credentials.status === 'valid' ? 'bg-emerald-500' :
                            selectedAsset.credentials.status === 'weak' ? 'bg-red-500' :
                            selectedAsset.credentials.status === 'expired' ? 'bg-orange-500' :
                            'bg-slate-400'
                          }`} />
                          <span className={`text-[9px] font-medium ${
                            selectedAsset.credentials.status === 'valid' ? 'text-emerald-700' :
                            selectedAsset.credentials.status === 'weak' ? 'text-red-700' :
                            selectedAsset.credentials.status === 'expired' ? 'text-orange-700' :
                            'text-slate-600'
                          }`}>
                            {selectedAsset.credentials.status === 'valid' ? '有效' :
                             selectedAsset.credentials.status === 'weak' ? '弱凭证' :
                             selectedAsset.credentials.status === 'expired' ? '已过期' : '未知'}
                          </span>
                        </div>
                      </div>
                    </div>
                  )}

                  {/* Business Purpose */}
                  {selectedAsset.businessPurpose && (
                    <div className="bg-blue-50 rounded-lg border border-blue-200 p-3">
                      <div className="text-[10px] text-blue-900 mb-1 font-medium">业务用途</div>
                      <div className="text-[10px] text-blue-800 leading-relaxed">
                        {selectedAsset.businessPurpose}
                      </div>
                    </div>
                  )}

                  {/* Owner */}
                  {selectedAsset.owner && (
                    <div className="bg-slate-50 rounded-lg border border-slate-200 p-2">
                      <div className="text-[9px] text-slate-500 mb-1 font-medium">负责人/团队</div>
                      <div className="text-[10px] text-slate-700 font-medium">
                        {selectedAsset.owner}
                      </div>
                    </div>
                  )}

                  {selectedAsset.status === 'scanning' && (
                    <div className="bg-purple-50 rounded-lg border border-purple-200 p-2">
                      <div className="flex items-center gap-2 mb-1">
                        <div className="w-1.5 h-1.5 bg-purple-500 rounded-full animate-pulse" />
                        <span className="text-[10px] text-purple-700 font-medium">扫描中</span>
                      </div>
                      <div className="text-[9px] text-purple-600/70 mb-1">{selectedAsset.scanPhase}</div>
                      <div className="relative h-1.5 bg-white rounded-full overflow-hidden border border-purple-100">
                        <div 
                          className="h-full bg-purple-500 transition-all" 
                          style={{ width: `${selectedAsset.scanProgress}%` }}
                        />
                      </div>
                    </div>
                  )}

                  {/* Compliance */}
                  {selectedAsset.compliance && selectedAsset.compliance.length > 0 && (
                    <div className="bg-green-50 rounded-lg border border-green-200 p-2">
                      <div className="text-[9px] text-green-900 mb-1.5 font-medium">合规标准</div>
                      <div className="flex flex-wrap gap-1">
                        {selectedAsset.compliance.map((std, i) => (
                          <span key={i} className="px-1.5 py-0.5 rounded bg-green-100 text-[9px] text-green-800 border border-green-300 font-mono">
                            {std}
                          </span>
                        ))}
                      </div>
                    </div>
                  )}
                </div>

                {/* Column 5: Actions */}
                <div className="space-y-2">
                  {onViewAIAnalysis && (
                    <button
                      onClick={(e) => { e.stopPropagation(); onViewAIAnalysis(selectedAsset); }}
                      className="w-full flex items-center justify-center gap-2 px-3 py-2 rounded-lg bg-purple-50 hover:bg-purple-100 border border-purple-200 text-xs text-purple-700 transition-all font-medium"
                    >
                      <Brain className="w-3.5 h-3.5" />
                      <span>AI 分析</span>
                    </button>
                  )}
                  
                  <button
                    onClick={(e) => { e.stopPropagation(); onScanAssets(); }}
                    className="w-full flex items-center justify-center gap-2 bg-purple-600 hover:bg-purple-700 text-white rounded-lg px-3 py-2 text-xs transition-all font-medium shadow-sm"
                  >
                    <Zap className="w-3.5 h-3.5" />
                    扫描资产
                  </button>
                  
                  <button className="w-full flex items-center justify-center gap-2 bg-slate-100 hover:bg-slate-200 border border-slate-200 rounded-lg px-3 py-2 text-xs text-slate-600 transition-all font-medium">
                    <Settings className="w-3.5 h-3.5" />
                    配置
                  </button>

                  <div className="pt-2 border-t border-slate-200">
                    <div className="text-[9px] text-slate-500 mb-1 font-medium">漏洞统计</div>
                    <div className="text-lg font-bold text-slate-900">{selectedAsset.vulns}</div>
                    <div className="text-[9px] text-slate-500">已检测漏洞</div>
                  </div>
                </div>
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
}