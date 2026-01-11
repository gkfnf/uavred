interface ThreatItem {
  name: string;
  description: string;
  severity: 'critical' | 'high' | 'medium' | 'low';
}

interface ThreatCategory {
  id: string;
  title: string;
  icon: string;
  type: 'software' | 'data' | 'internal' | 'abuse';
  threats: ThreatItem[];
}

interface ThreatCategoriesProps {
  categories: ThreatCategory[];
}

export function ThreatCategories({ categories }: ThreatCategoriesProps) {
  const getTypeColor = (type: string) => {
    switch (type) {
      case 'software': return 'border-purple-500/50 bg-purple-950/20';
      case 'data': return 'border-blue-500/50 bg-blue-950/20';
      case 'internal': return 'border-cyan-500/50 bg-cyan-950/20';
      case 'abuse': return 'border-pink-500/50 bg-pink-950/20';
      default: return 'border-green-500/50 bg-green-950/20';
    }
  };

  const getTitleColor = (type: string) => {
    switch (type) {
      case 'software': return 'text-purple-400';
      case 'data': return 'text-blue-400';
      case 'internal': return 'text-cyan-400';
      case 'abuse': return 'text-pink-400';
      default: return 'text-green-400';
    }
  };

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case 'critical': return 'text-red-400';
      case 'high': return 'text-orange-400';
      case 'medium': return 'text-yellow-400';
      case 'low': return 'text-green-400';
      default: return 'text-green-500';
    }
  };

  const getSeverityIcon = (severity: string) => {
    switch (severity) {
      case 'critical': return '⚠';
      case 'high': return '▲';
      case 'medium': return '◆';
      case 'low': return '○';
      default: return '•';
    }
  };

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-3">
      {categories.map((category) => (
        <div
          key={category.id}
          className={`border ${getTypeColor(category.type)} p-3 hover:border-opacity-100 transition-all`}
        >
          <div className="flex items-start gap-2 mb-3 pb-2 border-b border-green-900/30">
            <span className="text-lg">{category.icon}</span>
            <div className="flex-1">
              <div className={`${getTitleColor(category.type)}`}>
                {category.title}
              </div>
            </div>
          </div>
          <div className="space-y-2">
            {category.threats.map((threat, idx) => (
              <div key={idx} className="text-sm">
                <div className="flex items-start gap-2">
                  <span className={getSeverityColor(threat.severity)}>
                    {getSeverityIcon(threat.severity)}
                  </span>
                  <div className="flex-1">
                    <div className="text-green-400">{threat.name}</div>
                    <div className="text-green-700 text-xs mt-0.5">
                      {threat.description}
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}
