import { useState } from 'react';
import { Box, Play, Pause, Terminal, Monitor, Cpu, HardDrive, Network, X, Maximize2, ExternalLink, Activity } from 'lucide-react';

interface ContainerImage {
  id: string;
  name: string;
  tag: string;
  status: 'running' | 'stopped' | 'building';
  agent?: string;
  task?: string;
  cpuUsage: number;
  memoryUsage: number;
  uptime: string;
  ports: number[];
  vncPort?: number;
}

interface ImagesViewProps {
  focusPanel: 'left' | 'center' | 'right';
  setFocusPanel: (panel: 'left' | 'center' | 'right') => void;
}

const MOCK_IMAGES: ContainerImage[] = [
  {
    id: 'img-1',
    name: 'ai-pentest-agent',
    tag: 'v2.3.1',
    status: 'running',
    agent: 'Agent-Alpha',
    task: 'CVE-2024-1234 PoC 生成',
    cpuUsage: 67,
    memoryUsage: 45,
    uptime: '2h 34m',
    ports: [8080, 5900],
    vncPort: 5900,
  },
  {
    id: 'img-2',
    name: 'network-scanner',
    tag: 'v1.8.0',
    status: 'running',
    agent: 'Agent-Beta',
    task: '网络拓扑扫描',
    cpuUsage: 23,
    memoryUsage: 32,
    uptime: '45m',
    ports: [8081, 5901],
    vncPort: 5901,
  },
  {
    id: 'img-3',
    name: 'exploit-dev-env',
    tag: 'v3.1.0',
    status: 'running',
    agent: 'Agent-Gamma',
    task: 'DJI Mavic 漏洞利用开发',
    cpuUsage: 89,
    memoryUsage: 78,
    uptime: '1h 12m',
    ports: [8082, 5902],
    vncPort: 5902,
  },
  {
    id: 'img-4',
    name: 'fuzzing-toolkit',
    tag: 'v2.0.5',
    status: 'stopped',
    cpuUsage: 0,
    memoryUsage: 0,
    uptime: '-',
    ports: [],
  },
  {
    id: 'img-5',
    name: 'payload-generator',
    tag: 'v1.5.2',
    status: 'running',
    agent: 'Agent-Delta',
    task: 'Payload 模糊测试',
    cpuUsage: 45,
    memoryUsage: 38,
    uptime: '3h 21m',
    ports: [8083, 5903],
    vncPort: 5903,
  },
  {
    id: 'img-6',
    name: 'traffic-analyzer',
    tag: 'v2.1.0',
    status: 'building',
    cpuUsage: 12,
    memoryUsage: 15,
    uptime: '5m',
    ports: [],
  },
];

export function ImagesView({ focusPanel, setFocusPanel }: ImagesViewProps) {
  const [selectedImage, setSelectedImage] = useState<ContainerImage | null>(null);
  const [vncConnecting, setVncConnecting] = useState<string | null>(null);

  const handleVncConnect = (image: ContainerImage) => {
    if (!image.vncPort) return;
    setVncConnecting(image.id);
    
    // Simulate VNC connection
    setTimeout(() => {
      setVncConnecting(null);
      // In real implementation, this would open VNC viewer
      alert(`正在连接到 VNC: ${image.name}\n端口: ${image.vncPort}\nAgent: ${image.agent}\n任务: ${image.task}`);
    }, 1500);
  };

  return (
    <div className="flex flex-col gap-3 h-full bg-[#FAFAFA] p-3">
      {/* Header */}
      <div className="bg-white rounded-xl p-4 shadow-sm border-2 border-slate-200">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Box className="w-4 h-4 text-purple-600" />
            <h2 className="text-sm font-medium text-slate-800">容器镜像 & Agent 执行环境</h2>
            <span className="px-2 py-0.5 bg-purple-50 text-purple-700 text-xs rounded-full font-medium border border-purple-200">
              {MOCK_IMAGES.filter(i => i.status === 'running').length} 运行中
            </span>
          </div>
          <div className="flex items-center gap-2">
            <button className="px-3 py-1.5 bg-purple-600 hover:bg-purple-700 text-white rounded-lg text-xs transition-all font-medium shadow-sm">
              + 创建镜像
            </button>
          </div>
        </div>
      </div>

      {/* Images Grid */}
      <div className="flex-1 overflow-auto">
        <div className="grid grid-cols-3 gap-3">
          {MOCK_IMAGES.map((image) => (
            <div
              key={image.id}
              className={`bg-white rounded-xl overflow-hidden shadow-sm border-2 transition-all cursor-pointer ${
                selectedImage?.id === image.id
                  ? 'border-purple-400 shadow-lg scale-[1.02]'
                  : 'border-slate-200 hover:border-purple-200 hover:shadow-md'
              }`}
              onClick={() => setSelectedImage(image)}
            >
              {/* Preview/Thumbnail Area */}
              <div className={`relative h-40 flex items-center justify-center ${
                image.status === 'running' ? 'bg-gradient-to-br from-purple-50 to-blue-50' :
                image.status === 'building' ? 'bg-gradient-to-br from-yellow-50 to-orange-50' :
                'bg-gradient-to-br from-slate-50 to-slate-100'
              }`}>
                {/* Status Badge */}
                <div className="absolute top-2 right-2">
                  <div className={`flex items-center gap-1.5 px-2 py-1 rounded-full text-[9px] font-medium ${
                    image.status === 'running' ? 'bg-emerald-500 text-white' :
                    image.status === 'building' ? 'bg-yellow-500 text-white' :
                    'bg-slate-400 text-white'
                  }`}>
                    <div className={`w-1.5 h-1.5 rounded-full ${
                      image.status === 'running' ? 'bg-white animate-pulse' : 'bg-white'
                    }`} />
                    {image.status === 'running' ? 'RUNNING' : 
                     image.status === 'building' ? 'BUILDING' : 'STOPPED'}
                  </div>
                </div>

                {/* VNC Button (only for running containers with VNC) */}
                {image.status === 'running' && image.vncPort && (
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      handleVncConnect(image);
                    }}
                    disabled={vncConnecting === image.id}
                    className="absolute inset-0 bg-black/0 hover:bg-black/60 transition-all group"
                  >
                    <div className="flex flex-col items-center justify-center gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                      {vncConnecting === image.id ? (
                        <>
                          <div className="w-8 h-8 border-3 border-white border-t-transparent rounded-full animate-spin" />
                          <span className="text-white text-xs font-medium">连接中...</span>
                        </>
                      ) : (
                        <>
                          <Monitor className="w-8 h-8 text-white" />
                          <span className="text-white text-xs font-medium">点击进入 VNC</span>
                          <span className="text-white/70 text-[10px]">:{image.vncPort}</span>
                        </>
                      )}
                    </div>
                  </button>
                )}

                {/* Icon when not running */}
                {image.status !== 'running' && (
                  <Box className={`w-16 h-16 ${
                    image.status === 'building' ? 'text-yellow-300 animate-pulse' : 'text-slate-300'
                  }`} />
                )}

                {/* Terminal Preview for running containers */}
                {image.status === 'running' && (
                  <div className="absolute inset-4 bg-slate-900/90 rounded-lg p-2 font-mono text-[8px] leading-tight overflow-hidden">
                    <div className="text-emerald-400">$ docker exec -it {image.id.substring(0, 12)}</div>
                    <div className="text-slate-400 mt-1">Agent: {image.agent}</div>
                    <div className="text-purple-400 mt-1">Task: {image.task}</div>
                    <div className="text-slate-500 mt-2">[{image.uptime}] Running...</div>
                    <div className="absolute bottom-1 right-1 flex items-center gap-1">
                      <Activity className="w-2 h-2 text-emerald-500 animate-pulse" />
                      <span className="text-emerald-500">{image.cpuUsage}%</span>
                    </div>
                  </div>
                )}
              </div>

              {/* Info Area */}
              <div className="p-3 border-t border-slate-200">
                <div className="flex items-start justify-between mb-2">
                  <div>
                    <div className="font-mono text-xs font-medium text-slate-800">{image.name}</div>
                    <div className="text-[10px] text-slate-500 font-mono">{image.tag}</div>
                  </div>
                  {image.status === 'running' ? (
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                      }}
                      className="p-1 hover:bg-red-50 rounded transition-colors"
                    >
                      <Pause className="w-3.5 h-3.5 text-red-600" />
                    </button>
                  ) : image.status === 'stopped' ? (
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                      }}
                      className="p-1 hover:bg-emerald-50 rounded transition-colors"
                    >
                      <Play className="w-3.5 h-3.5 text-emerald-600" />
                    </button>
                  ) : null}
                </div>

                {image.agent && (
                  <div className="mb-2">
                    <div className="text-[9px] text-slate-500 mb-0.5">Agent</div>
                    <div className="text-[10px] text-purple-700 font-medium bg-purple-50 px-2 py-0.5 rounded border border-purple-200">
                      {image.agent}
                    </div>
                  </div>
                )}

                {image.task && (
                  <div className="mb-2">
                    <div className="text-[9px] text-slate-500 mb-0.5">当前任务</div>
                    <div className="text-[10px] text-slate-700 leading-tight">{image.task}</div>
                  </div>
                )}

                {/* Resource Usage */}
                {image.status === 'running' && (
                  <div className="space-y-1.5 mt-2 pt-2 border-t border-slate-100">
                    <div>
                      <div className="flex items-center justify-between text-[9px] mb-0.5">
                        <span className="text-slate-500">CPU</span>
                        <span className="text-slate-700 font-mono">{image.cpuUsage}%</span>
                      </div>
                      <div className="h-1 bg-slate-100 rounded-full overflow-hidden">
                        <div
                          className={`h-full transition-all ${
                            image.cpuUsage > 80 ? 'bg-red-500' :
                            image.cpuUsage > 50 ? 'bg-yellow-500' :
                            'bg-emerald-500'
                          }`}
                          style={{ width: `${image.cpuUsage}%` }}
                        />
                      </div>
                    </div>
                    <div>
                      <div className="flex items-center justify-between text-[9px] mb-0.5">
                        <span className="text-slate-500">Memory</span>
                        <span className="text-slate-700 font-mono">{image.memoryUsage}%</span>
                      </div>
                      <div className="h-1 bg-slate-100 rounded-full overflow-hidden">
                        <div
                          className={`h-full transition-all ${
                            image.memoryUsage > 80 ? 'bg-red-500' :
                            image.memoryUsage > 50 ? 'bg-yellow-500' :
                            'bg-blue-500'
                          }`}
                          style={{ width: `${image.memoryUsage}%` }}
                        />
                      </div>
                    </div>
                  </div>
                )}

                {/* Ports */}
                {image.ports.length > 0 && (
                  <div className="mt-2 pt-2 border-t border-slate-100">
                    <div className="text-[9px] text-slate-500 mb-1">暴露端口</div>
                    <div className="flex flex-wrap gap-1">
                      {image.ports.map(port => (
                        <span key={port} className="px-1.5 py-0.5 bg-slate-100 text-[9px] text-slate-700 rounded font-mono border border-slate-200">
                          {port}
                        </span>
                      ))}
                    </div>
                  </div>
                )}

                {/* Uptime */}
                <div className="mt-2 text-[9px] text-slate-500 flex items-center justify-between">
                  <span>运行时长</span>
                  <span className="font-mono text-slate-700">{image.uptime}</span>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Selected Image Details (Optional) */}
      {selectedImage && (
        <div className="bg-white rounded-xl p-4 shadow-sm border-2 border-purple-200">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-2">
              <Terminal className="w-4 h-4 text-purple-600" />
              <span className="text-xs font-medium text-slate-800">详细信息: {selectedImage.name}</span>
            </div>
            <button
              onClick={() => setSelectedImage(null)}
              className="p-1 hover:bg-slate-100 rounded transition-colors"
            >
              <X className="w-4 h-4 text-slate-500" />
            </button>
          </div>
          
          <div className="grid grid-cols-4 gap-3">
            <div className="bg-slate-50 rounded-lg p-2 border border-slate-200">
              <div className="flex items-center gap-1.5 mb-1">
                <Cpu className="w-3 h-3 text-slate-500" />
                <span className="text-[9px] text-slate-500">CPU 使用率</span>
              </div>
              <div className="text-lg font-bold text-slate-900">{selectedImage.cpuUsage}%</div>
            </div>
            <div className="bg-slate-50 rounded-lg p-2 border border-slate-200">
              <div className="flex items-center gap-1.5 mb-1">
                <HardDrive className="w-3 h-3 text-slate-500" />
                <span className="text-[9px] text-slate-500">内存使用</span>
              </div>
              <div className="text-lg font-bold text-slate-900">{selectedImage.memoryUsage}%</div>
            </div>
            <div className="bg-slate-50 rounded-lg p-2 border border-slate-200">
              <div className="flex items-center gap-1.5 mb-1">
                <Network className="w-3 h-3 text-slate-500" />
                <span className="text-[9px] text-slate-500">暴露端口</span>
              </div>
              <div className="text-lg font-bold text-slate-900">{selectedImage.ports.length}</div>
            </div>
            <div className="bg-slate-50 rounded-lg p-2 border border-slate-200">
              <div className="flex items-center gap-1.5 mb-1">
                <Activity className="w-3 h-3 text-slate-500" />
                <span className="text-[9px] text-slate-500">运行时长</span>
              </div>
              <div className="text-sm font-bold text-slate-900">{selectedImage.uptime}</div>
            </div>
          </div>

          {selectedImage.vncPort && selectedImage.status === 'running' && (
            <button
              onClick={() => handleVncConnect(selectedImage)}
              disabled={vncConnecting === selectedImage.id}
              className="w-full mt-3 flex items-center justify-center gap-2 px-4 py-2 bg-purple-600 hover:bg-purple-700 disabled:bg-purple-400 text-white rounded-lg text-xs transition-all font-medium"
            >
              {vncConnecting === selectedImage.id ? (
                <>
                  <div className="w-3 h-3 border-2 border-white border-t-transparent rounded-full animate-spin" />
                  <span>连接中...</span>
                </>
              ) : (
                <>
                  <ExternalLink className="w-3.5 h-3.5" />
                  <span>打开 VNC 连接 (:{selectedImage.vncPort})</span>
                </>
              )}
            </button>
          )}
        </div>
      )}
    </div>
  );
}
