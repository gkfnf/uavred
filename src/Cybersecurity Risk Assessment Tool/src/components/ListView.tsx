interface ListItem {
  id: string;
  label: string;
  status?: 'critical' | 'high' | 'medium' | 'low' | 'info';
  value?: string;
}

interface ListViewProps {
  items: ListItem[];
  selectedIndex?: number;
  onSelect?: (index: number) => void;
}

export function ListView({ items, selectedIndex = -1, onSelect }: ListViewProps) {
  const getStatusColor = (status?: string) => {
    switch (status) {
      case 'critical': return 'text-red-400';
      case 'high': return 'text-orange-400';
      case 'medium': return 'text-yellow-400';
      case 'low': return 'text-green-400';
      case 'info': return 'text-cyan-400';
      default: return 'text-green-500';
    }
  };

  const getStatusSymbol = (status?: string) => {
    switch (status) {
      case 'critical': return '●';
      case 'high': return '◆';
      case 'medium': return '▲';
      case 'low': return '○';
      case 'info': return 'ℹ';
      default: return '•';
    }
  };

  return (
    <div className="space-y-0">
      {items.map((item, idx) => (
        <div
          key={item.id}
          onClick={() => onSelect?.(idx)}
          className={`px-2 py-1 cursor-pointer flex items-center justify-between ${
            idx === selectedIndex
              ? 'bg-cyan-950/50 text-cyan-300'
              : 'text-green-400 hover:bg-green-950/30'
          }`}
        >
          <div className="flex items-center gap-2">
            <span className={getStatusColor(item.status)}>
              {getStatusSymbol(item.status)}
            </span>
            <span>{item.label}</span>
          </div>
          {item.value && (
            <span className={getStatusColor(item.status)}>
              {item.value}
            </span>
          )}
        </div>
      ))}
    </div>
  );
}
