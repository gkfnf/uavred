interface StatCardProps {
  label: string;
  value: string | number;
  subtitle?: string;
  trend?: 'up' | 'down' | 'neutral';
  status?: 'critical' | 'high' | 'medium' | 'low' | 'info';
}

export function StatCard({ label, value, subtitle, trend, status }: StatCardProps) {
  const getStatusColor = () => {
    switch (status) {
      case 'critical': return 'border-red-500/30 bg-red-950/20';
      case 'high': return 'border-orange-500/30 bg-orange-950/20';
      case 'medium': return 'border-yellow-500/30 bg-yellow-950/20';
      case 'low': return 'border-green-500/30 bg-green-950/20';
      case 'info': return 'border-cyan-500/30 bg-cyan-950/20';
      default: return 'border-[#2a2d35] bg-[#22252b]';
    }
  };

  const getValueColor = () => {
    switch (status) {
      case 'critical': return 'text-red-400';
      case 'high': return 'text-orange-400';
      case 'medium': return 'text-yellow-400';
      case 'low': return 'text-green-400';
      case 'info': return 'text-cyan-400';
      default: return 'text-white';
    }
  };

  return (
    <div className={`border rounded-lg p-4 ${getStatusColor()}`}>
      <div className="text-xs text-slate-500 uppercase tracking-wide mb-1">{label}</div>
      <div className={`text-2xl mb-1 ${getValueColor()}`}>{value}</div>
      {subtitle && (
        <div className="text-xs text-slate-600">{subtitle}</div>
      )}
    </div>
  );
}
