import { Play, Pause, Filter, Send, Copy, Download, RefreshCw, Zap, Edit3, Clock, Target, Hash, TrendingUp, AlertCircle, CheckCircle2 } from 'lucide-react';

interface TrafficPacket {
  id: number;
  time: string;
  src: string;
  dst: string;
  protocol: string;
  method: string;
  path: string;
  status: number;
  size: number;
  duration: string;
  anomaly: boolean;
  intercepted: boolean;
  request: string;
  response: string;
  vulnId?: string;
  assetId: number;
}

interface Asset {
  id: number;
  name: string;
  ip: string;
}

interface TrafficViewProps {
  traffic: TrafficPacket[];
  selectedTrafficIdx: number;
  setSelectedTrafficIdx: (idx: number) => void;
  trafficQL: string;
  setTrafficQL: (query: string) => void;
  trafficCapturing: boolean;
  setTrafficCapturing: (capturing: boolean) => void;
  interceptMode: 'off' | 'intercept';
  setInterceptMode: (mode: 'off' | 'intercept') => void;
  editingRequest: boolean;
  setEditingRequest: (editing: boolean) => void;
  editedRequest: string;
  setEditedRequest: (request: string) => void;
  assets: Asset[];
  onReplayTraffic: () => void;
  onForwardIntercepted: () => void;
  onDropIntercepted: () => void;
  onFuzzTest: () => void;
  getFilteredTraffic: () => TrafficPacket[];
  focusPanel: 'left' | 'center' | 'right';
  setFocusPanel: (panel: 'left' | 'center' | 'right') => void;
}

export function TrafficView({
  traffic,
  selectedTrafficIdx,
  setSelectedTrafficIdx,
  trafficQL,
  setTrafficQL,
  trafficCapturing,
  setTrafficCapturing,
  interceptMode,
  setInterceptMode,
  editingRequest,
  setEditingRequest,
  editedRequest,
  setEditedRequest,
  assets,
  onReplayTraffic,
  onForwardIntercepted,
  onDropIntercepted,
  onFuzzTest,
  getFilteredTraffic,
  focusPanel,
  setFocusPanel
}: TrafficViewProps) {
  const getPanelStyle = (panel: 'left' | 'center' | 'right') => `
    border-2 transition-all
    ${focusPanel === panel 
      ? 'border-purple-400/50 shadow-md ring-1 ring-purple-100' 
      : 'border-slate-200'
    }
  `;

  const filteredTraffic = getFilteredTraffic();
  const currentPacket = filteredTraffic[selectedTrafficIdx];

  return (
    <div className="flex flex-col gap-3 h-full bg-[#FAFAFA] p-3">
      {/* TrafficQL Bar */}
      <div className="bg-white rounded-xl border border-slate-200 p-2 shadow-sm">
        <div className="flex items-center gap-2">
          <span className="text-[10px] text-purple-600 font-mono font-bold">TrafficQL{' > '}</span>
          <input
            type="text"
            value={trafficQL}
            onChange={(e) => setTrafficQL(e.target.value)}
            placeholder="status:200 AND method:POST  |  protocol:MAVLink  |  anomaly:true  |  path~=&quot;/api&quot;"
            className="flex-1 bg-white border border-slate-200 rounded-lg px-3 py-1.5 text-[10px] text-slate-700 placeholder:text-slate-400 focus:outline-none focus:border-purple-500 shadow-sm"
          />
          <button
            onClick={() => setTrafficCapturing(!trafficCapturing)}
            className={`px-2.5 py-1.5 rounded-lg text-[10px] transition-all border flex items-center gap-1.5 font-medium ${
              trafficCapturing
                ? 'bg-emerald-50 border-emerald-200 text-emerald-600'
                : 'bg-slate-50 border-slate-200 text-slate-500'
            }`}
          >
            {trafficCapturing ? <Pause className="w-3 h-3" /> : <Play className="w-3 h-3" />}
            {trafficCapturing ? 'Capturing' : 'Paused'}
          </button>
          <button 
            onClick={() => setInterceptMode(interceptMode === 'off' ? 'intercept' : 'off')}
            className={`px-2.5 py-1.5 rounded-lg text-[10px] transition-all border font-medium ${
              interceptMode === 'intercept'
                ? 'bg-yellow-50 border-yellow-200 text-yellow-600'
                : 'bg-slate-50 border-slate-200 text-slate-500'
            }`}
          >
            Intercept
          </button>
          <button className="px-2.5 py-1.5 rounded-lg bg-white border border-slate-200 text-slate-500 hover:bg-slate-50 transition-all shadow-sm">
            <Filter className="w-3.5 h-3.5" />
          </button>
        </div>
      </div>

      {/* Middle Row: Request, Response, Actions (3 columns) */}
      <div className="h-[35%] shrink-0 flex gap-3 overflow-hidden">

        {/* Left: Request Details */}
        <div 
          className={`flex-1 bg-white rounded-xl overflow-hidden shadow-sm flex flex-col ${getPanelStyle('center')}`}
          onClick={() => setFocusPanel('center')}
        >
          {currentPacket ? (
            <>
              <div className="border-b border-slate-200 px-3 py-2.5 bg-slate-50 flex items-center justify-between">
                <span className="text-xs text-slate-700 font-medium">Request</span>
                <div className="flex items-center gap-1">
                  <button
                    onClick={() => setEditingRequest(!editingRequest)}
                    className="p-1.5 hover:bg-slate-200 rounded transition-all"
                    title="Edit"
                  >
                    <Edit3 className="w-3 h-3 text-slate-500" />
                  </button>
                  <button
                    onClick={() => navigator.clipboard.writeText(currentPacket.request)}
                    className="p-1.5 hover:bg-slate-200 rounded transition-all"
                    title="Copy"
                  >
                    <Copy className="w-3 h-3 text-slate-500" />
                  </button>
                </div>
              </div>
              <div className="p-0 overflow-auto h-full bg-[#282A36]">
                {editingRequest ? (
                  <textarea
                    value={editedRequest || currentPacket.request}
                    onChange={(e) => setEditedRequest(e.target.value)}
                    className="w-full h-full bg-[#282A36] text-slate-300 border-none p-3 font-mono text-[10px] focus:outline-none resize-none"
                  />
                ) : (
                  <pre className="font-mono text-[10px] text-slate-300 whitespace-pre-wrap p-3">
                    {currentPacket.request}
                  </pre>
                )}
              </div>
            </>
          ) : (
            <div className="flex items-center justify-center h-full text-slate-400 text-sm">
              Select a packet to view details
            </div>
          )}
        </div>

        {/* Center: Response Details */}
        <div 
          className={`flex-1 bg-white rounded-xl overflow-hidden shadow-sm flex flex-col ${getPanelStyle('center')}`}
          onClick={() => setFocusPanel('center')}
        >
          {currentPacket ? (
            <>
              <div className="border-b border-slate-200 px-3 py-2.5 bg-slate-50 flex items-center justify-between">
                <span className="text-xs text-slate-700 font-medium">Response</span>
                <button
                  onClick={() => navigator.clipboard.writeText(currentPacket.response)}
                  className="p-1.5 hover:bg-slate-200 rounded transition-all"
                  title="Copy"
                >
                  <Copy className="w-3 h-3 text-slate-500" />
                </button>
              </div>
              <div className="p-3 overflow-auto h-full bg-[#282A36]">
                <pre className="font-mono text-[10px] text-slate-300 whitespace-pre-wrap">
                  {currentPacket.response}
                </pre>
              </div>
            </>
          ) : (
            <div className="flex items-center justify-center h-full text-slate-400 text-sm">
              Select a packet to view details
            </div>
          )}
        </div>

        {/* Right: Actions Panel */}
        <div 
          className={`w-80 bg-white rounded-xl overflow-hidden shadow-sm ${getPanelStyle('right')}`}
          onClick={() => setFocusPanel('right')}
        >
          <div className="border-b border-slate-200 px-3 py-2.5 bg-slate-50">
            <span className="text-xs text-slate-700 font-medium">Actions</span>
          </div>
          <div className="p-3 space-y-3 overflow-auto h-full bg-white">
            {currentPacket && (
              <>
                {/* Packet Info */}
                <div className="bg-slate-50 rounded-lg p-2.5 border border-slate-200">
                  <div className="text-[10px] text-slate-500 mb-1.5 font-medium">Packet Info</div>
                  <div className="grid grid-cols-3 gap-2 text-[10px]">
                    <div>
                      <div className="text-slate-500">ID</div>
                      <div className="text-slate-700 font-mono">{currentPacket.id}</div>
                    </div>
                    <div>
                      <div className="text-slate-500">Size</div>
                      <div className="text-slate-700 font-mono">{currentPacket.size}B</div>
                    </div>
                    <div>
                      <div className="text-slate-500">Time</div>
                      <div className="text-slate-700 font-mono">{currentPacket.duration}</div>
                    </div>
                  </div>
                  {currentPacket.anomaly && (
                    <div className="flex items-center gap-1 text-red-600 mt-2 pt-2 border-t border-slate-200 font-medium">
                      <div className="w-1.5 h-1.5 bg-red-500 rounded-full animate-pulse" />
                      <span className="text-[9px]">Anomaly Detected</span>
                    </div>
                  )}
                </div>

                {/* Quick Actions */}
                <div className="grid grid-cols-2 gap-2">
                  <button
                    onClick={onReplayTraffic}
                    className="flex items-center justify-center gap-1.5 bg-cyan-50 hover:bg-cyan-100 border border-cyan-200 rounded-lg px-2 py-2 text-[10px] text-cyan-700 transition-all font-medium"
                  >
                    <RefreshCw className="w-3 h-3" />
                    Replay
                  </button>

                  <button
                    onClick={onFuzzTest}
                    className="flex items-center justify-center gap-1.5 bg-purple-50 hover:bg-purple-100 border border-purple-200 rounded-lg px-2 py-2 text-[10px] text-purple-700 transition-all font-medium"
                  >
                    <Zap className="w-3 h-3" />
                    FUZZ
                  </button>

                  <button
                    onClick={() => {}}
                    className="col-span-2 flex items-center justify-center gap-1.5 bg-slate-50 hover:bg-slate-100 border border-slate-200 rounded-lg px-2 py-2 text-[10px] text-slate-600 transition-all font-medium"
                  >
                    <Download className="w-3 h-3" />
                    Export as cURL
                  </button>
                </div>

                {/* Intercept Mode */}
                {interceptMode === 'intercept' && (
                  <div className="bg-yellow-50 rounded-lg border border-yellow-200 p-2.5">
                    <div className="flex items-center gap-2 mb-2">
                      <div className="w-1.5 h-1.5 bg-yellow-500 rounded-full animate-pulse" />
                      <span className="text-[10px] text-yellow-700 font-medium">Intercept Active</span>
                    </div>
                    <div className="grid grid-cols-2 gap-2">
                      <button
                        onClick={onForwardIntercepted}
                        className="bg-emerald-50 hover:bg-emerald-100 border border-emerald-200 rounded-lg px-2 py-1.5 text-[10px] text-emerald-700 transition-all font-medium"
                      >
                        Forward
                      </button>
                      <button
                        onClick={onDropIntercepted}
                        className="bg-red-50 hover:bg-red-100 border border-red-200 rounded-lg px-2 py-1.5 text-[10px] text-red-700 transition-all font-medium"
                      >
                        Drop
                      </button>
                    </div>
                  </div>
                )}

                {/* Traffic Statistics */}
                <div className="bg-slate-50 rounded-lg p-2.5 border border-slate-200">
                  <div className="text-[10px] text-slate-500 mb-2 font-medium">Statistics</div>
                  <div className="grid grid-cols-2 gap-2">
                    <div>
                      <div className="text-[9px] text-slate-500">Total</div>
                      <div className="text-xs text-cyan-600 font-mono font-bold">{filteredTraffic.length}</div>
                    </div>
                    <div>
                      <div className="text-[9px] text-slate-500">Anomalies</div>
                      <div className="text-xs text-red-600 font-mono font-bold">{filteredTraffic.filter(t => t.anomaly).length}</div>
                    </div>
                    <div>
                      <div className="text-[9px] text-slate-500">Success</div>
                      <div className="text-xs text-emerald-600 font-mono font-bold">
                        {filteredTraffic.length > 0 
                          ? Math.round((filteredTraffic.filter(t => t.status === 200).length / filteredTraffic.length) * 100)
                          : 0}%
                      </div>
                    </div>
                    <div>
                      <div className="text-[9px] text-slate-500">Avg Time</div>
                      <div className="text-xs text-purple-600 font-mono font-bold">
                        {filteredTraffic.length > 0
                          ? Math.round(filteredTraffic.reduce((sum, t) => sum + parseInt(t.duration), 0) / filteredTraffic.length)
                          : 0}ms
                      </div>
                    </div>
                  </div>
                </div>

                {/* Protocol Distribution */}
                <div className="bg-slate-50 rounded-lg p-2.5 border border-slate-200">
                  <div className="text-[10px] text-slate-500 mb-2 font-medium">Protocols</div>
                  <div className="space-y-1.5">
                    {Object.entries(
                      filteredTraffic.reduce((acc, t) => {
                        acc[t.protocol] = (acc[t.protocol] || 0) + 1;
                        return acc;
                      }, {} as Record<string, number>)
                    ).map(([protocol, count]) => (
                      <div key={protocol}>
                        <div className="flex items-center justify-between text-[9px] mb-0.5">
                          <span className={`font-mono font-medium ${
                            protocol === 'MAVLink' ? 'text-purple-600' :
                            protocol === 'HTTPS' || protocol === 'HTTP' ? 'text-cyan-600' :
                            protocol === 'DJI' ? 'text-yellow-600' :
                            protocol === 'RTSP' ? 'text-pink-600' :
                            'text-slate-600'
                          }`}>{protocol}</span>
                          <span className="text-slate-500">{count}</span>
                        </div>
                        <div className="h-1 bg-slate-200 rounded-full overflow-hidden">
                          <div 
                            className={`h-full ${
                              protocol === 'MAVLink' ? 'bg-purple-500' :
                              protocol === 'HTTPS' || protocol === 'HTTP' ? 'bg-cyan-500' :
                              protocol === 'DJI' ? 'bg-yellow-500' :
                              protocol === 'RTSP' ? 'bg-pink-500' :
                              'bg-slate-400'
                            }`}
                            style={{ width: `${(count / filteredTraffic.length) * 100}%` }}
                          />
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              </>
            )}

            {/* When no packet selected - show global stats */}
            {!currentPacket && filteredTraffic.length > 0 && (
              <div className="p-3 space-y-3">
                <div className="bg-slate-50 rounded-lg p-3 border border-slate-200 text-center">
                  <Target className="w-8 h-8 text-slate-400 mx-auto mb-2" />
                  <div className="text-[10px] text-slate-500">Select a packet to view details</div>
                  <div className="text-xs text-purple-600 font-mono mt-1 font-medium">{filteredTraffic.length} packets available</div>
                </div>

                {/* Global Statistics when nothing selected */}
                <div className="pt-3 border-t border-slate-200">
                  <div className="text-[10px] text-slate-500 mb-2 font-medium">Global Statistics</div>
                  <div className="grid grid-cols-2 gap-2">
                    <div className="bg-slate-50 rounded p-2 border border-slate-200">
                      <div className="text-[8px] text-slate-500">Total</div>
                      <div className="text-sm text-cyan-600 font-mono font-bold">{filteredTraffic.length}</div>
                    </div>
                    <div className="bg-slate-50 rounded p-2 border border-slate-200">
                      <div className="text-[8px] text-slate-500">Anomalies</div>
                      <div className="text-sm text-red-600 font-mono font-bold">{filteredTraffic.filter(t => t.anomaly).length}</div>
                    </div>
                    <div className="bg-slate-50 rounded p-2 border border-slate-200">
                      <div className="text-[8px] text-slate-500">Success</div>
                      <div className="text-sm text-emerald-600 font-mono font-bold">
                        {filteredTraffic.length > 0 
                          ? Math.round((filteredTraffic.filter(t => t.status === 200).length / filteredTraffic.length) * 100)
                          : 0}%
                      </div>
                    </div>
                    <div className="bg-slate-50 rounded p-2 border border-slate-200">
                      <div className="text-[8px] text-slate-500">Protocols</div>
                      <div className="text-sm text-purple-600 font-mono font-bold">
                        {Object.keys(filteredTraffic.reduce((acc, t) => {
                          acc[t.protocol] = true;
                          return acc;
                        }, {} as Record<string, boolean>)).length}
                      </div>
                    </div>
                  </div>
                </div>

                {/* Protocol Distribution */}
                <div className="pt-3 border-t border-slate-200">
                  <div className="text-[10px] text-slate-500 mb-2 font-medium">Protocols</div>
                  <div className="space-y-1.5">
                    {Object.entries(
                      filteredTraffic.reduce((acc, t) => {
                        acc[t.protocol] = (acc[t.protocol] || 0) + 1;
                        return acc;
                      }, {} as Record<string, number>)
                    ).sort((a, b) => b[1] - a[1]).map(([protocol, count]) => (
                      <div key={protocol} className="flex items-center gap-2">
                        <div className="flex-1">
                          <div className="flex items-center justify-between text-[10px] mb-0.5">
                            <span className={`font-mono font-medium ${
                              protocol === 'MAVLink' ? 'text-purple-600' :
                              protocol === 'HTTPS' || protocol === 'HTTP' ? 'text-cyan-600' :
                              protocol === 'DJI' ? 'text-yellow-600' :
                              protocol === 'RTSP' ? 'text-pink-600' :
                              'text-slate-600'
                            }`}>{protocol}</span>
                            <span className="text-slate-500">{count} ({Math.round((count / filteredTraffic.length) * 100)}%)</span>
                          </div>
                          <div className="h-1.5 bg-slate-200 rounded-full overflow-hidden">
                            <div 
                              className={`h-full ${
                                protocol === 'MAVLink' ? 'bg-purple-500' :
                                protocol === 'HTTPS' || protocol === 'HTTP' ? 'bg-cyan-500' :
                                protocol === 'DJI' ? 'bg-yellow-500' :
                                protocol === 'RTSP' ? 'bg-pink-500' :
                                'bg-slate-400'
                              }`}
                              style={{ width: `${(count / filteredTraffic.length) * 100}%` }}
                            />
                          </div>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>

                {/* TrafficQL Examples */}
                <div className="pt-3 border-t border-slate-200">
                  <div className="text-[10px] text-slate-500 mb-2 font-medium">TrafficQL Quick Filters</div>
                  <div className="space-y-1">
                    {[
                      { label: 'MAVLink Only', query: 'protocol:MAVLink' },
                      { label: 'Waypoint Commands', query: 'method:MAVLINK AND path~="MAV_CMD_NAV_WAYPOINT"' },
                      { label: 'Errors Only', query: 'status:500 OR status:404' },
                      { label: 'Show Anomalies', query: 'anomaly:true' },
                      { label: 'HTTP/HTTPS', query: 'protocol:HTTP OR protocol:HTTPS' },
                      { label: 'DJI Protocol', query: 'protocol:DJI' }
                    ].map((example, idx) => (
                      <button
                        key={idx}
                        onClick={() => setTrafficQL(example.query)}
                        className="w-full text-left px-2 py-1.5 rounded text-[9px] bg-slate-50 border border-slate-200 hover:border-purple-300 hover:bg-purple-50 hover:text-purple-700 transition-all shadow-sm"
                      >
                        <div className="text-slate-600 font-medium mb-0.5">{example.label}</div>
                        <div className="text-slate-400 font-mono text-[8px] truncate">{example.query}</div>
                      </button>
                    ))}
                  </div>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Bottom Row: Traffic Table (Large horizontal bar) */}
      <div 
        className={`flex-1 min-h-0 bg-white rounded-xl overflow-hidden shadow-sm ${getPanelStyle('left')}`}
        onClick={() => setFocusPanel('left')}
      >
        <div className="border-b border-slate-200 px-3 py-2 bg-slate-50 flex items-center justify-between">
          <span className="text-xs text-slate-700 font-medium">Captured Traffic ({filteredTraffic.length})</span>
          <div className="flex items-center gap-2 text-[10px]">
            <span className="text-purple-600 font-medium">{filteredTraffic.filter(t => t.anomaly).length} anomalies</span>
            {trafficCapturing && (
              <div className="flex items-center gap-1 text-emerald-600 font-medium">
                <div className="w-1.5 h-1.5 bg-emerald-500 rounded-full animate-pulse" />
                <span>Live</span>
              </div>
            )}
          </div>
        </div>
        <div className="overflow-auto bg-white" style={{ height: 'calc(100% - 41px)' }}>
          <table className="w-full text-[10px]">
            <thead className="sticky top-0 bg-slate-50 border-b border-slate-200">
              <tr>
                <th className="text-left px-3 py-1.5 text-slate-500 font-medium">#</th>
                <th className="text-left px-3 py-1.5 text-slate-500 font-medium">Time</th>
                <th className="text-left px-3 py-1.5 text-slate-500 font-medium">Asset</th>
                <th className="text-left px-3 py-1.5 text-slate-500 font-medium">Proto</th>
                <th className="text-left px-3 py-1.5 text-slate-500 font-medium">Method</th>
                <th className="text-left px-3 py-1.5 text-slate-500 font-medium">Path</th>
                <th className="text-left px-3 py-1.5 text-slate-500 font-medium">Status</th>
                <th className="text-left px-3 py-1.5 text-slate-500 font-medium">Size</th>
                <th className="text-left px-3 py-1.5 text-slate-500 font-medium">Duration</th>
              </tr>
            </thead>
            <tbody>
              {filteredTraffic.map((t, idx) => (
                <tr
                  key={t.id}
                  onClick={() => setSelectedTrafficIdx(idx)}
                  className={`border-b border-slate-100 cursor-pointer transition-all ${
                    selectedTrafficIdx === idx ? 'bg-purple-50' :
                    t.anomaly ? 'bg-red-50 hover:bg-red-100' :
                    'hover:bg-slate-50'
                  }`}
                >
                  <td className="px-3 py-1 text-slate-600">{t.id}</td>
                  <td className="px-3 py-1 text-slate-600 font-mono">{t.time.split(' ')[1] || t.time}</td>
                  <td className="px-3 py-1 text-slate-500">
                    {assets.find(a => a.id === t.assetId)?.name.substring(0, 12) || 'Unknown'}
                  </td>
                  <td className="px-3 py-1">
                    <span className={`px-1.5 py-0.5 rounded text-[9px] font-medium ${
                      t.protocol === 'HTTP' || t.protocol === 'HTTPS' ? 'bg-cyan-50 text-cyan-700' :
                      t.protocol === 'MAVLink' ? 'bg-purple-50 text-purple-700' :
                      t.protocol === 'DJI' ? 'bg-yellow-50 text-yellow-700' :
                      t.protocol === 'RTSP' ? 'bg-pink-50 text-pink-700' :
                      'bg-slate-100 text-slate-600'
                    }`}>
                      {t.protocol}
                    </span>
                  </td>
                  <td className="px-3 py-1 text-slate-700 font-medium">{t.method}</td>
                  <td className="px-3 py-1 text-slate-500 truncate max-w-[300px]">{t.path}</td>
                  <td className={`px-3 py-1 font-medium ${
                    t.status === 200 ? 'text-emerald-600' :
                    t.status === 401 || t.status === 403 ? 'text-yellow-600' : 
                    t.status >= 500 ? 'text-red-600' : 'text-slate-600'
                  }`}>
                    {t.status || '-'}
                  </td>
                  <td className="px-3 py-1 text-slate-500">{t.size}B</td>
                  <td className="px-3 py-1 text-slate-600 font-mono">{t.duration}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}