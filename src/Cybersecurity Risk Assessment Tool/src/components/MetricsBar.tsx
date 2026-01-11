interface Metric {
  label: string;
  value: string | number;
  status?: 'critical' | 'high' | 'medium' | 'low' | 'ok';
  bar?: number; // 0-100
}

interface MetricsBarProps {
  metrics: Metric[];
}

export function MetricsBar({ metrics }: MetricsBarProps) {
  const getStatusColor = (status?: string) => {
    switch (status) {
      case 'critical': return 'bg-red-500';
      case 'high': return 'bg-orange-500';
      case 'medium': return 'bg-yellow-500';
      case 'low': return 'bg-green-600';
      case 'ok': return 'bg-green-500';
      default: return 'bg-green-500';
    }
  };

  const getTextColor = (status?: string) => {
    switch (status) {
      case 'critical': return 'text-red-400';
      case 'high': return 'text-orange-400';
      case 'medium': return 'text-yellow-400';
      case 'low': return 'text-green-500';
      case 'ok': return 'text-green-400';
      default: return 'text-green-400';
    }
  };

  return (
    <div className="grid grid-cols-2 lg:grid-cols-4 gap-3">
      {metrics.map((metric, idx) => (
        <div key={idx} className="space-y-1">
          <div className="flex items-center justify-between text-sm">
            <span className="text-green-500">{metric.label}</span>
            <span className={getTextColor(metric.status)}>{metric.value}</span>
          </div>
          {metric.bar !== undefined && (
            <div className="h-1.5 bg-black border border-green-900">
              <div
                className={`h-full ${getStatusColor(metric.status)}`}
                style={{ width: `${metric.bar}%` }}
              />
            </div>
          )}
        </div>
      ))}
    </div>
  );
}
