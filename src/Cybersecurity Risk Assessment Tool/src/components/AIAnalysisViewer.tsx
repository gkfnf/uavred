import { Brain, GitBranch, Lightbulb, AlertTriangle, CheckCircle, Clock, TrendingUp, X } from 'lucide-react';
import { useState } from 'react';

interface AIAnalysisStep {
  id: string;
  timestamp: string;
  phase: string;
  finding: string;
  confidence: number;
  type: 'discovery' | 'hypothesis' | 'validation' | 'conclusion';
  relatedData?: {
    request?: string;
    response?: string;
    pattern?: string;
  };
  nextSteps?: string[];
}

interface AIAnalysisViewerProps {
  assetName: string;
  analysisSteps: AIAnalysisStep[];
  onClose: () => void;
}

export function AIAnalysisViewer({ assetName, analysisSteps, onClose }: AIAnalysisViewerProps) {
  const [selectedStep, setSelectedStep] = useState<AIAnalysisStep | null>(null);

  const getStepIcon = (type: string) => {
    switch (type) {
      case 'discovery': return <Lightbulb className="w-4 h-4 text-cyan-400" />;
      case 'hypothesis': return <GitBranch className="w-4 h-4 text-purple-400" />;
      case 'validation': return <TrendingUp className="w-4 h-4 text-yellow-400" />;
      case 'conclusion': return <CheckCircle className="w-4 h-4 text-emerald-400" />;
      default: return <Brain className="w-4 h-4 text-slate-400" />;
    }
  };

  const getStepColor = (type: string) => {
    switch (type) {
      case 'discovery': return 'border-cyan-800/30 bg-cyan-950/20';
      case 'hypothesis': return 'border-purple-800/30 bg-purple-950/20';
      case 'validation': return 'border-yellow-800/30 bg-yellow-950/20';
      case 'conclusion': return 'border-emerald-800/30 bg-emerald-950/20';
      default: return 'border-slate-800/30 bg-slate-950/20';
    }
  };

  return (
    <div className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
      <div className="bg-gradient-to-br from-slate-900 to-slate-800 rounded-xl border-2 border-purple-500/50 shadow-2xl shadow-purple-500/20 max-w-6xl w-full h-[85vh] overflow-hidden flex flex-col">
        {/* Header */}
        <div className="border-b border-slate-700/50 px-6 py-4 bg-slate-900/50 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-lg bg-purple-900/30 border border-purple-800/30 flex items-center justify-center">
              <Brain className="w-4 h-4 text-purple-400" />
            </div>
            <div>
              <div className="text-slate-100 font-semibold">AI Analysis Process</div>
              <div className="text-xs text-slate-500">Analyzing {assetName}</div>
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-2 hover:bg-slate-700 rounded-lg transition-all text-slate-400 hover:text-slate-200"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 flex overflow-hidden">
          {/* Timeline */}
          <div className="w-96 border-r border-slate-700/50 bg-slate-950/30 overflow-auto">
            <div className="p-4">
              <div className="text-xs text-slate-500 font-medium mb-3 uppercase tracking-wider">Analysis Timeline</div>
              <div className="space-y-2">
                {analysisSteps.map((step, idx) => (
                  <div
                    key={step.id}
                    onClick={() => setSelectedStep(step)}
                    className={`cursor-pointer rounded-lg border p-3 transition-all ${
                      selectedStep?.id === step.id
                        ? 'border-purple-500/50 bg-purple-950/20'
                        : `${getStepColor(step.type)} hover:border-purple-500/30`
                    }`}
                  >
                    <div className="flex items-start gap-2 mb-2">
                      {getStepIcon(step.type)}
                      <div className="flex-1 min-w-0">
                        <div className="text-xs text-slate-300 font-medium mb-1">{step.phase}</div>
                        <div className="text-[10px] text-slate-600">{step.timestamp}</div>
                      </div>
                      <div className="flex-shrink-0 px-2 py-0.5 rounded bg-slate-900/50 text-[9px] text-slate-500">
                        {step.confidence}%
                      </div>
                    </div>
                    <div className="text-[11px] text-slate-400 leading-relaxed line-clamp-2">
                      {step.finding}
                    </div>
                    {idx < analysisSteps.length - 1 && (
                      <div className="mt-2 ml-2 w-0.5 h-4 bg-gradient-to-b from-slate-700 to-transparent" />
                    )}
                  </div>
                ))}
              </div>
            </div>
          </div>

          {/* Details */}
          <div className="flex-1 overflow-auto p-6">
            {selectedStep ? (
              <div className="space-y-4">
                {/* Step Header */}
                <div className={`rounded-lg border p-4 ${getStepColor(selectedStep.type)}`}>
                  <div className="flex items-center gap-3 mb-3">
                    {getStepIcon(selectedStep.type)}
                    <div className="flex-1">
                      <div className="text-sm text-slate-200 font-medium">{selectedStep.phase}</div>
                      <div className="text-xs text-slate-500">{selectedStep.timestamp}</div>
                    </div>
                    <div className="px-3 py-1 rounded-full bg-slate-900/50 text-xs text-slate-400">
                      Confidence: <span className="text-purple-400 font-bold">{selectedStep.confidence}%</span>
                    </div>
                  </div>
                  <div className="text-sm text-slate-300 leading-relaxed">{selectedStep.finding}</div>
                </div>

                {/* Related Data */}
                {selectedStep.relatedData && (
                  <div className="space-y-3">
                    {selectedStep.relatedData.pattern && (
                      <div>
                        <div className="text-xs text-slate-500 font-medium mb-2">Pattern Detected</div>
                        <div className="bg-slate-950/50 rounded-lg border border-slate-800/50 p-3">
                          <code className="text-xs text-cyan-400 font-mono">{selectedStep.relatedData.pattern}</code>
                        </div>
                      </div>
                    )}

                    {selectedStep.relatedData.request && (
                      <div>
                        <div className="text-xs text-slate-500 font-medium mb-2">Request</div>
                        <div className="bg-slate-950/50 rounded-lg border border-slate-800/50 p-3">
                          <pre className="text-[10px] text-slate-400 font-mono overflow-x-auto">{selectedStep.relatedData.request}</pre>
                        </div>
                      </div>
                    )}

                    {selectedStep.relatedData.response && (
                      <div>
                        <div className="text-xs text-slate-500 font-medium mb-2">Response</div>
                        <div className="bg-slate-950/50 rounded-lg border border-slate-800/50 p-3">
                          <pre className="text-[10px] text-slate-400 font-mono overflow-x-auto">{selectedStep.relatedData.response}</pre>
                        </div>
                      </div>
                    )}
                  </div>
                )}

                {/* Next Steps */}
                {selectedStep.nextSteps && selectedStep.nextSteps.length > 0 && (
                  <div>
                    <div className="text-xs text-slate-500 font-medium mb-2">AI Next Steps</div>
                    <div className="space-y-2">
                      {selectedStep.nextSteps.map((step, idx) => (
                        <div key={idx} className="flex items-start gap-2 p-2 rounded bg-slate-950/30">
                          <div className="w-5 h-5 rounded bg-purple-900/30 border border-purple-800/30 flex items-center justify-center flex-shrink-0">
                            <span className="text-[10px] text-purple-400 font-bold">{idx + 1}</span>
                          </div>
                          <div className="text-xs text-slate-400">{step}</div>
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            ) : (
              <div className="flex items-center justify-center h-full">
                <div className="text-center text-slate-600">
                  <Brain className="w-12 h-12 mx-auto mb-3 opacity-50" />
                  <div className="text-sm">Select a step to view details</div>
                </div>
              </div>
            )}
          </div>
        </div>

        {/* Footer */}
        <div className="border-t border-slate-700/50 px-6 py-3 bg-slate-900/50">
          <div className="flex items-center justify-between text-xs">
            <div className="text-slate-500">
              <span className="text-purple-400 font-bold">{analysisSteps.length}</span> analysis steps completed
            </div>
            <div className="flex items-center gap-2 text-slate-600">
              <Clock className="w-3 h-3" />
              <span>Real-time analysis in progress</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
