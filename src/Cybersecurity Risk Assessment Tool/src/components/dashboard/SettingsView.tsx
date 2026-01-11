import { useState } from 'react';
import { 
  Search, ChevronRight, Globe, Palette, Brain, Shield, Network, 
  Zap, Database, HardDrive, Code, FileText, ExternalLink
} from 'lucide-react';
import { Switch } from '../ui/switch';

interface SettingsConfig {
  // General
  language: string;
  autoUpdate: boolean;
  startupView: string;
  
  // Appearance
  theme: 'light' | 'dark' | 'auto';
  compactMode: boolean;
  animations: boolean;
  
  // AI
  aiProvider: string;
  aiModel: string;
  enableAutoScan: boolean;
  enablePoCGeneration: boolean;
  
  // Security
  sslVerify: boolean;
  sessionTimeout: number;
  
  // Network
  connectionTimeout: number;
  concurrentRequests: number;
  
  // Workflow
  maxParallelWorkflows: number;
  autoSaveWorkflows: boolean;
  
  // Scanner
  scanSpeed: string;
  enablePlugins: boolean;
  
  // Storage
  logLevel: string;
  retentionDays: number;
  
  // Advanced
  debugMode: boolean;
  experimentalFeatures: boolean;
}

interface SettingsViewProps {
  onEditJson?: () => void;
}

const DEFAULT_CONFIG: SettingsConfig = {
  language: 'zh-CN',
  autoUpdate: true,
  startupView: 'dashboard',
  theme: 'light',
  compactMode: false,
  animations: true,
  aiProvider: 'local',
  aiModel: 'llama-3.1-70b',
  enableAutoScan: false,
  enablePoCGeneration: true,
  sslVerify: true,
  sessionTimeout: 30,
  connectionTimeout: 30,
  concurrentRequests: 10,
  maxParallelWorkflows: 5,
  autoSaveWorkflows: true,
  scanSpeed: 'normal',
  enablePlugins: true,
  logLevel: 'info',
  retentionDays: 30,
  debugMode: false,
  experimentalFeatures: false,
};

type SettingsCategory = 
  | 'general' 
  | 'appearance' 
  | 'ai' 
  | 'security' 
  | 'network' 
  | 'workflow' 
  | 'scanner' 
  | 'storage' 
  | 'advanced';

export function SettingsView({ onEditJson }: SettingsViewProps) {
  const [selectedCategory, setSelectedCategory] = useState<SettingsCategory>('general');
  const [searchQuery, setSearchQuery] = useState('');
  const [config, setConfig] = useState<SettingsConfig>(DEFAULT_CONFIG);

  const categories = [
    { id: 'general' as const, label: 'General', icon: Globe },
    { id: 'appearance' as const, label: 'Appearance', icon: Palette },
    { id: 'ai' as const, label: 'AI', icon: Brain },
    { id: 'security' as const, label: 'Security', icon: Shield },
    { id: 'network' as const, label: 'Network', icon: Network },
    { id: 'workflow' as const, label: 'Workflow', icon: Zap },
    { id: 'scanner' as const, label: 'Scanner', icon: Database },
    { id: 'storage' as const, label: 'Storage', icon: HardDrive },
    { id: 'advanced' as const, label: 'Advanced', icon: Code },
  ];

  const updateConfig = <K extends keyof SettingsConfig>(key: K, value: SettingsConfig[K]) => {
    setConfig(prev => ({ ...prev, [key]: value }));
    // Auto-save to localStorage
    const newConfig = { ...config, [key]: value };
    localStorage.setItem('appSettings', JSON.stringify(newConfig));
  };

  const handleEditJson = () => {
    if (onEditJson) {
      onEditJson();
    } else {
      // Fallback: open in new window with JSON editor
      const jsonStr = JSON.stringify(config, null, 2);
      const blob = new Blob([jsonStr], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      window.open(url, '_blank');
    }
  };

  return (
    <div className="flex h-full bg-[#FAFAFA] rounded-lg overflow-hidden border border-slate-200">
      {/* Left Sidebar */}
      <div className="w-48 bg-white border-r border-slate-200 flex flex-col">
        {/* Search */}
        <div className="p-3 border-b border-slate-200">
          <div className="relative">
            <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3 h-3 text-slate-400" />
            <input
              type="text"
              placeholder="Search settings..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full pl-7 pr-2 py-1.5 bg-slate-50 border border-slate-200 rounded text-[10px] text-slate-700 placeholder:text-slate-400 focus:outline-none focus:ring-2 focus:ring-purple-500/20"
            />
          </div>
        </div>

        {/* Categories */}
        <div className="flex-1 overflow-auto py-2">
          {categories.map(cat => {
            const Icon = cat.icon;
            return (
              <button
                key={cat.id}
                onClick={() => setSelectedCategory(cat.id)}
                className={`w-full flex items-center gap-2 px-3 py-2 text-[11px] transition-all ${
                  selectedCategory === cat.id
                    ? 'bg-slate-100 text-slate-900'
                    : 'text-slate-600 hover:bg-slate-50'
                }`}
              >
                <ChevronRight className={`w-3 h-3 transition-transform ${
                  selectedCategory === cat.id ? 'rotate-90' : ''
                }`} />
                <Icon className="w-3.5 h-3.5" />
                <span>{cat.label}</span>
              </button>
            );
          })}
        </div>
      </div>

      {/* Right Content */}
      <div className="flex-1 overflow-auto">
        <div className="max-w-3xl mx-auto p-6 space-y-6">
          {/* Header with Edit JSON button */}
          <div className="flex items-center justify-between pb-4 border-b border-slate-200">
            <div>
              <h2 className="text-sm font-bold text-slate-800 capitalize">{selectedCategory}</h2>
              <p className="text-[10px] text-slate-500 mt-0.5">
                Configure {selectedCategory} settings
              </p>
            </div>
            <button
              onClick={handleEditJson}
              className="flex items-center gap-1.5 px-3 py-1.5 bg-slate-100 hover:bg-slate-200 border border-slate-200 text-slate-700 rounded text-[10px] transition-all font-medium"
            >
              <FileText className="w-3 h-3" />
              Edit in settings.json
            </button>
          </div>

          {/* General Settings */}
          {selectedCategory === 'general' && (
            <div className="space-y-1">
              <SettingRow
                label="Auto Update"
                description="Automatically check and install updates"
                control={
                  <Switch
                    checked={config.autoUpdate}
                    onCheckedChange={(checked) => updateConfig('autoUpdate', checked)}
                  />
                }
              />
              <SettingRow
                label="Language"
                description="UI display language"
                control={
                  <select
                    value={config.language}
                    onChange={(e) => updateConfig('language', e.target.value)}
                    className="px-2 py-1 bg-white border border-slate-200 rounded text-[10px] text-slate-700 focus:outline-none focus:ring-2 focus:ring-purple-500/20"
                  >
                    <option value="zh-CN">简体中文</option>
                    <option value="en-US">English</option>
                    <option value="ja-JP">日本語</option>
                  </select>
                }
              />
              <SettingRow
                label="Startup View"
                description="Default view when application starts"
                control={
                  <select
                    value={config.startupView}
                    onChange={(e) => updateConfig('startupView', e.target.value)}
                    className="px-2 py-1 bg-white border border-slate-200 rounded text-[10px] text-slate-700 focus:outline-none focus:ring-2 focus:ring-purple-500/20"
                  >
                    <option value="dashboard">Dashboard</option>
                    <option value="assets">Assets</option>
                    <option value="workflows">Workflows</option>
                  </select>
                }
              />
            </div>
          )}

          {/* Appearance Settings */}
          {selectedCategory === 'appearance' && (
            <div className="space-y-1">
              <SettingRow
                label="Theme"
                description="Color theme for the interface"
                control={
                  <select
                    value={config.theme}
                    onChange={(e) => updateConfig('theme', e.target.value as any)}
                    className="px-2 py-1 bg-white border border-slate-200 rounded text-[10px] text-slate-700 focus:outline-none focus:ring-2 focus:ring-purple-500/20"
                  >
                    <option value="light">Light</option>
                    <option value="dark">Dark</option>
                    <option value="auto">Auto</option>
                  </select>
                }
              />
              <SettingRow
                label="Compact Mode"
                description="Reduce spacing to show more content"
                control={
                  <Switch
                    checked={config.compactMode}
                    onCheckedChange={(checked) => updateConfig('compactMode', checked)}
                  />
                }
              />
              <SettingRow
                label="Animations"
                description="Enable interface transition animations"
                control={
                  <Switch
                    checked={config.animations}
                    onCheckedChange={(checked) => updateConfig('animations', checked)}
                  />
                }
              />
            </div>
          )}

          {/* AI Settings */}
          {selectedCategory === 'ai' && (
            <div className="space-y-1">
              <SettingRow
                label="AI Provider"
                description="Which AI service to use for analysis"
                control={
                  <select
                    value={config.aiProvider}
                    onChange={(e) => updateConfig('aiProvider', e.target.value)}
                    className="px-2 py-1 bg-white border border-slate-200 rounded text-[10px] text-slate-700 focus:outline-none focus:ring-2 focus:ring-purple-500/20"
                  >
                    <option value="local">Local</option>
                    <option value="openai">OpenAI</option>
                    <option value="anthropic">Anthropic</option>
                    <option value="custom">Custom</option>
                  </select>
                }
                editInJson
              />
              <SettingRow
                label="AI Model"
                description="Model name or identifier"
                control={
                  <input
                    type="text"
                    value={config.aiModel}
                    onChange={(e) => updateConfig('aiModel', e.target.value)}
                    className="w-40 px-2 py-1 bg-white border border-slate-200 rounded text-[10px] text-slate-700 font-mono focus:outline-none focus:ring-2 focus:ring-purple-500/20"
                  />
                }
                editInJson
              />
              <SettingRow
                label="Auto Vulnerability Scanning"
                description="Automatically scan for vulnerabilities during traffic capture"
                control={
                  <Switch
                    checked={config.enableAutoScan}
                    onCheckedChange={(checked) => updateConfig('enableAutoScan', checked)}
                  />
                }
              />
              <SettingRow
                label="PoC Generation"
                description="Generate proof-of-concept exploits for findings"
                control={
                  <Switch
                    checked={config.enablePoCGeneration}
                    onCheckedChange={(checked) => updateConfig('enablePoCGeneration', checked)}
                  />
                }
              />
            </div>
          )}

          {/* Security Settings */}
          {selectedCategory === 'security' && (
            <div className="space-y-1">
              <SettingRow
                label="SSL Certificate Verification"
                description="Verify HTTPS certificate validity"
                control={
                  <Switch
                    checked={config.sslVerify}
                    onCheckedChange={(checked) => updateConfig('sslVerify', checked)}
                  />
                }
              />
              <SettingRow
                label="Session Timeout"
                description="Automatically logout after inactivity (minutes)"
                control={
                  <input
                    type="number"
                    value={config.sessionTimeout}
                    onChange={(e) => updateConfig('sessionTimeout', parseInt(e.target.value))}
                    className="w-20 px-2 py-1 bg-white border border-slate-200 rounded text-[10px] text-slate-700 font-mono text-right focus:outline-none focus:ring-2 focus:ring-purple-500/20"
                  />
                }
              />
            </div>
          )}

          {/* Network Settings */}
          {selectedCategory === 'network' && (
            <div className="space-y-1">
              <SettingRow
                label="Connection Timeout"
                description="Maximum time to wait for connection (seconds)"
                control={
                  <input
                    type="number"
                    value={config.connectionTimeout}
                    onChange={(e) => updateConfig('connectionTimeout', parseInt(e.target.value))}
                    className="w-20 px-2 py-1 bg-white border border-slate-200 rounded text-[10px] text-slate-700 font-mono text-right focus:outline-none focus:ring-2 focus:ring-purple-500/20"
                  />
                }
              />
              <SettingRow
                label="Concurrent Requests"
                description="Maximum number of simultaneous requests"
                control={
                  <input
                    type="number"
                    value={config.concurrentRequests}
                    onChange={(e) => updateConfig('concurrentRequests', parseInt(e.target.value))}
                    className="w-20 px-2 py-1 bg-white border border-slate-200 rounded text-[10px] text-slate-700 font-mono text-right focus:outline-none focus:ring-2 focus:ring-purple-500/20"
                  />
                }
              />
            </div>
          )}

          {/* Workflow Settings */}
          {selectedCategory === 'workflow' && (
            <div className="space-y-1">
              <SettingRow
                label="Max Parallel Workflows"
                description="Maximum number of workflows to run simultaneously"
                control={
                  <input
                    type="number"
                    value={config.maxParallelWorkflows}
                    onChange={(e) => updateConfig('maxParallelWorkflows', parseInt(e.target.value))}
                    className="w-20 px-2 py-1 bg-white border border-slate-200 rounded text-[10px] text-slate-700 font-mono text-right focus:outline-none focus:ring-2 focus:ring-purple-500/20"
                  />
                }
              />
              <SettingRow
                label="Auto Save Workflows"
                description="Automatically save workflow changes"
                control={
                  <Switch
                    checked={config.autoSaveWorkflows}
                    onCheckedChange={(checked) => updateConfig('autoSaveWorkflows', checked)}
                  />
                }
              />
            </div>
          )}

          {/* Scanner Settings */}
          {selectedCategory === 'scanner' && (
            <div className="space-y-1">
              <SettingRow
                label="Scan Speed"
                description="Balance between speed and stealth"
                control={
                  <select
                    value={config.scanSpeed}
                    onChange={(e) => updateConfig('scanSpeed', e.target.value)}
                    className="px-2 py-1 bg-white border border-slate-200 rounded text-[10px] text-slate-700 focus:outline-none focus:ring-2 focus:ring-purple-500/20"
                  >
                    <option value="slow">Slow</option>
                    <option value="normal">Normal</option>
                    <option value="fast">Fast</option>
                    <option value="aggressive">Aggressive</option>
                  </select>
                }
              />
              <SettingRow
                label="Enable Plugins"
                description="Load third-party scanner plugins"
                control={
                  <Switch
                    checked={config.enablePlugins}
                    onCheckedChange={(checked) => updateConfig('enablePlugins', checked)}
                  />
                }
              />
            </div>
          )}

          {/* Storage Settings */}
          {selectedCategory === 'storage' && (
            <div className="space-y-1">
              <SettingRow
                label="Log Level"
                description="Verbosity of application logs"
                control={
                  <select
                    value={config.logLevel}
                    onChange={(e) => updateConfig('logLevel', e.target.value)}
                    className="px-2 py-1 bg-white border border-slate-200 rounded text-[10px] text-slate-700 focus:outline-none focus:ring-2 focus:ring-purple-500/20"
                  >
                    <option value="debug">Debug</option>
                    <option value="info">Info</option>
                    <option value="warn">Warning</option>
                    <option value="error">Error</option>
                  </select>
                }
              />
              <SettingRow
                label="Retention Days"
                description="How long to keep logs and scan data"
                control={
                  <input
                    type="number"
                    value={config.retentionDays}
                    onChange={(e) => updateConfig('retentionDays', parseInt(e.target.value))}
                    className="w-20 px-2 py-1 bg-white border border-slate-200 rounded text-[10px] text-slate-700 font-mono text-right focus:outline-none focus:ring-2 focus:ring-purple-500/20"
                  />
                }
              />
            </div>
          )}

          {/* Advanced Settings */}
          {selectedCategory === 'advanced' && (
            <div className="space-y-1">
              <SettingRow
                label="Debug Mode"
                description="Enable verbose logging and debug tools"
                control={
                  <Switch
                    checked={config.debugMode}
                    onCheckedChange={(checked) => updateConfig('debugMode', checked)}
                  />
                }
              />
              <SettingRow
                label="Experimental Features"
                description="Enable unstable or preview features"
                control={
                  <Switch
                    checked={config.experimentalFeatures}
                    onCheckedChange={(checked) => updateConfig('experimentalFeatures', checked)}
                  />
                }
              />
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

interface SettingRowProps {
  label: string;
  description: string;
  control: React.ReactNode;
  editInJson?: boolean;
}

function SettingRow({ label, description, control, editInJson }: SettingRowProps) {
  return (
    <div className="flex items-center justify-between py-3 px-4 hover:bg-slate-50 transition-colors border-b border-slate-100 last:border-0">
      <div className="flex-1 pr-4">
        <div className="text-xs text-slate-800 mb-0.5">{label}</div>
        <div className="text-[10px] text-slate-500">{description}</div>
      </div>
      <div className="flex items-center gap-2">
        {control}
        {editInJson && (
          <button className="text-[9px] text-slate-400 hover:text-slate-600 px-2 py-1 bg-slate-50 hover:bg-slate-100 border border-slate-200 rounded transition-all">
            Edit in settings.json
          </button>
        )}
      </div>
    </div>
  );
}
