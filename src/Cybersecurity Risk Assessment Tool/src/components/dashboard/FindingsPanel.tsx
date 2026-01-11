import { 
  Shield, CheckCircle, XCircle, AlertTriangle, Clock, 
  ChevronRight, ExternalLink, Filter, Search 
} from "lucide-react";
import { ScrollArea } from "../ui/scroll-area";
import { Badge } from "../ui/badge";
import { Button } from "../ui/button";

export interface Vulnerability {
  id: number;
  title: string;
  severity: 'critical' | 'high' | 'medium' | 'low' | 'info';
  asset: string;
  protocol: string;
  cve?: string;
  cvss: number;
  detectedAt: string;
  status: 'new' | 'validating' | 'confirmed' | 'false_positive';
  aiConfidence: number;
  description?: string;
}

interface FindingsPanelProps {
  vulnerabilities: Vulnerability[];
  onViewDetails?: (id: number) => void;
}

export function FindingsPanel({ vulnerabilities, onViewDetails }: FindingsPanelProps) {
  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case 'critical': return 'bg-red-50 text-red-600 border-red-200';
      case 'high': return 'bg-orange-50 text-orange-600 border-orange-200';
      case 'medium': return 'bg-yellow-50 text-yellow-600 border-yellow-200';
      case 'low': return 'bg-blue-50 text-blue-600 border-blue-200';
      default: return 'bg-slate-50 text-slate-600 border-slate-200';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'confirmed': return <CheckCircle className="w-3.5 h-3.5 text-emerald-500" />;
      case 'validating': return <Clock className="w-3.5 h-3.5 text-yellow-500" />;
      case 'false_positive': return <XCircle className="w-3.5 h-3.5 text-slate-400" />;
      default: return <AlertTriangle className="w-3.5 h-3.5 text-blue-500" />;
    }
  };

  return (
    <div className="flex-1 flex flex-col h-full bg-[#FAFAFA]">
      {/* Header / Stats */}
      <div className="px-4 py-3 border-b border-slate-200 bg-slate-50">
        <div className="flex items-center justify-between mb-3">
          <div className="flex items-center gap-4">
            <div className="flex items-center gap-2">
              <Shield className="w-4 h-4 text-purple-600" />
              <h3 className="text-sm font-medium text-slate-800">Security Findings</h3>
            </div>
            <div className="h-4 w-px bg-slate-300" />
            <div className="flex items-center gap-3 text-xs">
              <div className="flex items-center gap-1.5">
                <span className="text-slate-500">Total:</span>
                <span className="font-mono text-slate-700">{vulnerabilities.length}</span>
              </div>
              <div className="flex items-center gap-1.5">
                <span className="text-red-500">Critical:</span>
                <span className="font-mono text-red-600">
                  {vulnerabilities.filter(v => v.severity === 'critical').length}
                </span>
              </div>
              <div className="flex items-center gap-1.5">
                <span className="text-orange-500">High:</span>
                <span className="font-mono text-orange-600">
                  {vulnerabilities.filter(v => v.severity === 'high').length}
                </span>
              </div>
            </div>
          </div>
          
          <Button variant="outline" size="sm" className="h-8 border-slate-200 bg-white text-slate-600 hover:text-slate-900 hover:bg-slate-50 shadow-sm">
            <ExternalLink className="w-3.5 h-3.5 mr-2" />
            Export Report
          </Button>
        </div>
        
        <div className="relative">
          <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-slate-400" />
          <input 
            type="text"
            placeholder="Filter findings..."
            className="w-full h-9 pl-9 pr-3 bg-white border border-slate-200 rounded-lg text-xs text-slate-700 placeholder-slate-400 focus:outline-none focus:border-purple-500 transition-colors shadow-sm"
          />
        </div>
      </div>

      {/* Tabs / Filters */}
      <div className="px-4 py-2 border-b border-slate-200 flex items-center gap-2 bg-white">
        {['All', 'Critical', 'High', 'Medium', 'Low', 'Info'].map((tab) => (
          <button
            key={tab}
            className={`
              px-3 py-1 text-[10px] font-medium rounded-full border transition-colors
              ${tab === 'All' 
                ? 'bg-purple-50 text-purple-700 border-purple-200' 
                : 'bg-white text-slate-500 border-slate-200 hover:border-slate-300 hover:text-slate-700'}`}
          >
            {tab === 'All' ? 'All Findings' : tab}
          </button>
        ))}
      </div>

      {/* List Content */}
      <ScrollArea className="flex-1 bg-[#FAFAFA]">
        <div className="p-2 space-y-2">
          {vulnerabilities.map((vuln) => (
            <div 
              key={vuln.id}
              onClick={() => onViewDetails?.(vuln.id)}
              className="group flex flex-col gap-2 p-3 rounded-lg border border-slate-200 bg-white hover:border-purple-300 hover:shadow-md transition-all cursor-pointer shadow-sm"
            >
              <div className="flex items-start justify-between">
                <div className="flex items-start gap-3">
                  <div className={`mt-2 w-2 h-2 rounded-full shrink-0 ${
                    vuln.severity === 'critical' ? 'bg-red-500 shadow-[0_0_8px_rgba(239,68,68,0.4)]' :
                    vuln.severity === 'high' ? 'bg-orange-500 shadow-[0_0_8px_rgba(249,115,22,0.4)]' :
                    vuln.severity === 'medium' ? 'bg-yellow-500 shadow-[0_0_8px_rgba(234,179,8,0.4)]' :
                    'bg-blue-500 shadow-[0_0_8px_rgba(59,130,246,0.4)]'
                  }`} title={vuln.severity.toUpperCase()} />
                  <div>
                    <div className="flex items-center gap-2 mb-1">
                      <h4 className="text-sm font-medium text-slate-800 group-hover:text-purple-700 transition-colors">
                        {vuln.title}
                      </h4>
                      {vuln.cve && (
                        <span className="px-1.5 py-0.5 rounded bg-slate-50 text-slate-500 text-[10px] font-mono border border-slate-200">
                          {vuln.cve}
                        </span>
                      )}
                    </div>
                    <p className="text-xs text-slate-500 line-clamp-1">
                      {vuln.description || `Detected potential ${vuln.title} vulnerability on ${vuln.asset} via ${vuln.protocol} protocol.`}
                    </p>
                  </div>
                </div>
                
                <div className="flex flex-col items-end gap-2">
                  <div className="flex items-center gap-2">
                    <div className={`flex items-center gap-1.5 px-2 py-1 rounded text-[10px] font-medium border ${
                      vuln.status === 'confirmed' ? 'bg-emerald-50 text-emerald-600 border-emerald-200' :
                      vuln.status === 'validating' ? 'bg-yellow-50 text-yellow-600 border-yellow-200' :
                      'bg-slate-50 text-slate-500 border-slate-200'
                    }`}>
                      {getStatusIcon(vuln.status)}
                      <span className="uppercase">{vuln.status}</span>
                    </div>
                    <ChevronRight className="w-4 h-4 text-slate-400 group-hover:text-slate-600 transition-colors" />
                  </div>
                </div>
              </div>

              <div className="flex items-center justify-between pl-[84px] text-[10px] text-slate-500 font-mono mt-1 pt-2 border-t border-slate-100">
                <div className="flex items-center gap-4">
                  <div className="flex items-center gap-1.5">
                    <Clock className="w-3 h-3" />
                    {vuln.detectedAt} ago
                  </div>
                  <div className="flex items-center gap-1.5">
                    <ExternalLink className="w-3 h-3" />
                    {vuln.asset}
                  </div>
                </div>
                <div className="flex items-center gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                   <button className="hover:text-emerald-600 hover:underline">Verify</button>
                   <span className="text-slate-300">|</span>
                   <button className="hover:text-slate-600 hover:underline">Ignore</button>
                </div>
              </div>
            </div>
          ))}
        </div>
      </ScrollArea>
    </div>
  );
}