import { useState } from 'react';
import { ChevronLeft, ChevronRight } from 'lucide-react';

interface MenuItem {
  id: string;
  label: string;
  icon: string;
  section?: string;
}

interface SidebarProps {
  items: MenuItem[];
  activeItem: string;
  onItemChange: (itemId: string) => void;
  collapsed: boolean;
  onToggleCollapse: () => void;
}

export function Sidebar({ items, activeItem, onItemChange, collapsed, onToggleCollapse }: SidebarProps) {
  const [expandedSections, setExpandedSections] = useState<Record<string, boolean>>({
    proxy: true,
    testing: false,
    logging: true,
    workspace: true
  });

  const toggleSection = (section: string) => {
    setExpandedSections(prev => ({ ...prev, [section]: !prev[section] }));
  };

  const sections = {
    proxy: ['overview', 'assets', 'intercept', 'history'],
    testing: ['replay', 'automate', 'workflows'],
    logging: ['tasks', 'threats', 'logs', 'findings'],
    workspace: ['files', 'plugins', 'settings']
  };

  const sectionLabels = {
    proxy: 'Proxy',
    testing: 'Testing',
    logging: 'Logging',
    workspace: 'Workspace'
  };

  return (
    <div className={`bg-[#1a1d23] border-r border-[#2a2d35] flex flex-col transition-all duration-300 ${collapsed ? 'w-12' : 'w-56'}`}>
      {/* Logo */}
      <div className="h-12 border-b border-[#2a2d35] flex items-center px-3">
        {!collapsed && (
          <div className="flex items-center gap-2">
            <span className="text-red-500">UAV</span>
            <span className="text-slate-400">RISK</span>
          </div>
        )}
        {collapsed && <span className="text-red-500">UR</span>}
      </div>

      {/* Navigation */}
      <div className="flex-1 overflow-y-auto py-2">
        {Object.entries(sections).map(([section, itemIds]) => (
          <div key={section} className="mb-1">
            {!collapsed && (
              <button
                onClick={() => toggleSection(section)}
                className="w-full px-3 py-1.5 text-xs text-slate-500 hover:text-slate-300 flex items-center gap-1"
              >
                <ChevronRight className={`w-3 h-3 transition-transform ${expandedSections[section] ? 'rotate-90' : ''}`} />
                {sectionLabels[section as keyof typeof sectionLabels]}
              </button>
            )}
            {(collapsed || expandedSections[section]) && (
              <div className={collapsed ? 'space-y-0' : 'space-y-0.5 px-2'}>
                {itemIds.map(itemId => {
                  const item = items.find(i => i.id === itemId);
                  if (!item) return null;
                  
                  return (
                    <button
                      key={item.id}
                      onClick={() => onItemChange(item.id)}
                      className={`w-full px-3 py-2 text-sm flex items-center gap-2 rounded transition-colors ${
                        activeItem === item.id
                          ? 'bg-[#2a2d35] text-white'
                          : 'text-slate-400 hover:text-slate-200 hover:bg-[#22252b]'
                      }`}
                      title={collapsed ? item.label : undefined}
                    >
                      <span className="text-base">{item.icon}</span>
                      {!collapsed && <span>{item.label}</span>}
                    </button>
                  );
                })}
              </div>
            )}
          </div>
        ))}
      </div>

      {/* Collapse Button */}
      <button
        onClick={onToggleCollapse}
        className="h-10 border-t border-[#2a2d35] flex items-center justify-center text-slate-500 hover:text-slate-300 hover:bg-[#22252b]"
      >
        {collapsed ? <ChevronRight className="w-4 h-4" /> : <ChevronLeft className="w-4 h-4" />}
        {!collapsed && <span className="ml-2 text-xs">Collapse Sidebar</span>}
      </button>
    </div>
  );
}
