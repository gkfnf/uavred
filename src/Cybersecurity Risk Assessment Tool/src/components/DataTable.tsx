interface Column {
  key: string;
  header: string;
  width?: string;
}

interface DataTableProps {
  columns: Column[];
  rows: any[];
  onRowClick?: (row: any) => void;
  selectedRow?: any;
}

export function DataTable({ columns, rows, onRowClick, selectedRow }: DataTableProps) {
  const getSeverityColor = (status?: string) => {
    switch (status) {
      case 'critical': return 'text-red-400';
      case 'high': return 'text-orange-400';
      case 'medium': return 'text-yellow-400';
      case 'low': return 'text-green-400';
      default: return 'text-slate-400';
    }
  };

  return (
    <div className="overflow-x-auto">
      <table className="w-full">
        <thead>
          <tr className="border-b border-[#2a2d35]">
            {columns.map((col) => (
              <th
                key={col.key}
                className="text-left px-4 py-3 text-xs text-slate-500 uppercase tracking-wider"
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
              onClick={() => onRowClick?.(row)}
              className={`border-b border-[#2a2d35] transition-colors ${
                onRowClick ? 'cursor-pointer hover:bg-[#22252b]' : ''
              } ${selectedRow === row ? 'bg-[#2a2d35]' : ''}`}
            >
              {columns.map((col) => (
                <td
                  key={col.key}
                  className={`px-4 py-3 text-sm ${getSeverityColor(row._status)}`}
                >
                  {row[col.key]}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
      {rows.length === 0 && (
        <div className="text-center py-12">
          <div className="text-slate-600 text-sm">No data available</div>
        </div>
      )}
    </div>
  );
}
