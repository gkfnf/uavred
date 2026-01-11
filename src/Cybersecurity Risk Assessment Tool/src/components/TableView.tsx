interface TableColumn {
  key: string;
  header: string;
  width?: string;
}

interface TableRow {
  [key: string]: string | number;
  _status?: 'critical' | 'high' | 'medium' | 'low';
}

interface TableViewProps {
  columns: TableColumn[];
  rows: TableRow[];
}

export function TableView({ columns, rows }: TableViewProps) {
  const getStatusColor = (status?: string) => {
    switch (status) {
      case 'critical': return 'text-red-400';
      case 'high': return 'text-orange-400';
      case 'medium': return 'text-yellow-400';
      case 'low': return 'text-green-400';
      default: return 'text-green-500';
    }
  };

  return (
    <div className="overflow-x-auto">
      <table className="w-full text-sm">
        <thead>
          <tr className="border-b border-green-600">
            {columns.map((col) => (
              <th
                key={col.key}
                className="text-left px-2 py-1 text-cyan-400"
                style={col.width ? { width: col.width } : undefined}
              >
                {col.header}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {rows.map((row, idx) => (
            <tr
              key={idx}
              className={`border-b border-green-900/50 ${getStatusColor(row._status)} hover:bg-green-950/30`}
            >
              {columns.map((col) => (
                <td key={col.key} className="px-2 py-1">
                  {row[col.key]}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
