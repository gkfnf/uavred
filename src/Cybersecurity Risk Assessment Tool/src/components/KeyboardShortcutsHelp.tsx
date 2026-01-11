import { X, Keyboard } from 'lucide-react';

interface KeyboardShortcut {
  key: string;
  description: string;
  category: 'navigation' | 'actions' | 'panels';
}

interface KeyboardShortcutsHelpProps {
  show: boolean;
  onClose: () => void;
}

export function KeyboardShortcutsHelp({ show, onClose }: KeyboardShortcutsHelpProps) {
  const shortcuts: KeyboardShortcut[] = [
    // Navigation
    { key: '1', description: 'Dashboard', category: 'navigation' },
    { key: '2', description: 'Assets', category: 'navigation' },
    { key: '3', description: 'Images (Containers)', category: 'navigation' },
    { key: '4', description: 'Vulnerabilities', category: 'navigation' },
    { key: '5', description: 'Traffic', category: 'navigation' },
    { key: '6', description: 'Workflows', category: 'navigation' },
    { key: '7', description: 'Devices (SDR)', category: 'navigation' },
    
    // Panel Focus
    { key: 'Tab', description: 'Cycle focus between panels', category: 'panels' },
    { key: 'Esc', description: 'Close dialogs / Reset focus', category: 'panels' },
    
    // Actions
    { key: 'n', description: 'New asset', category: 'actions' },
    { key: 's', description: 'Start scan', category: 'actions' },
    { key: 'Space', description: 'Pause/Resume (Traffic)', category: 'actions' },
    { key: '?', description: 'Show this help', category: 'actions' },
    { key: '/', description: 'Focus search', category: 'actions' },
    { key: 'r', description: 'Refresh data', category: 'actions' },
  ];

  const categories = {
    navigation: 'Navigation',
    panels: 'Panel Control',
    actions: 'Actions'
  };

  if (!show) return null;

  return (
    <div className="fixed inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-4">
      <div className="bg-gradient-to-br from-slate-900 to-slate-800 rounded-xl border-2 border-orange-500/50 shadow-2xl shadow-orange-500/20 max-w-2xl w-full max-h-[80vh] overflow-hidden">
        {/* Header */}
        <div className="border-b border-slate-700/50 px-6 py-4 bg-slate-900/50 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Keyboard className="w-5 h-5 text-orange-400" />
            <span className="text-slate-100 font-semibold">Keyboard Shortcuts</span>
          </div>
          <button
            onClick={onClose}
            className="p-2 hover:bg-slate-700 rounded-lg transition-all text-slate-400 hover:text-slate-200"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* Content */}
        <div className="p-6 overflow-auto max-h-[calc(80vh-73px)]">
          <div className="space-y-6">
            {Object.entries(categories).map(([key, title]) => {
              const categoryShortcuts = shortcuts.filter(s => s.category === key);
              return (
                <div key={key}>
                  <div className="text-xs text-orange-400 font-semibold mb-3 uppercase tracking-wider">
                    {title}
                  </div>
                  <div className="space-y-2">
                    {categoryShortcuts.map((shortcut, idx) => (
                      <div
                        key={idx}
                        className="flex items-center justify-between p-3 rounded-lg bg-slate-950/50 border border-slate-800/50 hover:border-slate-700/50 transition-all"
                      >
                        <span className="text-sm text-slate-300">{shortcut.description}</span>
                        <kbd className="px-3 py-1.5 bg-gradient-to-br from-slate-800 to-slate-900 border border-slate-700 rounded-lg text-xs text-orange-400 font-mono font-semibold shadow-lg">
                          {shortcut.key}
                        </kbd>
                      </div>
                    ))}
                  </div>
                </div>
              );
            })}
          </div>

          {/* Footer Tips */}
          <div className="mt-6 pt-6 border-t border-slate-700/50">
            <div className="bg-gradient-to-br from-purple-950/30 to-slate-950 rounded-lg border border-purple-800/30 p-4">
              <div className="flex items-start gap-3">
                <div className="w-2 h-2 bg-purple-400 rounded-full mt-1.5" />
                <div>
                  <div className="text-xs text-purple-400 font-medium mb-1">Pro Tip</div>
                  <div className="text-xs text-slate-400 leading-relaxed">
                    Use <kbd className="px-1.5 py-0.5 bg-slate-800 border border-slate-700 rounded text-[10px] text-orange-400 font-mono mx-1">Tab</kbd> 
                    to navigate between panels and maximize your workflow efficiency. The AI assistant will learn your patterns and provide contextual suggestions.
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
