import { Progress } from './ui/progress';
import { Badge } from './ui/badge';

interface RiskLevel {
  category: string;
  score: number;
  status: 'critical' | 'high' | 'medium' | 'low';
  findings: string[];
}

interface AssessmentResultsProps {
  type: 'pilot' | 'system';
  data: RiskLevel[];
}

export function AssessmentResults({ type, data }: AssessmentResultsProps) {
  const getStatusColor = (status: string) => {
    switch (status) {
      case 'critical': return 'text-red-400';
      case 'high': return 'text-orange-400';
      case 'medium': return 'text-yellow-400';
      case 'low': return 'text-green-400';
      default: return 'text-green-400';
    }
  };

  const overallRisk = Math.round(data.reduce((acc, item) => acc + item.score, 0) / data.length);
  
  return (
    <div className="space-y-4 my-4">
      <div className="border border-green-500/30 p-4">
        <div className="text-green-400 mb-2">
          {type === 'pilot' ? '👤 PILOT RISK ASSESSMENT' : '🛸 UAV SYSTEM ASSESSMENT'}
        </div>
        <div className="flex items-center gap-4 mb-4">
          <span className="text-green-300">Overall Risk Score:</span>
          <span className={`${getStatusColor(
            overallRisk > 75 ? 'critical' : overallRisk > 50 ? 'high' : overallRisk > 25 ? 'medium' : 'low'
          )}`}>
            {overallRisk}%
          </span>
          <Progress value={overallRisk} className="flex-1" />
        </div>
        
        <div className="space-y-3">
          {data.map((item, idx) => (
            <div key={idx} className="border-l-2 border-green-500/50 pl-3">
              <div className="flex items-center justify-between mb-1">
                <span className="text-green-300">{item.category}</span>
                <span className={getStatusColor(item.status)}>
                  [{item.status.toUpperCase()}] {item.score}%
                </span>
              </div>
              <Progress value={item.score} className="h-1 mb-2" />
              <div className="text-green-500/70 text-sm space-y-1">
                {item.findings.map((finding, fidx) => (
                  <div key={fidx} className="flex items-start gap-2">
                    <span className="text-yellow-400">⚠</span>
                    <span>{finding}</span>
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
