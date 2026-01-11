import { useState } from 'react';

interface TreeNode {
  id: string;
  label: string;
  children?: TreeNode[];
  status?: 'critical' | 'high' | 'medium' | 'low' | 'ok';
  metadata?: string;
}

interface TreeViewProps {
  nodes: TreeNode[];
  level?: number;
}

export function TreeView({ nodes, level = 0 }: TreeViewProps) {
  const [expanded, setExpanded] = useState<Record<string, boolean>>({});

  const toggleNode = (id: string) => {
    setExpanded(prev => ({ ...prev, [id]: !prev[id] }));
  };

  const getStatusColor = (status?: string) => {
    switch (status) {
      case 'critical': return 'text-red-400';
      case 'high': return 'text-orange-400';
      case 'medium': return 'text-yellow-400';
      case 'low': return 'text-green-400';
      case 'ok': return 'text-green-500';
      default: return 'text-green-500';
    }
  };

  const getStatusIcon = (status?: string) => {
    switch (status) {
      case 'critical': return '✗';
      case 'high': return '!';
      case 'medium': return '⚠';
      case 'low': return '◆';
      case 'ok': return '✓';
      default: return '•';
    }
  };

  return (
    <div className="space-y-0">
      {nodes.map((node) => (
        <div key={node.id}>
          <div
            onClick={() => node.children && toggleNode(node.id)}
            className="flex items-center gap-2 px-2 py-0.5 hover:bg-green-950/30 cursor-pointer"
            style={{ paddingLeft: `${level * 1.5 + 0.5}rem` }}
          >
            {node.children && (
              <span className="text-yellow-400 w-3">
                {expanded[node.id] ? '▼' : '▶'}
              </span>
            )}
            {!node.children && <span className="w-3" />}
            <span className={getStatusColor(node.status)}>
              {getStatusIcon(node.status)}
            </span>
            <span className="text-green-400">{node.label}</span>
            {node.metadata && (
              <span className="text-green-700 text-xs ml-auto">{node.metadata}</span>
            )}
          </div>
          {node.children && expanded[node.id] && (
            <TreeView nodes={node.children} level={level + 1} />
          )}
        </div>
      ))}
    </div>
  );
}
