interface LogEntry {
  timestamp: string;
  level: 'error' | 'warn' | 'info' | 'debug';
  message: string;
}

interface LogViewerProps {
  logs: LogEntry[];
  maxHeight?: string;
}

export function LogViewer({ logs, maxHeight = '300px' }: LogViewerProps) {
  const getLevelColor = (level: string) => {
    switch (level) {
      case 'error': return 'text-red-400';
      case 'warn': return 'text-yellow-400';
      case 'info': return 'text-cyan-400';
      case 'debug': return 'text-green-600';
      default: return 'text-green-500';
    }
  };

  const getLevelLabel = (level: string) => {
    return `[${level.toUpperCase().padEnd(5)}]`;
  };

  return (
    <div className="font-mono text-sm overflow-y-auto" style={{ maxHeight }}>
      {logs.map((log, idx) => (
        <div key={idx} className="flex gap-2 py-0.5 hover:bg-green-950/20">
          <span className="text-green-700">{log.timestamp}</span>
          <span className={getLevelColor(log.level)}>
            {getLevelLabel(log.level)}
          </span>
          <span className="text-green-400">{log.message}</span>
        </div>
      ))}
    </div>
  );
}
