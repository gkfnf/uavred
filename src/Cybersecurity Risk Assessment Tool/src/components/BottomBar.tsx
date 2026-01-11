import { Activity, Cpu, HardDrive, Wifi } from 'lucide-react';

interface BottomBarProps {
  view: string;
  focusPanel: 'left' | 'center' | 'right';
  stats: {
    assets: { total: number };
    vulns: { total: number };
    traffic: { captured: number };
  };
  onHelpClick?: () => void;
  onExportClick?: () => void;
  aiOpsPerSecond?: number;
}

export function BottomBar({ view, focusPanel, stats, onHelpClick, onExportClick, aiOpsPerSecond }: BottomBarProps) {
  const getShortcuts = () => {
    if (view === 'dashboard') {
      return '[1-7] Views • [n] Add Task • [?] Help';
    }
    if (view === 'assets') {
      if (focusPanel === 'left') {
        return '[↑↓] Navigate • [n] Add • [s] Scan Selected • [Del] Delete';
      }
      return '[Tab] Switch Panel • [s] Scan • [e] Edit';
    }
    if (view === 'images') {
      return '[↑↓] Navigate • [Enter] Connect VNC • [Space] Start/Stop';
    }
    if (view === 'vulns') {
      if (focusPanel === 'left') {
        return '[↑↓] Navigate • [Enter] Details • [/] Search';
      }
      if (focusPanel === 'center') {
        return '[t] Test in Traffic • [e] Edit PoC • [f] FUZZ';
      }
      return '[Tab] Switch Panel';
    }
    if (view === 'traffic') {
      return '[↑↓] Navigate • [Space] Pause/Resume • [r] Replay • [/] TrafficQL';
    }
    if (view === 'devices') {
      return '[↑↓] Navigate • [Enter] Connect • [Space] Start/Stop Task';
    }
    return '[1-7] Switch View • [Tab] Switch Panel • [?] Help • [Esc] Close';
  };

  // 模拟实时系统指标（在实际应用中应从真实数据源获取）
  const cpuUsage = 34;
  const memUsage = 56;
  const networkRate = '2.4';
  const scanQueueSize = 3;

  return (
    <div className="border-t border-slate-200 bg-slate-50 px-4 py-1.5">
      <div className="flex items-center justify-between text-[10px]">
        {/* 左侧：快捷键提示 */}
        <div className="text-slate-500 font-mono flex-shrink-0">
          {getShortcuts()}
        </div>
        
        {/* 中间：实时系统监控 */}
        <div className="flex items-center gap-3 flex-1 justify-center font-mono">
          {/* CPU */}
          <div className="flex items-center gap-1">
            <span className="text-slate-400">CPU</span>
            <span className="text-slate-600">{cpuUsage}%</span>
          </div>
          
          <span className="text-slate-300">·</span>
          
          {/* Memory */}
          <div className="flex items-center gap-1">
            <span className="text-slate-400">MEM</span>
            <span className="text-slate-600">{memUsage}%</span>
          </div>
          
          <span className="text-slate-300">·</span>
          
          {/* Network */}
          <div className="flex items-center gap-1">
            <span className="text-slate-400">NET</span>
            <span className="text-slate-600">{networkRate}K/s</span>
          </div>
          
          {scanQueueSize > 0 && (
            <>
              <span className="text-slate-300">·</span>
              <div className="flex items-center gap-1">
                <span className="text-slate-400">Queue</span>
                <span className="text-slate-600">{scanQueueSize}</span>
              </div>
            </>
          )}
        </div>
        
        {/* 右侧：统计信息 + AI 性能 */}
        <div className="flex items-center gap-3 flex-shrink-0 font-mono">
          {aiOpsPerSecond !== undefined && (
            <>
              <div className="flex items-center gap-1">
                <span className="text-slate-400">AI</span>
                <span className="text-slate-600">{aiOpsPerSecond} ops/s</span>
              </div>
              <span className="text-slate-300">·</span>
            </>
          )}
          <div className="flex items-center gap-2 text-slate-500">
            <span>{stats.assets.total} assets</span>
            <span className="text-slate-300">·</span>
            <span>{stats.vulns.total} vulns</span>
            <span className="text-slate-300">·</span>
            <span>{stats.traffic.captured} packets</span>
          </div>
          {onHelpClick && (
            <>
              <span className="text-slate-300">·</span>
              <button
                onClick={onHelpClick}
                className="text-slate-500 hover:text-slate-700 transition-colors"
              >
                [?]
              </button>
            </>
          )}
        </div>
      </div>
    </div>
  );
}