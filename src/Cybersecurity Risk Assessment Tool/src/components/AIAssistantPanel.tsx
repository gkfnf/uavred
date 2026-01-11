import { Sparkles, TrendingUp, Shield, AlertTriangle, CheckCircle, ChevronRight } from 'lucide-react';

interface AIInsight {
  type: 'risk' | 'recommendation' | 'alert' | 'success';
  title: string;
  description: string;
  action?: string;
  priority: 'high' | 'medium' | 'low';
}

interface AIAssistantPanelProps {
  insights: AIInsight[];
  onActionClick?: (insight: AIInsight) => void;
}

export function AIAssistantPanel({ insights, onActionClick }: AIAssistantPanelProps) {
  const getIcon = (type: string) => {
    switch (type) {
      case 'risk': return <AlertTriangle className="w-3.5 h-3.5 text-red-400" />;
      case 'recommendation': return <TrendingUp className="w-3.5 h-3.5 text-cyan-400" />;
      case 'alert': return <Shield className="w-3.5 h-3.5 text-yellow-400" />;
      case 'success': return <CheckCircle className="w-3.5 h-3.5 text-emerald-400" />;
      default: return <Sparkles className="w-3.5 h-3.5 text-purple-400" />;
    }
  };

  const getBgColor = (type: string) => {
    switch (type) {
      case 'risk': return 'from-red-950/30 to-slate-950';
      case 'recommendation': return 'from-cyan-950/30 to-slate-950';
      case 'alert': return 'from-yellow-950/30 to-slate-950';
      case 'success': return 'from-emerald-950/30 to-slate-950';
      default: return 'from-purple-950/30 to-slate-950';
    }
  };

  const getBorderColor = (type: string) => {
    switch (type) {
      case 'risk': return 'border-red-800/30';
      case 'recommendation': return 'border-cyan-800/30';
      case 'alert': return 'border-yellow-800/30';
      case 'success': return 'border-emerald-800/30';
      default: return 'border-purple-800/30';
    }
  };

  return (
    <div className="bg-gradient-to-br from-slate-900 to-slate-800 rounded-xl border border-slate-700/50 overflow-hidden">
      <div className="border-b border-slate-700/50 px-4 py-3 bg-slate-900/50">
        <div className="flex items-center gap-2">
          <div className="w-2 h-2 bg-purple-400 rounded-full animate-pulse" />
          <span className="text-xs text-purple-400 font-medium">AI Security Assistant</span>
          <span className="ml-auto px-2 py-0.5 rounded-full text-[9px] bg-purple-900/30 text-purple-400 border border-purple-800/30">
            {insights.length} insights
          </span>
        </div>
      </div>
      <div className="p-3 space-y-2 overflow-auto max-h-96">
        {insights.length === 0 ? (
          <div className="text-center py-8 text-slate-600 text-xs">
            <Sparkles className="w-8 h-8 mx-auto mb-2 opacity-50" />
            <div>AI is analyzing your security posture...</div>
          </div>
        ) : (
          insights.map((insight, idx) => (
            <div
              key={idx}
              className={`bg-gradient-to-br ${getBgColor(insight.type)} rounded-lg border ${getBorderColor(insight.type)} p-3 transition-all hover:border-opacity-50`}
            >
              <div className="flex items-start gap-2 mb-2">
                {getIcon(insight.type)}
                <div className="flex-1">
                  <div className="text-xs text-slate-200 font-medium mb-1">{insight.title}</div>
                  <div className="text-[10px] text-slate-500 leading-relaxed">{insight.description}</div>
                </div>
                {insight.priority === 'high' && (
                  <div className="flex-shrink-0 w-1.5 h-1.5 bg-red-400 rounded-full animate-pulse" />
                )}
              </div>
              {insight.action && onActionClick && (
                <button
                  onClick={() => onActionClick(insight)}
                  className="w-full mt-2 flex items-center justify-between px-2 py-1.5 rounded bg-slate-950/50 hover:bg-slate-900 border border-slate-800/50 text-[10px] text-cyan-400 transition-all group"
                >
                  <span>{insight.action}</span>
                  <ChevronRight className="w-3 h-3 group-hover:translate-x-0.5 transition-transform" />
                </button>
              )}
            </div>
          ))
        )}
      </div>
    </div>
  );
}
