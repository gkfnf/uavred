import { useState } from 'react';
import { Radio, Cpu, Wifi, Signal, Power, Settings, RefreshCw, AlertTriangle, CheckCircle, XCircle, Activity, ChevronRight, Zap } from 'lucide-react';

interface HardwareDevice {
  id: string;
  name: string;
  type: 'USRP' | 'HackRF' | 'BladeRF' | 'RTL-SDR' | 'PlutoSDR';
  status: 'connected' | 'disconnected' | 'busy' | 'error';
  port?: string;
  frequency?: string;
  sampleRate?: string;
  bandwidth?: string;
  gain?: number;
  temperature?: number;
  firmwareVersion?: string;
  serialNumber?: string;
  lastUsed?: string;
  currentTask?: string;
}

interface DevicesViewProps {
  focusPanel: 'left' | 'center' | 'right';
  setFocusPanel: (panel: 'left' | 'center' | 'right') => void;
}

const MOCK_DEVICES: HardwareDevice[] = [
  {
    id: 'dev-1',
    name: 'USRP B210',
    type: 'USRP',
    status: 'busy',
    port: '/dev/ttyUSB0',
    frequency: '2.4 GHz',
    sampleRate: '20 MS/s',
    bandwidth: '56 MHz',
    gain: 45,
    temperature: 42,
    firmwareVersion: '4.1.0.0',
    serialNumber: '31F8A2B',
    lastUsed: '刚刚',
    currentTask: 'MAVLink 信号捕获',
  },
  {
    id: 'dev-2',
    name: 'HackRF One',
    type: 'HackRF',
    status: 'connected',
    port: '/dev/ttyUSB1',
    frequency: '915 MHz',
    sampleRate: '10 MS/s',
    bandwidth: '20 MHz',
    gain: 38,
    temperature: 38,
    firmwareVersion: '2022.09.1',
    serialNumber: '000000000456C2DC3C59375F',
    lastUsed: '5分钟前',
  },
  {
    id: 'dev-3',
    name: 'BladeRF x40',
    type: 'BladeRF',
    status: 'connected',
    port: '/dev/ttyUSB2',
    frequency: '5.8 GHz',
    sampleRate: '40 MS/s',
    bandwidth: '28 MHz',
    gain: 42,
    temperature: 35,
    firmwareVersion: '2.4.0',
    serialNumber: 'c44d1b1bfa0e48b88e8d3d9f1df6c01e',
    lastUsed: '1小时前',
  },
  {
    id: 'dev-4',
    name: 'RTL-SDR v3',
    type: 'RTL-SDR',
    status: 'disconnected',
    firmwareVersion: '1.0',
    serialNumber: '00000001',
    lastUsed: '昨天',
  },
  {
    id: 'dev-5',
    name: 'PlutoSDR',
    type: 'PlutoSDR',
    status: 'error',
    port: '/dev/ttyUSB3',
    frequency: '2.4 GHz',
    temperature: 51,
    firmwareVersion: 'v0.35',
    serialNumber: '104700CAFE5C0E14A80031001CF63E9A50',
    lastUsed: '10分钟前',
  },
  {
    id: 'dev-6',
    name: 'USRP B200mini',
    type: 'USRP',
    status: 'connected',
    port: '/dev/ttyUSB4',
    frequency: '433 MHz',
    sampleRate: '30.72 MS/s',
    bandwidth: '56 MHz',
    gain: 40,
    temperature: 37,
    firmwareVersion: '4.0.0.0',
    serialNumber: '31F8A3C',
    lastUsed: '2小时前',
  },
  {
    id: 'dev-7',
    name: 'HackRF One #2',
    type: 'HackRF',
    status: 'connected',
    port: '/dev/ttyUSB5',
    frequency: '1.2 GHz',
    sampleRate: '10 MS/s',
    bandwidth: '20 MHz',
    gain: 35,
    temperature: 40,
    firmwareVersion: '2022.09.1',
    serialNumber: '000000000456C2DC3C59376A',
    lastUsed: '30分钟前',
  },
];

export function DevicesView({ focusPanel, setFocusPanel }: DevicesViewProps) {
  const [selectedDevice, setSelectedDevice] = useState<HardwareDevice | null>(MOCK_DEVICES[0]);
  const [scanning, setScanning] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');

  const handleScanDevices = () => {
    setScanning(true);
    setTimeout(() => {
      setScanning(false);
    }, 2000);
  };

  const getStatusIcon = (status: HardwareDevice['status']) => {
    switch (status) {
      case 'connected':
        return <CheckCircle className="w-3.5 h-3.5 text-emerald-500" />;
      case 'busy':
        return <Activity className="w-3.5 h-3.5 text-purple-500 animate-pulse" />;
      case 'disconnected':
        return <XCircle className="w-3.5 h-3.5 text-slate-400" />;
      case 'error':
        return <AlertTriangle className="w-3.5 h-3.5 text-red-500" />;
    }
  };

  const getStatusText = (status: HardwareDevice['status']) => {
    switch (status) {
      case 'connected': return '就绪';
      case 'busy': return '忙碌';
      case 'disconnected': return '未连接';
      case 'error': return '错误';
    }
  };

  const filteredDevices = MOCK_DEVICES.filter(device =>
    device.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
    device.type.toLowerCase().includes(searchQuery.toLowerCase()) ||
    device.serialNumber?.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className="flex gap-3 h-full bg-[#FAFAFA] p-3">
      {/* Left Panel: Devices List */}
      <div className="w-96 bg-white rounded-xl overflow-hidden shadow-sm border-2 border-slate-200 flex flex-col">
        {/* Header */}
        <div className="border-b border-slate-200 px-4 py-3 bg-slate-50">
          <div className="flex items-center justify-between mb-3">
            <div className="flex items-center gap-2">
              <Radio className="w-4 h-4 text-purple-600" />
              <h2 className="text-xs font-medium text-slate-800">硬件设备</h2>
            </div>
            <div className="flex items-center gap-2">
              <span className="px-2 py-0.5 bg-emerald-50 text-emerald-700 text-[10px] rounded-full font-medium border border-emerald-200">
                {MOCK_DEVICES.filter(d => d.status === 'connected' || d.status === 'busy').length} 已连接
              </span>
            </div>
          </div>

          {/* Search Bar */}
          <div className="relative">
            <input
              type="text"
              placeholder="搜索设备..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full px-3 py-1.5 bg-white border border-slate-200 rounded-lg text-xs text-slate-700 placeholder:text-slate-400 focus:outline-none focus:ring-2 focus:ring-purple-500/20 focus:border-purple-400"
            />
          </div>

          {/* Action Buttons */}
          <div className="flex gap-2 mt-2">
            <button
              onClick={handleScanDevices}
              disabled={scanning}
              className="flex-1 flex items-center justify-center gap-1.5 px-3 py-1.5 bg-slate-100 hover:bg-slate-200 disabled:bg-slate-50 text-slate-700 rounded-lg text-xs transition-all font-medium border border-slate-200"
            >
              <RefreshCw className={`w-3 h-3 ${scanning ? 'animate-spin' : ''}`} />
              {scanning ? '扫描中...' : '扫描设备'}
            </button>
            <button className="flex-1 px-3 py-1.5 bg-purple-600 hover:bg-purple-700 text-white rounded-lg text-xs transition-all font-medium shadow-sm">
              + 添加
            </button>
          </div>
        </div>

        {/* Devices List */}
        <div className="flex-1 overflow-auto">
          {filteredDevices.map((device) => (
            <div
              key={device.id}
              onClick={() => setSelectedDevice(device)}
              className={`px-4 py-3 border-b border-slate-100 cursor-pointer transition-all ${
                selectedDevice?.id === device.id
                  ? 'bg-purple-50 border-l-4 border-l-purple-500'
                  : 'hover:bg-slate-50 border-l-4 border-l-transparent'
              }`}
            >
              <div className="flex items-start justify-between mb-1.5">
                <div className="flex items-center gap-2">
                  {getStatusIcon(device.status)}
                  <span className="text-xs font-medium text-slate-800">{device.name}</span>
                </div>
                <span className="text-[9px] text-slate-500 font-mono">{device.type}</span>
              </div>

              <div className="flex items-center justify-between text-[10px] text-slate-600 mb-1">
                <span className={`px-1.5 py-0.5 rounded text-[9px] font-medium ${
                  device.status === 'connected' ? 'bg-emerald-100 text-emerald-700' :
                  device.status === 'busy' ? 'bg-purple-100 text-purple-700' :
                  device.status === 'disconnected' ? 'bg-slate-200 text-slate-600' :
                  'bg-red-100 text-red-700'
                }`}>
                  {getStatusText(device.status)}
                </span>
                {device.port && (
                  <span className="font-mono text-[9px] text-slate-500">{device.port}</span>
                )}
              </div>

              {device.currentTask && (
                <div className="flex items-center gap-1 mt-1.5 p-1.5 bg-purple-50 rounded border border-purple-200">
                  <Signal className="w-2.5 h-2.5 text-purple-600 animate-pulse flex-shrink-0" />
                  <span className="text-[9px] text-purple-700 font-medium truncate">{device.currentTask}</span>
                </div>
              )}

              {!device.currentTask && device.frequency && (
                <div className="text-[9px] text-slate-500 mt-1">
                  <span className="font-mono">{device.frequency}</span>
                  {device.temperature && (
                    <>
                      <span className="mx-1">•</span>
                      <span className={device.temperature > 50 ? 'text-red-600 font-medium' : ''}>
                        {device.temperature}°C
                      </span>
                    </>
                  )}
                </div>
              )}

              {selectedDevice?.id === device.id && (
                <ChevronRight className="absolute right-2 top-1/2 -translate-y-1/2 w-4 h-4 text-purple-500" />
              )}
            </div>
          ))}
        </div>

        {/* Stats Footer */}
        <div className="border-t border-slate-200 px-4 py-2 bg-slate-50">
          <div className="flex items-center justify-between text-[9px] text-slate-500">
            <span>{filteredDevices.length} 设备</span>
            <span>最后扫描: 刚刚</span>
          </div>
        </div>
      </div>

      {/* Right Panel: Device Details & Control */}
      {selectedDevice && (
        <div className="flex-1 bg-white rounded-xl overflow-hidden shadow-sm border-2 border-slate-200 flex flex-col">
          {/* Header */}
          <div className="border-b border-slate-200 px-4 py-3 bg-slate-50">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-3">
                <Radio className="w-4 h-4 text-purple-600" />
                <div>
                  <h3 className="text-xs font-bold text-slate-800">{selectedDevice.name}</h3>
                  <div className="flex items-center gap-2 mt-0.5">
                    <span className="text-[10px] text-slate-500 font-mono">{selectedDevice.type}</span>
                    <span className="text-slate-300">•</span>
                    <div className="flex items-center gap-1">
                      {getStatusIcon(selectedDevice.status)}
                      <span className="text-[10px] text-slate-600">{getStatusText(selectedDevice.status)}</span>
                    </div>
                  </div>
                </div>
              </div>
              <div className="flex items-center gap-2">
                <button className="p-2 hover:bg-slate-200 rounded-lg transition-all text-slate-600">
                  <Settings className="w-4 h-4" />
                </button>
              </div>
            </div>
          </div>

          {/* Content */}
          <div className="flex-1 overflow-auto p-4">
            {/* Current Task */}
            {selectedDevice.currentTask && (
              <div className="mb-4 p-3 bg-purple-50 rounded-lg border-2 border-purple-200">
                <div className="flex items-center gap-2 mb-2">
                  <Signal className="w-4 h-4 text-purple-600 animate-pulse" />
                  <span className="text-xs text-purple-900 font-medium">当前任务</span>
                </div>
                <div className="text-sm text-purple-700 font-medium mb-3">{selectedDevice.currentTask}</div>
                <div className="flex gap-2">
                  <button className="flex-1 flex items-center justify-center gap-1.5 px-3 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg text-xs transition-all font-medium">
                    <Power className="w-3.5 h-3.5" />
                    停止任务
                  </button>
                  <button className="flex-1 flex items-center justify-center gap-1.5 px-3 py-2 bg-slate-600 hover:bg-slate-700 text-white rounded-lg text-xs transition-all font-medium">
                    <Activity className="w-3.5 h-3.5" />
                    查看日志
                  </button>
                </div>
              </div>
            )}

            {/* Device Info Grid */}
            <div className="mb-4">
              <div className="text-xs text-slate-500 mb-2 font-medium">设备信息</div>
              <div className="grid grid-cols-3 gap-3">
                <div className="bg-slate-50 rounded-lg p-3 border border-slate-200">
                  <div className="text-[9px] text-slate-500 mb-1">序列号</div>
                  <div className="text-[10px] font-mono text-slate-800 break-all">{selectedDevice.serialNumber}</div>
                </div>
                <div className="bg-slate-50 rounded-lg p-3 border border-slate-200">
                  <div className="text-[9px] text-slate-500 mb-1">固件版本</div>
                  <div className="text-xs font-mono text-slate-800">{selectedDevice.firmwareVersion}</div>
                </div>
                <div className="bg-slate-50 rounded-lg p-3 border border-slate-200">
                  <div className="text-[9px] text-slate-500 mb-1">端口</div>
                  <div className="text-xs font-mono text-slate-800">{selectedDevice.port || 'N/A'}</div>
                </div>
              </div>
            </div>

            {/* Radio Parameters */}
            {(selectedDevice.frequency || selectedDevice.sampleRate) && (
              <div className="mb-4">
                <div className="text-xs text-slate-500 mb-2 font-medium">无线参数</div>
                <div className="grid grid-cols-2 gap-3">
                  {selectedDevice.frequency && (
                    <div className="bg-blue-50 rounded-lg p-3 border border-blue-200">
                      <div className="text-[9px] text-blue-900 mb-1">频率</div>
                      <div className="text-sm font-bold text-blue-700 font-mono">{selectedDevice.frequency}</div>
                    </div>
                  )}
                  {selectedDevice.sampleRate && (
                    <div className="bg-green-50 rounded-lg p-3 border border-green-200">
                      <div className="text-[9px] text-green-900 mb-1">采样率</div>
                      <div className="text-sm font-bold text-green-700 font-mono">{selectedDevice.sampleRate}</div>
                    </div>
                  )}
                  {selectedDevice.bandwidth && (
                    <div className="bg-purple-50 rounded-lg p-3 border border-purple-200">
                      <div className="text-[9px] text-purple-900 mb-1">带宽</div>
                      <div className="text-sm font-bold text-purple-700 font-mono">{selectedDevice.bandwidth}</div>
                    </div>
                  )}
                  {selectedDevice.gain !== undefined && (
                    <div className="bg-orange-50 rounded-lg p-3 border border-orange-200">
                      <div className="text-[9px] text-orange-900 mb-1">增益</div>
                      <div className="text-sm font-bold text-orange-700 font-mono">{selectedDevice.gain} dB</div>
                    </div>
                  )}
                </div>
              </div>
            )}

            {/* Temperature Monitor */}
            {selectedDevice.temperature !== undefined && (
              <div className="mb-4">
                <div className="text-xs text-slate-500 mb-2 font-medium">设备状态</div>
                <div className={`rounded-lg p-3 border-2 ${
                  selectedDevice.temperature > 50 ? 'bg-red-50 border-red-300' :
                  selectedDevice.temperature > 45 ? 'bg-yellow-50 border-yellow-300' :
                  'bg-emerald-50 border-emerald-300'
                }`}>
                  <div className="flex items-center justify-between mb-2">
                    <div className="flex items-center gap-2">
                      <Cpu className={`w-4 h-4 ${
                        selectedDevice.temperature > 50 ? 'text-red-600' :
                        selectedDevice.temperature > 45 ? 'text-yellow-600' :
                        'text-emerald-600'
                      }`} />
                      <span className="text-xs font-medium text-slate-700">温度</span>
                    </div>
                    <span className={`text-2xl font-bold font-mono ${
                      selectedDevice.temperature > 50 ? 'text-red-700' :
                      selectedDevice.temperature > 45 ? 'text-yellow-700' :
                      'text-emerald-700'
                    }`}>
                      {selectedDevice.temperature}°C
                    </span>
                  </div>
                  <div className="relative h-2 bg-white rounded-full overflow-hidden">
                    <div
                      className={`h-full transition-all ${
                        selectedDevice.temperature > 50 ? 'bg-red-500' :
                        selectedDevice.temperature > 45 ? 'bg-yellow-500' :
                        'bg-emerald-500'
                      }`}
                      style={{ width: `${Math.min((selectedDevice.temperature / 80) * 100, 100)}%` }}
                    />
                  </div>
                  {selectedDevice.temperature > 50 && (
                    <div className="mt-2 flex items-center gap-1.5 text-[10px] text-red-700">
                      <AlertTriangle className="w-3 h-3" />
                      <span className="font-medium">警告：设备温度过高，建议降低工作负载</span>
                    </div>
                  )}
                </div>
              </div>
            )}

            {/* Quick Actions */}
            <div className="mb-4">
              <div className="text-xs text-slate-500 mb-2 font-medium">快速操作</div>
              <div className="grid grid-cols-2 gap-2">
                {selectedDevice.status === 'connected' && (
                  <>
                    <button className="flex items-center justify-center gap-2 px-4 py-3 bg-purple-600 hover:bg-purple-700 text-white rounded-lg text-xs transition-all font-medium shadow-sm">
                      <Zap className="w-4 h-4" />
                      启动扫描任务
                    </button>
                    <button className="flex items-center justify-center gap-2 px-4 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-xs transition-all font-medium shadow-sm">
                      <Activity className="w-4 h-4" />
                      信号监听
                    </button>
                  </>
                )}
                {selectedDevice.status === 'disconnected' && (
                  <button className="col-span-2 flex items-center justify-center gap-2 px-4 py-3 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg text-xs transition-all font-medium shadow-sm">
                    <Power className="w-4 h-4" />
                    连接设备
                  </button>
                )}
                {selectedDevice.status === 'error' && (
                  <button className="col-span-2 flex items-center justify-center gap-2 px-4 py-3 bg-red-600 hover:bg-red-700 text-white rounded-lg text-xs transition-all font-medium shadow-sm">
                    <RefreshCw className="w-4 h-4" />
                    重新连接
                  </button>
                )}
                <button className="flex items-center justify-center gap-2 px-4 py-3 bg-slate-100 hover:bg-slate-200 border border-slate-200 text-slate-700 rounded-lg text-xs transition-all font-medium">
                  <Settings className="w-4 h-4" />
                  配置参数
                </button>
                <button className="flex items-center justify-center gap-2 px-4 py-3 bg-slate-100 hover:bg-slate-200 border border-slate-200 text-slate-700 rounded-lg text-xs transition-all font-medium">
                  <RefreshCw className="w-4 h-4" />
                  固件更新
                </button>
              </div>
            </div>

            {/* Usage Info */}
            <div className="bg-slate-50 rounded-lg p-3 border border-slate-200">
              <div className="text-[9px] text-slate-500 mb-2">使用信息</div>
              <div className="space-y-1 text-[10px] text-slate-600">
                <div className="flex justify-between">
                  <span>最后使用</span>
                  <span className="font-medium">{selectedDevice.lastUsed}</span>
                </div>
                <div className="flex justify-between">
                  <span>累计运行时长</span>
                  <span className="font-medium font-mono">127.5 小时</span>
                </div>
                <div className="flex justify-between">
                  <span>完成任务数</span>
                  <span className="font-medium">34 个</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
