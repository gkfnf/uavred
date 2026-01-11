interface Task {
  id: string;
  name: string;
  target: string;
  status: 'running' | 'completed' | 'failed' | 'queued';
  progress: number;
  stage: string;
  startTime: string;
  eta?: string;
}

interface TaskProgressProps {
  tasks: Task[];
  compact?: boolean;
}

export function TaskProgress({ tasks, compact = false }: TaskProgressProps) {
  const getStatusColor = (status: string) => {
    switch (status) {
      case 'running': return 'text-cyan-400';
      case 'completed': return 'text-green-400';
      case 'failed': return 'text-red-400';
      case 'queued': return 'text-yellow-400';
      default: return 'text-green-500';
    }
  };

  const getStatusSymbol = (status: string) => {
    switch (status) {
      case 'running': return '⟳';
      case 'completed': return '✓';
      case 'failed': return '✗';
      case 'queued': return '◷';
      default: return '•';
    }
  };

  return (
    <div className="space-y-3">
      {tasks.map((task) => (
        <div key={task.id} className={compact ? 'space-y-1' : 'space-y-2'}>
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className={`${getStatusColor(task.status)} ${task.status === 'running' ? 'animate-spin' : ''}`}>
                {getStatusSymbol(task.status)}
              </span>
              <span className="text-green-400">{task.name}</span>
              <span className="text-green-700">→ {task.target}</span>
            </div>
            <span className="text-green-500 text-sm">{task.progress}%</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="flex-1 h-1.5 bg-black border border-green-900">
              <div 
                className={`h-full transition-all duration-300 ${
                  task.status === 'completed' ? 'bg-green-500' :
                  task.status === 'failed' ? 'bg-red-500' :
                  task.status === 'running' ? 'bg-cyan-500' :
                  'bg-yellow-500'
                }`}
                style={{ width: `${task.progress}%` }}
              />
            </div>
          </div>
          {!compact && (
            <div className="flex items-center justify-between text-sm">
              <span className="text-green-600">{task.stage}</span>
              <div className="flex gap-3 text-green-700">
                <span>Started: {task.startTime}</span>
                {task.eta && <span>ETA: {task.eta}</span>}
              </div>
            </div>
          )}
        </div>
      ))}
    </div>
  );
}
