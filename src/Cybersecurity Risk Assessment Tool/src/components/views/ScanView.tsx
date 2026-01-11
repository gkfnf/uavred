import { Activity, Play, Pause, X, Terminal, Clock, CheckCircle, XCircle, AlertCircle } from 'lucide-react';

interface ScanLog {
  time: string;
  level: 'info' | 'success' | 'warning' | 'error';
  message: string;
}

interface ScanJob {
  id: number;
  name: string;
  assets: number[];
  status: 'idle' | 'running' | 'paused' | 'completed' | 'failed';
  progress: number;
  startTime: string;
  endTime?: string;
  currentAsset?: number;
  logs: ScanLog[];
  results: {
    vulnsFound: number;
    portsScanned: number;
    servicesDetected: number;
  };
}

interface Asset {
  id: number;
  name: string;
  ip: string;
  scanProgress: number;
  scanPhase: string;
  status: string;
}

interface ScanViewProps {
  scanJobs: ScanJob[];
  selectedScanJobIdx: number;
  setSelectedScanJobIdx: (idx: number) => void;
  assets: Asset[];
  onPauseScan: () => void;
  onCancelScan: () => void;
  focusPanel: 'left' | 'center' | 'right';
  setFocusPanel: (panel: 'left' | 'center' | 'right') => void;
}

export function ScanView({
  scanJobs,
  selectedScanJobIdx,
  setSelectedScanJobIdx,
  assets,
  onPauseScan,
  onCancelScan,
  focusPanel,
  setFocusPanel
}: ScanViewProps) {
  const getPanelStyle = (panel: 'left' | 'center' | 'right') => `
    border-2 transition-all
    ${focusPanel === panel 
      ? 'border-purple-400/50 shadow-md ring-1 ring-purple-100' 
      : 'border-slate-200'
    }
  `;

  const currentJob = scanJobs[selectedScanJobIdx];
  const scanAssets = currentJob ? assets.filter(a => currentJob.assets.includes(a.id)) : [];

  return (
    <div className="flex gap-3 h-full bg-[#FAFAFA] p-3">
      {/* Left: Scan Jobs List */}
      <div 
        className={`w-80 bg-white rounded-xl overflow-hidden flex flex-col shadow-sm ${getPanelStyle('left')}`}
        onClick={() => setFocusPanel('left')}
      >
        <div className="border-b border-slate-200 px-3 py-2.5 bg-slate-50">
          <span className="text-xs text-slate-700 font-medium">Scan Jobs ({scanJobs.length})</span>
        </div>
        <div className="p-2 space-y-2 overflow-auto flex-1 bg-[#FAFAFA]">
          {scanJobs.map((job, idx) => (
            <div
              key={job.id}
              onClick={() => setSelectedScanJobIdx(idx)}
              className={`rounded-lg p-3 cursor-pointer transition-all border shadow-sm ${
                selectedScanJobIdx === idx
                  ? 'bg-white border-purple-400 ring-1 ring-purple-400/30 shadow-md'
                  : 'bg-white border-slate-200 hover:border-purple-300 hover:shadow-md'
              }`}
            >
              <div className="flex items-center justify-between mb-2">
                <span className="text-xs text-slate-800 font-medium">{job.name}</span>
                <div className={`w-2 h-2 rounded-full ${
                  job.status === 'running' ? 'bg-purple-500 animate-pulse' :
                  job.status === 'completed' ? 'bg-emerald-500' :
                  job.status === 'failed' ? 'bg-red-500' : 'bg-yellow-500'
                }`} />
              </div>
              <div className="text-[10px] text-slate-500 mb-2">{job.assets.length} assets • {job.startTime.split(' ')[1]}</div>
              <div className="relative h-1.5 bg-slate-100 rounded-full overflow-hidden border border-slate-200">
                <div
                  className={`h-full transition-all ${
                    job.status === 'completed' ? 'bg-emerald-500' :
                    job.status === 'failed' ? 'bg-red-500' :
                    'bg-purple-500'
                  }`}
                  style={{ width: `${job.progress}%` }}
                />
              </div>
              <div className="flex items-center justify-between mt-1.5 text-[9px]">
                <span className="text-slate-500 font-mono">{job.progress}%</span>
                <span className={`font-medium ${
                  job.status === 'running' ? 'text-purple-600' :
                  job.status === 'completed' ? 'text-emerald-600' :
                  job.status === 'failed' ? 'text-red-600' : 'text-yellow-600'
                }`}>
                  {job.status.toUpperCase()}
                </span>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Center: Scan Progress */}
      <div 
        className={`flex-1 bg-white rounded-xl overflow-hidden flex flex-col shadow-sm ${getPanelStyle('center')}`}
        onClick={() => setFocusPanel('center')}
      >
        {currentJob ? (
          <>
            <div className="border-b border-slate-200 px-4 py-2.5 bg-slate-50 flex items-center justify-between">
              <span className="text-xs text-slate-700 font-medium">Scan Progress</span>
              <div className="flex items-center gap-2">
                {currentJob.status === 'running' && (
                  <>
                    <button
                      onClick={(e) => { e.stopPropagation(); onPauseScan(); }}
                      className="p-1.5 hover:bg-slate-200 rounded transition-all"
                      title="Pause"
                    >
                      <Pause className="w-3.5 h-3.5 text-yellow-500" />
                    </button>
                    <button
                      onClick={(e) => { e.stopPropagation(); onCancelScan(); }}
                      className="p-1.5 hover:bg-red-50 rounded transition-all"
                      title="Cancel"
                    >
                      <X className="w-3.5 h-3.5 text-red-500" />
                    </button>
                  </>
                )}
                {currentJob.status === 'paused' && (
                  <button
                    onClick={(e) => { e.stopPropagation(); onPauseScan(); }}
                    className="p-1.5 hover:bg-slate-200 rounded transition-all"
                    title="Resume"
                  >
                    <Play className="w-3.5 h-3.5 text-purple-600" />
                  </button>
                )}
              </div>
            </div>
            <div className="p-4 space-y-4 overflow-auto flex-1 bg-white">
              {/* Overall Progress */}
              <div>
                <div className="flex items-center justify-between mb-2">
                  <span className="text-sm text-slate-700 font-medium">Overall Progress</span>
                  <span className="text-sm text-slate-500 font-mono">{currentJob.progress}%</span>
                </div>
                <div className="relative h-4 bg-slate-100 rounded-full overflow-hidden border border-slate-200 shadow-inner">
                  <div
                    className="h-full bg-gradient-to-r from-purple-400 via-purple-500 to-indigo-500 transition-all"
                    style={{ width: `${currentJob.progress}%` }}
                  />
                </div>
              </div>

              {/* Results Summary */}
              <div className="grid grid-cols-3 gap-3">
                <div className="bg-slate-50 rounded-lg p-3 border border-slate-200">
                  <div className="text-[10px] text-slate-500 mb-1 font-medium">Vulnerabilities</div>
                  <div className="text-2xl text-red-500 font-bold">{currentJob.results.vulnsFound}</div>
                </div>
                <div className="bg-slate-50 rounded-lg p-3 border border-slate-200">
                  <div className="text-[10px] text-slate-500 mb-1 font-medium">Ports Scanned</div>
                  <div className="text-2xl text-purple-600 font-bold">{currentJob.results.portsScanned}</div>
                </div>
                <div className="bg-slate-50 rounded-lg p-3 border border-slate-200">
                  <div className="text-[10px] text-slate-500 mb-1 font-medium">Services</div>
                  <div className="text-2xl text-blue-500 font-bold">{currentJob.results.servicesDetected}</div>
                </div>
              </div>

              {/* Per-Asset Progress */}
              <div>
                <div className="text-xs text-slate-700 font-medium mb-2">Asset Progress</div>
                <div className="space-y-2">
                  {scanAssets.map((asset) => (
                    <div key={asset.id} className="bg-slate-50 rounded-lg p-3 border border-slate-200 shadow-sm">
                      <div className="flex items-center justify-between mb-2">
                        <div className="flex items-center gap-2">
                          <div className={`w-1.5 h-1.5 rounded-full ${
                            asset.status === 'scanning' ? 'bg-purple-500 animate-pulse' : 'bg-emerald-500'
                          }`} />
                          <span className="text-xs text-slate-800 font-medium">{asset.name}</span>
                        </div>
                        <span className="text-xs text-slate-500 font-mono">{asset.ip}</span>
                      </div>
                      <div className="flex items-center gap-2 mb-1">
                        <div className="flex-1 h-1.5 bg-white rounded-full overflow-hidden border border-slate-200">
                          <div
                            className="h-full bg-purple-500 transition-all"
                            style={{ width: `${asset.scanProgress}%` }}
                          />
                        </div>
                        <span className="text-[10px] text-slate-500 w-8 text-right font-mono">{asset.scanProgress}%</span>
                      </div>
                      <div className="text-[10px] text-purple-600 font-medium">{asset.scanPhase}</div>
                    </div>
                  ))}
                </div>
              </div>

              {/* Status */}
              <div className={`rounded-lg p-3 border ${
                currentJob.status === 'running' ? 'bg-purple-50 border-purple-200' :
                currentJob.status === 'completed' ? 'bg-emerald-50 border-emerald-200' :
                currentJob.status === 'failed' ? 'bg-red-50 border-red-200' :
                'bg-yellow-50 border-yellow-200'
              }`}>
                <div className="flex items-center gap-2">
                  {currentJob.status === 'running' && <Activity className="w-4 h-4 text-purple-600 animate-pulse" />}
                  {currentJob.status === 'completed' && <CheckCircle className="w-4 h-4 text-emerald-600" />}
                  {currentJob.status === 'failed' && <XCircle className="w-4 h-4 text-red-600" />}
                  {currentJob.status === 'paused' && <Pause className="w-4 h-4 text-yellow-600" />}
                  <div className="flex-1">
                    <div className={`text-xs font-medium ${
                      currentJob.status === 'running' ? 'text-purple-700' :
                      currentJob.status === 'completed' ? 'text-emerald-700' :
                      currentJob.status === 'failed' ? 'text-red-700' :
                      'text-yellow-700'
                    }`}>
                      {currentJob.status === 'running' ? 'Scan in progress...' :
                       currentJob.status === 'completed' ? 'Scan completed successfully' :
                       currentJob.status === 'failed' ? 'Scan cancelled' :
                       'Scan paused'}
                    </div>
                    <div className="text-[10px] text-slate-500 mt-0.5">
                      Started: {currentJob.startTime}
                      {currentJob.endTime && ` • Ended: ${currentJob.endTime}`}
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </>
        ) : (
          <div className="flex items-center justify-center h-full">
            <div className="text-center text-slate-400">
              <Activity className="w-12 h-12 mx-auto mb-3 opacity-20" />
              <div className="text-sm font-medium text-slate-500">No scan jobs yet</div>
            </div>
          </div>
        )}
      </div>

      {/* Right: Live Logs */}
      <div 
        className={`w-96 bg-[#282A36] rounded-xl overflow-hidden flex flex-col shadow-lg border border-slate-700 ${getPanelStyle('right')}`}
        onClick={() => setFocusPanel('right')}
      >
        <div className="border-b border-slate-700/50 px-3 py-2.5 bg-[#44475A] flex items-center gap-2">
          <Terminal className="w-3.5 h-3.5 text-slate-300" />
          <span className="text-xs text-slate-200 font-medium">Live Logs</span>
          {currentJob && currentJob.status === 'running' && (
            <div className="w-1.5 h-1.5 bg-green-400 rounded-full animate-pulse ml-auto" />
          )}
        </div>
        <div className="p-3 overflow-auto font-mono flex-1 bg-[#282A36]" style={{ height: 'calc(100% - 41px)' }}>
          {currentJob && currentJob.logs.length > 0 ? (
            <div className="space-y-1">
              {currentJob.logs.slice().reverse().map((log, idx) => (
                <div key={idx} className="flex items-start gap-2 text-[10px]">
                  <span className="text-slate-500 font-mono">[{log.time}]</span>
                  {log.level === 'info' && <span className="text-blue-400">ℹ</span>}
                  {log.level === 'success' && <span className="text-green-400">✓</span>}
                  {log.level === 'warning' && <span className="text-yellow-400">⚠</span>}
                  {log.level === 'error' && <span className="text-red-400">✗</span>}
                  <span className={`flex-1 break-words ${
                    log.level === 'info' ? 'text-slate-300' :
                    log.level === 'success' ? 'text-green-400' :
                    log.level === 'warning' ? 'text-yellow-400' :
                    'text-red-400'
                  }`}>
                    {log.message}
                  </span>
                </div>
              ))}
            </div>
          ) : (
            <div className="text-center text-slate-500 py-8 text-xs">
              <Terminal className="w-8 h-8 mx-auto mb-2 opacity-50" />
              <div>No logs yet</div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
