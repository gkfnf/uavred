interface Tab {
  id: string;
  label: string;
  key: string;
}

interface TabBarProps {
  tabs: Tab[];
  activeTab: string;
  onTabChange: (tabId: string) => void;
}

export function TabBar({ tabs, activeTab, onTabChange }: TabBarProps) {
  return (
    <div className="flex border-b border-green-600 bg-black">
      {tabs.map((tab) => (
        <button
          key={tab.id}
          onClick={() => onTabChange(tab.id)}
          className={`px-4 py-2 border-r border-green-600 transition-colors ${
            activeTab === tab.id
              ? 'bg-cyan-950/50 text-cyan-300 border-b-2 border-b-cyan-400'
              : 'text-green-500 hover:bg-green-950/30'
          }`}
        >
          <span className="text-yellow-400">{tab.key}</span> {tab.label}
        </button>
      ))}
    </div>
  );
}
