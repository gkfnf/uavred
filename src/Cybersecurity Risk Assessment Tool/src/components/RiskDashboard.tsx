import { Badge } from './ui/badge';

export function RiskDashboard() {
  const metrics = [
    { label: 'Overall Security Posture', value: 62, status: 'medium' },
    { label: 'Pilot Compliance', value: 78, status: 'low' },
    { label: 'System Integrity', value: 45, status: 'high' },
    { label: 'Network Security', value: 34, status: 'critical' },
  ];

  const recentEvents = [
    { time: '14:23:45', event: 'Unauthorized access attempt detected', severity: 'high' },
    { time: '14:18:12', event: 'Pilot certification verified', severity: 'low' },
    { time: '14:15:03', event: 'Firmware version outdated', severity: 'medium' },
    { time: '14:10:44', event: 'GPS spoofing attempt blocked', severity: 'critical' },
  ];

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'critical': return 'text-red-400';
      case 'high': return 'text-orange-400';
      case 'medium': return 'text-yellow-400';
      case 'low': return 'text-green-400';
      default: return 'text-green-400';
    }
  };

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 gap-4 my-4">
      <div className="border border-green-500/30 p-4">
        <div className="text-green-400 mb-3">📊 SECURITY METRICS</div>
        <div className="space-y-3">
          {metrics.map((metric, idx) => (
            <div key={idx}>
              <div className="flex justify-between items-center mb-1">
                <span className="text-green-300 text-sm">{metric.label}</span>
                <span className={getStatusColor(metric.status)}>{metric.value}%</span>
              </div>
              <div className="h-1.5 bg-black border border-green-500/30">
                <div 
                  className={`h-full ${
                    metric.status === 'critical' ? 'bg-red-500' :
                    metric.status === 'high' ? 'bg-orange-500' :
                    metric.status === 'medium' ? 'bg-yellow-500' :
                    'bg-green-500'
                  }`}
                  style={{ width: `${metric.value}%` }}
                />
              </div>
            </div>
          ))}
        </div>
      </div>

      <div className="border border-green-500/30 p-4">
        <div className="text-green-400 mb-3">🔔 RECENT SECURITY EVENTS</div>
        <div className="space-y-2 text-sm">
          {recentEvents.map((event, idx) => (
            <div key={idx} className="flex items-start gap-2">
              <span className="text-green-500/70">[{event.time}]</span>
              <span className={getStatusColor(event.severity)}>{event.event}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
