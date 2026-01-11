import { 
  X, MoreHorizontal, Eye, Download, Send, Paperclip, 
  MessageSquare, RefreshCw, ChevronDown, Check, Play, Terminal
} from "lucide-react";
import { ScrollArea } from "../ui/scroll-area";
import { Button } from "../ui/button";
import { Avatar, AvatarFallback, AvatarImage } from "../ui/avatar";

interface AgentDetailsProps {
  taskId: string;
  taskTitle: string;
  status: "running" | "paused" | "completed" | "failed";
  onClose?: () => void;
}

export function AgentDetails({ taskId, taskTitle, status, onClose }: AgentDetailsProps) {
  // Mock data for the thought stream
  const steps = [
    {
      id: 1,
      type: "history",
      content: "Initial reconnaissance completed. Target appears to be running OpenResty + PHP 5.6.40. Several potentially vulnerable parameters identified.",
      timestamp: "14:30:05"
    },
    {
      id: 2,
      type: "thought",
      content: "Detected suspicious parameter `?ip=` in the URL. This pattern suggests a potential Command Injection vulnerability. The legacy PHP version (5.6.40) increases the likelihood of unpatched security flaws.",
      timestamp: "14:31:12"
    },
    {
      id: 3,
      type: "plan",
      content: "1. Verify connection to target.\n2. Fuzz the `ip` parameter with common injection payloads.\n3. Analyze response time and content for execution indicators.",
      timestamp: "14:31:15"
    },
    {
      id: 4,
      type: "tool",
      toolName: "curl",
      command: "curl -s -I -L --max-time 10 'http://target-drone-api:8080/?ip=127.0.0.1;id'",
      output: "uid=33(www-data) gid=33(www-data) groups=33(www-data)",
      status: "success",
      timestamp: "14:32:00"
    },
    {
      id: 5,
      type: "analysis",
      content: "Command execution confirmed. The server returned the output of the `id` command. This is a CRITICAL vulnerability allowing remote code execution.",
      timestamp: "14:32:05"
    }
  ];

  return (
    <div className="flex flex-col h-full bg-[#FAFAFA] w-full transition-all duration-300">
      {/* Header */}
      <div className="flex items-center justify-between px-3 border-b border-slate-200 bg-slate-50 h-[42px] shrink-0">
        <div className="flex items-center gap-3 overflow-hidden">
          <div className="flex items-center gap-2 shrink-0">
            <div className={`w-2 h-2 rounded-full ${
              status === 'running' ? 'bg-purple-500 animate-pulse shadow-[0_0_8px_rgba(168,85,247,0.5)]' : 
              status === 'failed' ? 'bg-red-500' : 'bg-slate-400'
            }`} />
            <span className="text-sm font-medium text-slate-800 tracking-tight truncate max-w-[200px]" title={taskTitle}>{taskTitle}</span>
          </div>
          <span className="text-[10px] text-slate-500 font-mono bg-white px-1.5 py-0.5 rounded border border-slate-200 shrink-0">ID: {taskId}</span>
        </div>
        <div className="flex items-center gap-1 shrink-0">
          <Button variant="ghost" size="icon" className="h-7 w-7 text-slate-500 hover:text-slate-800 hover:bg-slate-200/50 rounded-sm transition-colors">
            <Eye className="w-4 h-4" />
          </Button>
          <Button variant="ghost" size="icon" className="h-7 w-7 text-slate-500 hover:text-slate-800 hover:bg-slate-200/50 rounded-sm transition-colors">
            <Download className="w-4 h-4" />
          </Button>
          <Button variant="ghost" size="icon" className="h-7 w-7 text-slate-500 hover:text-slate-800 hover:bg-slate-200/50 rounded-sm transition-colors" onClick={onClose}>
            <X className="w-4 h-4" />
          </Button>
        </div>
      </div>

      {/* Content Area */}
      <ScrollArea className="flex-1 bg-[#FAFAFA]">
        <div className="p-4 space-y-6">
          
          {/* User Intent / Context */}
          <div className="space-y-2 pb-4 border-b border-slate-200">
            <div className="flex items-center gap-2 mb-2">
              <Avatar className="h-5 w-5 border border-slate-300">
                <AvatarFallback className="bg-slate-100 text-[9px] text-slate-600">U</AvatarFallback>
              </Avatar>
              <span className="text-xs font-medium text-slate-500 uppercase tracking-wider">Mission Objective</span>
            </div>
            <div className="ml-7 text-sm text-slate-700 leading-relaxed bg-white p-3 rounded-md border border-slate-200 font-mono text-[11px] shadow-sm">
              &gt; Analyze the target drone communication interface for injection vulnerabilities.
              <br/>&gt; Focus on legacy PHP endpoints.
            </div>
          </div>

          {/* Agent Thought Stream */}
          <div className="space-y-6">
             <div className="flex items-center gap-2">
              <Avatar className="h-5 w-5 border border-purple-200 ring-2 ring-purple-50">
                <AvatarFallback className="bg-purple-100 text-[9px] text-purple-600">AI</AvatarFallback>
              </Avatar>
              <span className="text-xs font-bold text-purple-600 uppercase tracking-wider">Penligent Agent</span>
              <span className="text-[10px] text-slate-400 ml-auto font-mono">LIVE TRACE</span>
            </div>

            <div className="ml-7 space-y-4 relative before:absolute before:left-[-11px] before:top-2 before:bottom-0 before:w-px before:bg-slate-200">
              
              {steps.map((step) => (
                <div key={step.id} className="relative animate-in fade-in slide-in-from-bottom-2 duration-500">
                  {/* Timeline Dot */}
                  <div className={`absolute -left-[14px] top-1.5 w-1.5 h-1.5 rounded-full border ${
                    step.type === 'tool' ? 'bg-cyan-100 border-cyan-500' : 
                    step.type === 'analysis' ? 'bg-red-100 border-red-500' :
                    'bg-white border-slate-400'
                  }`} />

                  {/* Header for Step */}
                  <div className="flex items-center gap-2 mb-1.5">
                    <span className={`text-[10px] font-bold uppercase tracking-wider px-1.5 py-0.5 rounded border ${
                      step.type === 'history' ? 'text-slate-500 border-slate-200 bg-slate-50' :
                      step.type === 'thought' ? 'text-purple-600 border-purple-200 bg-purple-50' :
                      step.type === 'plan' ? 'text-yellow-600 border-yellow-200 bg-yellow-50' :
                      step.type === 'tool' ? 'text-cyan-600 border-cyan-200 bg-cyan-50' :
                      'text-red-600 border-red-200 bg-red-50'
                    }`}>
                      {step.type}
                    </span>
                    <span className="text-[10px] text-slate-500 font-mono">{step.timestamp}</span>
                  </div>

                  {/* Content */}
                  {step.type === 'tool' ? (
                    <div className="bg-[#282A36] rounded border border-slate-700/50 overflow-hidden font-mono text-[10px] shadow-sm">
                      <div className="bg-[#44475A] px-3 py-1.5 border-b border-slate-600 flex items-center justify-between">
                        <span className="text-[#BD93F9] font-semibold flex items-center gap-1.5">
                          <Terminal className="w-3 h-3" />
                          {step.toolName}
                        </span>
                        <span className="text-[#50FA7B] text-[9px]">Success</span>
                      </div>
                      <div className="p-2 space-y-2">
                        <div className="text-[#F8F8F2] break-all pl-2 border-l-2 border-[#6272A4]">
                          <span className="text-[#8BE9FD] select-none">$ </span>
                          {step.command}
                        </div>
                        {step.output && (
                          <div className="text-[#F8F8F2]/80 pl-2 border-l-2 border-[#50FA7B]/50 pt-1">
                            <span className="text-[#6272A4] select-none block text-[9px] mb-0.5">OUTPUT &gt;&gt;</span>
                            {step.output}
                          </div>
                        )}
                      </div>
                    </div>
                  ) : (
                     <div className="text-xs text-slate-600 leading-relaxed pl-1">
                        {step.content}
                     </div>
                  )}
                </div>
              ))}

              {/* Running Indicator */}
              {status === 'running' && (
                <div className="relative pt-2">
                  <div className="absolute -left-[14px] top-3 w-1.5 h-1.5 rounded-full bg-purple-500 animate-pulse shadow-[0_0_8px_rgba(168,85,247,0.8)]" />
                  <div className="flex items-center gap-2 text-purple-600 text-xs animate-pulse font-mono">
                    <RefreshCw className="w-3 h-3 animate-spin" />
                    Thinking...
                  </div>
                </div>
              )}

            </div>
          </div>

        </div>
      </ScrollArea>


      {/* Footer / Input Area */}
      <div className="p-4 bg-slate-50 border-t border-slate-200">
        <div className="text-[10px] text-slate-500 mb-2 font-mono">
          Continue working on this task attempt... Type @ to insert tags or search files.
        </div>
        <div className="relative">
          <textarea 
            className="w-full bg-white border border-slate-200 rounded-lg p-3 text-sm text-slate-800 placeholder-slate-400 min-h-[80px] focus:outline-none focus:border-purple-400 focus:ring-1 focus:ring-purple-400/20 resize-none font-mono shadow-sm"
            placeholder=""
          />
          <div className="absolute bottom-2 right-2 flex items-center gap-2">
             <Button size="icon" variant="ghost" className="h-6 w-6 text-slate-400 hover:text-slate-600">
                <Paperclip className="w-3.5 h-3.5" />
             </Button>
             <Button size="icon" variant="ghost" className="h-6 w-6 text-slate-400 hover:text-slate-600">
                <MessageSquare className="w-3.5 h-3.5" />
             </Button>
             <Button size="sm" className="h-7 px-3 bg-white hover:bg-slate-50 text-slate-600 border border-slate-200 text-xs gap-1.5 shadow-sm">
                <Send className="w-3 h-3" />
                Send
             </Button>
          </div>
          <div className="absolute bottom-2 left-2">
             <Button variant="ghost" size="sm" className="h-6 text-[10px] text-slate-400 hover:text-slate-600 gap-1 px-1">
                <RefreshCw className="w-3 h-3" />
                DEFAULT
                <ChevronDown className="w-3 h-3" />
             </Button>
          </div>
        </div>
      </div>
    </div>
  );
}
