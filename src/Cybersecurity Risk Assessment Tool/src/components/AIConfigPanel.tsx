import { useState } from 'react';
import { Brain, Server, Wifi, WifiOff, Settings, CheckCircle, AlertCircle } from 'lucide-react';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription } from './ui/dialog';
import { Label } from './ui/label';
import { Input } from './ui/input';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from './ui/select';
import { Switch } from './ui/switch';
import { Slider } from './ui/slider';

interface AIConfig {
  provider: 'local' | 'openai' | 'anthropic' | 'custom';
  model: string;
  apiKey?: string;
  endpoint?: string;
  temperature: number;
  maxTokens: number;
  enableAutoScan: boolean;
  enablePoCGeneration: boolean;
  enableTrafficAnalysis: boolean;
  aggressiveness: number;
}

interface AIConfigPanelProps {
  show: boolean;
  onClose: () => void;
  config: AIConfig;
  onSave: (config: AIConfig) => void;
}

export function AIConfigPanel({ show, onClose, config, onSave }: AIConfigPanelProps) {
  const [localConfig, setLocalConfig] = useState<AIConfig>(config);
  const [isConnected, setIsConnected] = useState(false);

  const handleSave = () => {
    onSave(localConfig);
    onClose();
  };

  const testConnection = async () => {
    // Simulate connection test
    setIsConnected(true);
    setTimeout(() => setIsConnected(false), 3000);
  };

  const modelOptions = {
    local: ['llama-3.1-70b', 'qwen-2.5-72b', 'deepseek-v3'],
    openai: ['gpt-4o', 'gpt-4o-mini', 'o1-preview'],
    anthropic: ['claude-3.5-sonnet', 'claude-3-opus'],
    custom: []
  };

  return (
    <Dialog open={show} onOpenChange={onClose}>
      <DialogContent className="bg-slate-900 border-slate-700 max-w-3xl max-h-[85vh] overflow-auto">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2 text-slate-100">
            <Brain className="w-5 h-5 text-purple-400" />
            AI Engine Configuration
          </DialogTitle>
          <DialogDescription className="text-slate-400 text-xs">
            Configure AI models for vulnerability analysis, PoC generation, and traffic inspection
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-6">
          {/* Provider Selection */}
          <div className="space-y-3">
            <Label className="text-slate-300">AI Provider</Label>
            <div className="grid grid-cols-4 gap-2">
              {(['local', 'openai', 'anthropic', 'custom'] as const).map((provider) => (
                <button
                  key={provider}
                  onClick={() => setLocalConfig({ ...localConfig, provider })}
                  className={`p-3 rounded-lg border transition-all ${
                    localConfig.provider === provider
                      ? 'border-purple-500/50 bg-purple-950/30 text-purple-400'
                      : 'border-slate-700 bg-slate-950/50 text-slate-400 hover:border-slate-600'
                  }`}
                >
                  <div className="flex flex-col items-center gap-1">
                    {provider === 'local' ? <Server className="w-4 h-4" /> : <Wifi className="w-4 h-4" />}
                    <span className="text-xs capitalize">{provider}</span>
                  </div>
                </button>
              ))}
            </div>
          </div>

          {/* Model Selection */}
          <div className="space-y-2">
            <Label className="text-slate-300">Model</Label>
            <Select
              value={localConfig.model}
              onValueChange={(value) => setLocalConfig({ ...localConfig, model: value })}
            >
              <SelectTrigger className="bg-slate-950 border-slate-700 text-slate-300">
                <SelectValue />
              </SelectTrigger>
              <SelectContent className="bg-slate-900 border-slate-700">
                {modelOptions[localConfig.provider].map((model) => (
                  <SelectItem key={model} value={model} className="text-slate-300">
                    {model}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          {/* API Configuration for non-local providers */}
          {localConfig.provider !== 'local' && (
            <div className="space-y-3 p-4 rounded-lg bg-slate-950/50 border border-slate-800">
              <div className="space-y-2">
                <Label className="text-slate-300 text-xs">API Key</Label>
                <div className="flex gap-2">
                  <Input
                    type="password"
                    value={localConfig.apiKey || ''}
                    onChange={(e) => setLocalConfig({ ...localConfig, apiKey: e.target.value })}
                    placeholder="sk-..."
                    className="flex-1 bg-slate-900 border-slate-700 text-slate-300"
                  />
                  <button
                    onClick={testConnection}
                    className="px-3 py-2 rounded-lg bg-cyan-900/30 hover:bg-cyan-900/50 border border-cyan-700/50 text-cyan-400 text-xs transition-all"
                  >
                    Test
                  </button>
                </div>
                {isConnected && (
                  <div className="flex items-center gap-2 text-xs text-emerald-400">
                    <CheckCircle className="w-3 h-3" />
                    <span>Connection successful</span>
                  </div>
                )}
              </div>

              {localConfig.provider === 'custom' && (
                <div className="space-y-2">
                  <Label className="text-slate-300 text-xs">API Endpoint</Label>
                  <Input
                    value={localConfig.endpoint || ''}
                    onChange={(e) => setLocalConfig({ ...localConfig, endpoint: e.target.value })}
                    placeholder="https://api.example.com/v1"
                    className="bg-slate-900 border-slate-700 text-slate-300"
                  />
                </div>
              )}
            </div>
          )}

          {/* Model Parameters */}
          <div className="space-y-4 p-4 rounded-lg bg-slate-950/50 border border-slate-800">
            <div className="text-xs text-slate-400 font-medium uppercase tracking-wider">Model Parameters</div>
            
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <Label className="text-slate-300 text-xs">Temperature</Label>
                <span className="text-xs text-purple-400 font-mono">{localConfig.temperature.toFixed(1)}</span>
              </div>
              <Slider
                value={[localConfig.temperature]}
                onValueChange={([value]) => setLocalConfig({ ...localConfig, temperature: value })}
                min={0}
                max={2}
                step={0.1}
                className="w-full"
              />
              <div className="text-[10px] text-slate-600">Higher values = more creative, lower = more deterministic</div>
            </div>

            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <Label className="text-slate-300 text-xs">Max Tokens</Label>
                <span className="text-xs text-purple-400 font-mono">{localConfig.maxTokens}</span>
              </div>
              <Slider
                value={[localConfig.maxTokens]}
                onValueChange={([value]) => setLocalConfig({ ...localConfig, maxTokens: value })}
                min={1024}
                max={8192}
                step={256}
                className="w-full"
              />
            </div>
          </div>

          {/* Feature Toggles */}
          <div className="space-y-3 p-4 rounded-lg bg-slate-950/50 border border-slate-800">
            <div className="text-xs text-slate-400 font-medium uppercase tracking-wider">AI Features</div>
            
            <div className="flex items-center justify-between p-2 rounded bg-slate-900/50">
              <div>
                <div className="text-sm text-slate-300">Auto Vulnerability Scanning</div>
                <div className="text-xs text-slate-600">Automatically scan for vulnerabilities during traffic capture</div>
              </div>
              <Switch
                checked={localConfig.enableAutoScan}
                onCheckedChange={(checked) => setLocalConfig({ ...localConfig, enableAutoScan: checked })}
              />
            </div>

            <div className="flex items-center justify-between p-2 rounded bg-slate-900/50">
              <div>
                <div className="text-sm text-slate-300">PoC Generation</div>
                <div className="text-xs text-slate-600">Generate proof-of-concept exploits for findings</div>
              </div>
              <Switch
                checked={localConfig.enablePoCGeneration}
                onCheckedChange={(checked) => setLocalConfig({ ...localConfig, enablePoCGeneration: checked })}
              />
            </div>

            <div className="flex items-center justify-between p-2 rounded bg-slate-900/50">
              <div>
                <div className="text-sm text-slate-300">Real-time Traffic Analysis</div>
                <div className="text-xs text-slate-600">Analyze traffic patterns in real-time</div>
              </div>
              <Switch
                checked={localConfig.enableTrafficAnalysis}
                onCheckedChange={(checked) => setLocalConfig({ ...localConfig, enableTrafficAnalysis: checked })}
              />
            </div>
          </div>

          {/* Scan Aggressiveness */}
          <div className="space-y-3 p-4 rounded-lg bg-slate-950/50 border border-slate-800">
            <div className="flex items-center justify-between">
              <Label className="text-slate-300 text-xs">Scan Aggressiveness</Label>
              <span className="text-xs text-purple-400 font-mono">{localConfig.aggressiveness}/10</span>
            </div>
            <Slider
              value={[localConfig.aggressiveness]}
              onValueChange={([value]) => setLocalConfig({ ...localConfig, aggressiveness: value })}
              min={1}
              max={10}
              step={1}
              className="w-full"
            />
            <div className="text-[10px] text-slate-600">
              Higher levels may trigger IDS/IPS. Use with caution in production environments.
            </div>
          </div>

          {/* Warning */}
          <div className="p-3 rounded-lg bg-yellow-950/20 border border-yellow-800/30">
            <div className="flex items-start gap-2">
              <AlertCircle className="w-4 h-4 text-yellow-400 flex-shrink-0 mt-0.5" />
              <div className="text-xs text-yellow-400/90">
                AI models process all data locally in your environment. No data is sent to external servers unless using cloud providers.
              </div>
            </div>
          </div>

          {/* Actions */}
          <div className="flex gap-3 pt-3 border-t border-slate-800">
            <button
              onClick={handleSave}
              className="flex-1 bg-purple-900/30 hover:bg-purple-900/50 border border-purple-700/50 rounded-lg px-4 py-2 text-sm text-purple-400 transition-all"
            >
              Save Configuration
            </button>
            <button
              onClick={onClose}
              className="flex-1 bg-slate-800 hover:bg-slate-700 border border-slate-700 rounded-lg px-4 py-2 text-sm text-slate-400 transition-all"
            >
              Cancel
            </button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
