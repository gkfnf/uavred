import { Bug, Search, Send, Edit3, Save, Zap, ChevronRight } from 'lucide-react';

interface Vulnerability {
  id: string;
  severity: 'CRITICAL' | 'HIGH' | 'MEDIUM' | 'LOW';
  title: string;
  description: string;
  asset: string;
  assetId: number;
  cvss: number;
  aiScore: number;
  exploitability: number;
  impact: number;
  verified: boolean;
  pocAvailable: boolean;
  pocRequest: {
    method: string;
    path: string;
    headers: Record<string, string>;
    body: string;
    protocol: string;
  };
  mitreIds: string[];
  recommendation: string;
  detectedAt: string;
  cwe: string;
  affectedComponent: string;
}

interface VulnerabilitiesViewProps {
  vulnerabilities: Vulnerability[];
  selectedVulnIdx: number;
  setSelectedVulnIdx: (idx: number) => void;
  vulnGroupBy: 'severity' | 'asset' | 'mitre';
  setVulnGroupBy: (groupBy: 'severity' | 'asset' | 'mitre') => void;
  vulnSearchQuery: string;
  setVulnSearchQuery: (query: string) => void;
  editingPoc: boolean;
  setEditingPoc: (editing: boolean) => void;
  pocBody: string;
  setPocBody: (body: string) => void;
  onSendPoC: (vuln: Vulnerability) => void;
  onFuzzTest: () => void;
  focusPanel: 'left' | 'center' | 'right';
  setFocusPanel: (panel: 'left' | 'center' | 'right') => void;
  getGroupedVulns: () => Record<string, Vulnerability[]>;
}

export function VulnerabilitiesView({
  vulnerabilities,
  selectedVulnIdx,
  setSelectedVulnIdx,
  vulnGroupBy,
  setVulnGroupBy,
  vulnSearchQuery,
  setVulnSearchQuery,
  editingPoc,
  setEditingPoc,
  pocBody,
  setPocBody,
  onSendPoC,
  onFuzzTest,
  focusPanel,
  setFocusPanel,
  getGroupedVulns
}: VulnerabilitiesViewProps) {
  const getPanelStyle = (panel: 'left' | 'center' | 'right') => `
    border-2 transition-all
    ${focusPanel === panel 
      ? 'border-purple-400/50 shadow-md ring-1 ring-purple-100' 
      : 'border-slate-200'
    }
  `;

  const grouped = getGroupedVulns();
  const currentVuln = vulnerabilities[selectedVulnIdx];

  return (
    <div className="flex gap-3 h-full bg-[#FAFAFA] p-3">
      {/* Left: Vulnerability List */}
      <div 
        className={`w-80 bg-white rounded-xl overflow-hidden flex flex-col shadow-sm ${getPanelStyle('left')}`}
        onClick={() => setFocusPanel('left')}
      >
        <div className="border-b border-slate-200 px-3 py-2.5 bg-slate-50">
          <div className="flex items-center justify-between mb-2">
            <div className="flex items-center gap-2">
              <span className="text-xs text-slate-700 font-medium">Vulnerabilities</span>
              <span className="px-1.5 py-0.5 rounded-full text-[9px] bg-red-50 text-red-600 border border-red-200 font-bold">
                {vulnerabilities.length}
              </span>
            </div>
          </div>
          <div className="flex gap-1 mb-2">
            {(['severity', 'asset', 'mitre'] as const).map((groupType) => (
              <button
                key={groupType}
                onClick={() => setVulnGroupBy(groupType)}
                className={`px-2 py-1 rounded text-[9px] transition-all font-medium ${
                  vulnGroupBy === groupType
                    ? 'bg-purple-50 text-purple-700 border border-purple-200'
                    : 'text-slate-500 hover:text-slate-800 hover:bg-slate-100'
                }`}
              >
                {groupType === 'severity' ? 'Severity' : groupType === 'asset' ? 'Asset' : 'MITRE'}
              </button>
            ))}
          </div>
          <div className="relative">
            <Search className="absolute left-2 top-1.5 w-3 h-3 text-slate-400" />
            <input
              type="text"
              value={vulnSearchQuery}
              onChange={(e) => setVulnSearchQuery(e.target.value)}
              placeholder="Search vulnerabilities..."
              className="w-full bg-white border border-slate-200 rounded px-7 py-1 text-[10px] text-slate-700 placeholder:text-slate-400 focus:outline-none focus:border-purple-500 shadow-sm"
            />
          </div>
        </div>
        <div className="overflow-auto p-2 space-y-3 flex-1 bg-[#FAFAFA]">
          {Object.entries(grouped).map(([group, vulns]) => (
            vulns.length > 0 && (
              <div key={group}>
                <div className="text-[10px] text-slate-500 font-medium mb-1.5 px-1 uppercase tracking-wider">
                  {group} ({vulns.length})
                </div>
                <div className="space-y-1">
                  {vulns.map((vuln) => (
                    <div
                      key={vuln.id}
                      onClick={() => setSelectedVulnIdx(vulnerabilities.indexOf(vuln))}
                      className={`rounded-lg p-2.5 cursor-pointer transition-all border shadow-sm ${
                        selectedVulnIdx === vulnerabilities.indexOf(vuln)
                          ? 'bg-white border-purple-400 ring-1 ring-purple-400/30 shadow-md'
                          : 'bg-white border-slate-200 hover:border-purple-300 hover:shadow-md'
                      }`}
                    >
                      <div className="flex items-start gap-2">
                        <div className={`w-1 h-12 rounded-full ${
                          vuln.severity === 'CRITICAL' ? 'bg-red-500' :
                          vuln.severity === 'HIGH' ? 'bg-orange-500' :
                          vuln.severity === 'MEDIUM' ? 'bg-yellow-500' : 
                          'bg-blue-500'
                        }`} />
                        <div className="flex-1 min-w-0">
                          <div className="text-xs text-slate-800 mb-0.5 leading-tight font-medium">{vuln.title}</div>
                          <div className="text-[10px] text-slate-500 mb-1 font-mono">{vuln.id}</div>
                          <div className="flex items-center gap-1.5 text-[9px]">
                            <span className="text-purple-600 font-medium">AI {vuln.aiScore}%</span>
                            {vuln.pocAvailable && (
                              <>
                                <span className="text-slate-300">•</span>
                                <span className="text-blue-600 font-medium">PoC</span>
                              </>
                            )}
                            {vuln.verified && (
                              <>
                                <span className="text-slate-300">•</span>
                                <span className="text-emerald-600 font-medium">✓</span>
                              </>
                            )}
                          </div>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )
          ))}
        </div>
      </div>

      {/* Center: Vulnerability Details */}
      <div 
        className={`flex-1 bg-white rounded-xl overflow-hidden flex flex-col shadow-sm ${getPanelStyle('center')}`}
        onClick={() => setFocusPanel('center')}
      >
        <div className="border-b border-slate-200 px-4 py-2.5 bg-slate-50">
          <span className="text-xs text-slate-700 font-medium">Details & PoC</span>
        </div>
        <div className="p-4 space-y-4 overflow-auto flex-1 bg-white">
          {currentVuln && (
            <>
              <div>
                <div className="flex items-center gap-2 mb-2">
                  <span className={`px-2 py-1 rounded text-[10px] font-bold border ${
                    currentVuln.severity === 'CRITICAL' ? 'bg-red-50 text-red-600 border-red-200' :
                    currentVuln.severity === 'HIGH' ? 'bg-orange-50 text-orange-600 border-orange-200' :
                    'bg-yellow-50 text-yellow-600 border-yellow-200'
                  }`}>
                    {currentVuln.severity}
                  </span>
                  <span className="text-[10px] text-slate-500 font-mono">{currentVuln.id}</span>
                  <span className="text-[10px] text-slate-300">•</span>
                  <span className="text-[10px] text-slate-500 font-mono">{currentVuln.cwe}</span>
                  {currentVuln.verified && (
                    <span className="ml-auto px-2 py-0.5 rounded text-[9px] bg-emerald-50 text-emerald-600 border border-emerald-200 font-medium">
                      Verified
                    </span>
                  )}
                </div>
                <div className="text-base text-slate-900 font-bold mb-2">{currentVuln.title}</div>
                <div className="text-xs text-slate-600 leading-relaxed mb-2">{currentVuln.description}</div>
                <div className="flex items-center gap-2 text-[10px] text-slate-500 font-mono">
                  <span>Detected in: {currentVuln.asset}</span>
                  <span className="text-slate-300">•</span>
                  <span>{currentVuln.affectedComponent}</span>
                </div>
              </div>

              {/* AI Analysis */}
              <div className="bg-purple-50 rounded-lg border border-purple-100 p-4">
                <div className="flex items-center gap-2 mb-3">
                  <div className="w-2 h-2 bg-purple-500 rounded-full animate-pulse" />
                  <span className="text-xs text-purple-700 font-medium">AI Security Analysis</span>
                </div>
                
                <div className="space-y-3 text-xs">
                  <div>
                    <div className="flex items-center justify-between mb-1.5">
                      <span className="text-slate-600">Confidence Score</span>
                      <span className="text-purple-600 font-bold">{currentVuln.aiScore}%</span>
                    </div>
                    <div className="h-2 bg-white rounded-full overflow-hidden border border-purple-100">
                      <div className="h-full bg-purple-500" style={{ width: `${currentVuln.aiScore}%` }} />
                    </div>
                  </div>

                  <div>
                    <div className="flex items-center justify-between mb-1.5">
                      <span className="text-slate-600">Exploitability</span>
                      <span className="text-red-500 font-bold">{currentVuln.exploitability}%</span>
                    </div>
                    <div className="h-2 bg-white rounded-full overflow-hidden border border-purple-100">
                      <div className="h-full bg-red-500" style={{ width: `${currentVuln.exploitability}%` }} />
                    </div>
                  </div>

                  <div>
                    <div className="flex items-center justify-between mb-1.5">
                      <span className="text-slate-600">Potential Impact</span>
                      <span className="text-orange-500 font-bold">{currentVuln.impact}%</span>
                    </div>
                    <div className="h-2 bg-white rounded-full overflow-hidden border border-purple-100">
                      <div className="h-full bg-orange-500" style={{ width: `${currentVuln.impact}%` }} />
                    </div>
                  </div>
                </div>
              </div>

              {/* PoC Section */}
              {currentVuln.pocAvailable && (
                <div className="bg-[#282A36] rounded-lg border border-slate-200 overflow-hidden shadow-sm">
                  <div className="border-b border-slate-600 px-3 py-2 flex items-center justify-between bg-[#44475A]">
                    <span className="text-[10px] text-white font-medium flex items-center gap-2">
                       <Zap className="w-3 h-3 text-yellow-400" />
                       AI-Generated PoC
                    </span>
                    <div className="flex items-center gap-1">
                      <button
                        onClick={() => setEditingPoc(!editingPoc)}
                        className="px-2 py-1 rounded text-[9px] text-slate-300 hover:bg-slate-600 transition-all flex items-center gap-1"
                      >
                        {editingPoc ? <Save className="w-3 h-3" /> : <Edit3 className="w-3 h-3" />}
                        {editingPoc ? 'Save' : 'Edit'}
                      </button>
                      <button
                        onClick={() => onSendPoC(currentVuln)}
                        className="p-1.5 hover:bg-white/10 rounded transition-all"
                        title="Send to Traffic"
                      >
                        <Send className="w-3 h-3 text-cyan-400" />
                      </button>
                    </div>
                  </div>
                  <div className="p-3">
                    {editingPoc ? (
                      <textarea
                        value={pocBody || currentVuln.pocRequest.body}
                        onChange={(e) => setPocBody(e.target.value)}
                        className="w-full h-32 bg-[#282A36] text-slate-300 border border-slate-600 rounded p-2 font-mono text-[10px] focus:outline-none focus:border-purple-400 resize-none"
                      />
                    ) : (
                      <div className="font-mono text-[10px] text-slate-300 space-y-2">
                        <div className="text-purple-400 font-semibold">{currentVuln.pocRequest.method} <span className="text-green-400">{currentVuln.pocRequest.path}</span></div>
                        <div className="text-slate-400">
                          {Object.entries(currentVuln.pocRequest.headers).map(([k, v]) => (
                            <div key={k}><span className="text-cyan-300">{k}:</span> {v}</div>
                          ))}
                        </div>
                        {currentVuln.pocRequest.body && (
                          <>
                            <div className="border-t border-slate-600 my-2" />
                            <div className="text-slate-300 whitespace-pre-wrap break-all max-h-32 overflow-auto">
                              {currentVuln.pocRequest.body.length > 200 
                                ? currentVuln.pocRequest.body.substring(0, 200) + '...' 
                                : currentVuln.pocRequest.body}
                            </div>
                          </>
                        )}
                      </div>
                    )}
                  </div>
                </div>
              )}

              {/* MITRE ATT&CK */}
              <div>
                <div className="text-[10px] text-slate-500 mb-2 font-medium">MITRE ATT&CK Techniques</div>
                <div className="flex flex-wrap gap-1.5">
                  {currentVuln.mitreIds.map((id) => (
                    <span key={id} className="px-2 py-1 rounded bg-orange-50 text-[10px] text-orange-700 border border-orange-200 font-mono font-medium">
                      {id}
                    </span>
                  ))}
                </div>
              </div>

              {/* Recommendation */}
              <div className="bg-blue-50 rounded-lg border border-blue-100 p-3">
                <div className="flex items-center gap-2 mb-2">
                  <span className="text-[10px] text-blue-700 font-bold">💡 AI Recommendation</span>
                </div>
                <div className="text-xs text-blue-900 leading-relaxed">{currentVuln.recommendation}</div>
              </div>

              {/* Actions */}
              <div className="grid grid-cols-2 gap-2">
                <button
                  onClick={() => onSendPoC(currentVuln)}
                  className="flex items-center justify-center gap-2 bg-blue-50 hover:bg-blue-100 border border-blue-200 rounded-lg px-3 py-2.5 text-xs text-blue-700 transition-all font-medium"
                >
                  <Send className="w-3.5 h-3.5" />
                  Test in Traffic
                </button>
                <button 
                  onClick={onFuzzTest}
                  className="flex items-center justify-center gap-2 bg-purple-50 hover:bg-purple-100 border border-purple-200 rounded-lg px-3 py-2.5 text-xs text-purple-700 transition-all font-medium"
                >
                  <Zap className="w-3.5 h-3.5" />
                  FUZZ Test
                </button>
              </div>
            </>
          )}
        </div>
      </div>

      {/* Right: Related Information */}
      <div 
        className={`w-80 bg-white rounded-xl overflow-hidden flex flex-col shadow-sm ${getPanelStyle('right')}`}
        onClick={() => setFocusPanel('right')}
      >
        <div className="border-b border-slate-200 px-3 py-2.5 bg-slate-50">
          <span className="text-xs text-slate-700 font-medium">CVE Database</span>
        </div>
        <div className="p-3 space-y-3 flex-1 bg-[#FAFAFA]">
          {currentVuln && (
            <>
              <div className="bg-white rounded-lg p-3 border border-slate-200 shadow-sm">
                <div className="text-[10px] text-slate-500 mb-1 font-medium">CVSS Score</div>
                <div className="text-2xl text-red-500 font-bold mb-1">{currentVuln.cvss}</div>
                <div className="text-[10px] text-slate-400">v3.1 Base Score</div>
              </div>

              <div className="bg-white rounded-lg p-3 border border-slate-200 shadow-sm">
                <div className="text-[10px] text-slate-500 mb-2 font-medium">Detection Time</div>
                <div className="text-xs text-slate-700 font-mono">{currentVuln.detectedAt}</div>
              </div>

              <div className="bg-white rounded-lg p-3 border border-slate-200 shadow-sm">
                <div className="text-[10px] text-slate-500 mb-2 font-medium">Asset</div>
                <div className="text-xs text-slate-700 font-mono">{currentVuln.asset}</div>
              </div>

              <div className="pt-3 border-t border-slate-200">
                <div className="text-[10px] text-slate-500 mb-2 font-medium">Quick Actions</div>
                <button className="w-full bg-white hover:bg-slate-50 border border-slate-200 rounded-lg px-3 py-2 text-xs text-slate-600 transition-all text-left font-medium shadow-sm">
                  View Asset Details
                </button>
              </div>
            </>
          )}
        </div>
      </div>
    </div>
  );
}
