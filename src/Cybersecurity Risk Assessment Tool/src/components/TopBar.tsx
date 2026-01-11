import { Shield, Target, Database, Box, Bug, Network, Cpu, Settings, Radio } from 'lucide-react';

interface TopBarProps {
  currentTime: Date;
  view: string;
  setView: (view: any) => void;
  stats: {
    assets: { total: number };
    vulns: { critical: number; total: number };
    traffic: { captured: number; anomalies: number };
  };
}

export function TopBar({ currentTime, view, setView, stats }: TopBarProps) {
  const views = [
    { id: 'dashboard', label: 'Dashboard', key: '1', icon: Target },
    { id: 'assets', label: 'Assets', key: '2', icon: Database },
    { id: 'images', label: 'Images', key: '3', icon: Box },
    { id: 'vulns', label: 'Vulns', key: '4', icon: Bug, count: stats.vulns.critical },
    { id: 'traffic', label: 'Traffic', key: '5', icon: Network, count: stats.traffic.anomalies },
    { id: 'workflows', label: 'Flows', key: '6', icon: Cpu },
    { id: 'devices', label: 'Devices', key: '7', icon: Radio },
    { id: 'settings', label: 'Settings', key: '8', icon: Settings },
  ];

  return (
    <div className="border-b border-slate-200 bg-[#F3F4F6] px-4 py-2.5">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <div className="flex items-center gap-2 pr-2 border-r border-slate-300">
            <div className="w-3 h-3 rounded-full bg-red-400 hover:bg-red-500 transition-colors border border-red-500/20" />
            <div className="w-3 h-3 rounded-full bg-yellow-400 hover:bg-yellow-500 transition-colors border border-yellow-500/20" />
            <div className="w-3 h-3 rounded-full bg-emerald-400 hover:bg-emerald-500 transition-colors border border-emerald-500/20" />
          </div>
          <div className="flex items-center gap-1.5">
            {views.map((v) => {
              const Icon = v.icon;
              return (
                <button
                  key={v.id}
                  onClick={() => setView(v.id)}
                  className={`px-3 py-1.5 rounded-md text-[10px] font-medium transition-all flex items-center gap-1.5 ${
                    view === v.id
                      ? 'bg-white text-purple-700 shadow-sm border border-slate-200'
                      : 'text-slate-500 hover:text-slate-800 hover:bg-slate-200/50'
                  }`}
                >
                  <Icon className="w-3.5 h-3.5" />
                  {v.label}
                  {v.count !== undefined && v.count > 0 && (
                    <span className="px-1.5 py-0.5 rounded-full text-[8px] bg-red-100 text-red-600 font-bold">
                      {v.count}
                    </span>
                  )}
                </button>
              );
            })}
          </div>
        </div>
        <div className="flex items-center gap-4 text-[10px]">
          <div className="flex items-center gap-1.5">
            <div className="w-1.5 h-1.5 bg-purple-500 rounded-full animate-pulse" />
            <span className="text-purple-600 font-medium">AI Active</span>
          </div>
          <span className="text-slate-500 font-mono">{currentTime.toLocaleTimeString()}</span>
        </div>
      </div>
    </div>
  );
}