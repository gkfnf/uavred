interface MiniChartProps {
  data: number[];
  color?: string;
  height?: number;
  showGradient?: boolean;
}

export function MiniChart({ data, color = '#fb923c', height = 40, showGradient = true }: MiniChartProps) {
  if (data.length === 0) return null;

  const max = Math.max(...data);
  const min = Math.min(...data);
  const range = max - min || 1;

  const points = data.map((value, index) => {
    const x = (index / (data.length - 1)) * 100;
    const y = height - ((value - min) / range) * height;
    return `${x},${y}`;
  }).join(' ');

  const areaPoints = `0,${height} ${points} 100,${height}`;

  return (
    <svg width="100%" height={height} className="overflow-visible">
      <defs>
        {showGradient && (
          <linearGradient id={`gradient-${color}`} x1="0%" y1="0%" x2="0%" y2="100%">
            <stop offset="0%" stopColor={color} stopOpacity="0.3" />
            <stop offset="100%" stopColor={color} stopOpacity="0.05" />
          </linearGradient>
        )}
      </defs>
      
      {/* Area fill */}
      {showGradient && (
        <polygon
          points={areaPoints}
          fill={`url(#gradient-${color})`}
          className="transition-all duration-300"
        />
      )}
      
      {/* Line */}
      <polyline
        points={points}
        fill="none"
        stroke={color}
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
        className="transition-all duration-300"
        style={{ filter: `drop-shadow(0 0 4px ${color}40)` }}
      />
      
      {/* Data points */}
      {data.map((value, index) => {
        const x = (index / (data.length - 1)) * 100;
        const y = height - ((value - min) / range) * height;
        return (
          <circle
            key={index}
            cx={`${x}%`}
            cy={y}
            r="2"
            fill={color}
            className="opacity-0 hover:opacity-100 transition-opacity"
          />
        );
      })}
    </svg>
  );
}

interface SparklineProps {
  value: number;
  change: number;
  data: number[];
  label: string;
  color?: string;
}

export function Sparkline({ value, change, data, label, color = '#fb923c' }: SparklineProps) {
  const isPositive = change >= 0;
  
  return (
    <div className="bg-slate-950/50 rounded-lg p-3 border border-slate-800/50">
      <div className="flex items-center justify-between mb-2">
        <span className="text-[10px] text-slate-500">{label}</span>
        <span className={`text-[10px] font-bold ${isPositive ? 'text-emerald-400' : 'text-red-400'}`}>
          {isPositive ? '+' : ''}{change}%
        </span>
      </div>
      <div className="text-2xl font-bold mb-2" style={{ color }}>
        {value}
      </div>
      <MiniChart data={data} color={color} height={30} />
    </div>
  );
}
