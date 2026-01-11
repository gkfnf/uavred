import { Plus, MoreHorizontal, Circle } from "lucide-react";
import { ScrollArea } from "../ui/scroll-area";
import { Badge } from "../ui/badge";

export interface Task {
  id: string;
  title: string;
  status: "todo" | "in-progress" | "done";
  tag?: string;
  priority?: "low" | "medium" | "high";
}

interface TasksKanbanProps {
  tasks: Task[];
  onTaskClick?: (taskId: string) => void;
  selectedTaskId?: string;
}

export function TasksKanban({ tasks, onTaskClick, selectedTaskId }: TasksKanbanProps) {
  const columns = [
    { id: "todo", title: "To Do", color: "bg-slate-300" },
    { id: "in-progress", title: "In Progress", color: "bg-blue-500" },
    { id: "done", title: "Done", color: "bg-emerald-500" },
  ];

  const getPriorityColor = (priority?: string) => {
    switch (priority) {
      case "high": return "text-red-600 bg-red-50 border-red-200";
      case "medium": return "text-orange-600 bg-orange-50 border-orange-200";
      case "low": return "text-emerald-600 bg-emerald-50 border-emerald-200";
      default: return "text-slate-500 bg-slate-100 border-slate-200";
    }
  };

  return (
    <div className="flex-1 flex h-full overflow-hidden bg-white">
      {columns.map((col) => (
        <div 
          key={col.id} 
          className="flex-1 flex flex-col min-w-[200px] border-r border-slate-200 last:border-r-0"
        >
          {/* Column Header */}
          <div className="flex items-center justify-between px-3 border-b border-slate-200 bg-slate-50 h-[42px]">
            <div className="flex items-center gap-2">
              <div className={`w-2 h-2 rounded-full ${col.color}`} />
              <span className="text-xs font-medium text-slate-700">{col.title}</span>
              <span className="text-[10px] text-slate-400 font-mono">
                {tasks.filter(t => t.status === col.id).length}
              </span>
            </div>
            <button className="text-slate-400 hover:text-slate-600 transition-colors">
              <Plus className="w-3.5 h-3.5" />
            </button>
          </div>

          {/* Column Content */}
          <ScrollArea className="flex-1 p-2 bg-[#FAFAFA]">
            <div className="space-y-2">
              {tasks
                .filter((task) => task.status === col.id)
                .map((task) => (
                  <div
                    key={task.id}
                    onClick={() => onTaskClick?.(task.id)}
                    className={`
                      group relative p-3 rounded-md border cursor-pointer transition-all shadow-sm
                      ${selectedTaskId === task.id 
                        ? "bg-white border-purple-400 ring-1 ring-purple-400/50 shadow-md" 
                        : "bg-white border-slate-200 hover:border-purple-300 hover:shadow-md"}
                    `}
                  >
                    <div className="flex justify-between items-start gap-2 mb-2">
                      <h4 className="text-xs font-medium text-slate-700 leading-snug">
                        {task.title}
                      </h4>
                      <button className="opacity-0 group-hover:opacity-100 transition-opacity text-slate-400 hover:text-slate-600">
                        <MoreHorizontal className="w-3.5 h-3.5" />
                      </button>
                    </div>
                    
                    <div className="flex items-center gap-2">
                      {task.tag && (
                        <span className="px-1.5 py-0.5 rounded text-[9px] font-mono bg-slate-100 text-slate-500 border border-slate-200">
                          {task.tag}
                        </span>
                      )}
                      {task.priority && (
                        <span className={`px-1.5 py-0.5 rounded text-[9px] font-mono border ${getPriorityColor(task.priority)}`}>
                          {task.priority}
                        </span>
                      )}
                    </div>
                  </div>
                ))}
            </div>
          </ScrollArea>
        </div>
      ))}
    </div>
  );
}
