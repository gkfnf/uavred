import { Download, FileJson, FileText, FileCode, CheckCircle } from 'lucide-react';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription } from './ui/dialog';

interface ExportDialogProps {
  show: boolean;
  onClose: () => void;
  data: any;
  dataType: 'assets' | 'vulnerabilities' | 'traffic' | 'scan-results' | 'full-report';
}

export function ExportDialog({ show, onClose, data, dataType }: ExportDialogProps) {
  const exportFormats = [
    { 
      id: 'json', 
      name: 'JSON', 
      icon: FileJson, 
      description: 'Machine-readable format for automation',
      extension: '.json'
    },
    { 
      id: 'csv', 
      name: 'CSV', 
      icon: FileText, 
      description: 'Spreadsheet compatible format',
      extension: '.csv'
    },
    { 
      id: 'markdown', 
      name: 'Markdown', 
      icon: FileCode, 
      description: 'Human-readable report format',
      extension: '.md'
    },
    { 
      id: 'html', 
      name: 'HTML Report', 
      icon: FileCode, 
      description: 'Styled web report with charts',
      extension: '.html'
    },
  ];

  const handleExport = (format: string) => {
    let content = '';
    let filename = `uav-security-${dataType}-${new Date().toISOString().split('T')[0]}`;
    
    switch (format) {
      case 'json':
        content = JSON.stringify(data, null, 2);
        break;
      case 'csv':
        content = convertToCSV(data);
        break;
      case 'markdown':
        content = convertToMarkdown(data, dataType);
        break;
      case 'html':
        content = convertToHTML(data, dataType);
        break;
    }

    // Create download
    const blob = new Blob([content], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename + exportFormats.find(f => f.id === format)?.extension;
    a.click();
    URL.revokeObjectURL(url);

    onClose();
  };

  const convertToCSV = (data: any): string => {
    if (Array.isArray(data) && data.length > 0) {
      const headers = Object.keys(data[0]).join(',');
      const rows = data.map(item => Object.values(item).join(',')).join('\n');
      return `${headers}\n${rows}`;
    }
    return '';
  };

  const convertToMarkdown = (data: any, type: string): string => {
    let md = `# UAV Security Assessment - ${type}\n\n`;
    md += `**Generated:** ${new Date().toLocaleString()}\n\n`;
    md += `## Summary\n\n`;
    
    if (Array.isArray(data)) {
      md += `Total items: ${data.length}\n\n`;
      md += `## Details\n\n`;
      data.forEach((item, idx) => {
        md += `### ${idx + 1}. ${item.name || item.title || item.id}\n\n`;
        Object.entries(item).forEach(([key, value]) => {
          md += `- **${key}**: ${value}\n`;
        });
        md += `\n`;
      });
    }
    
    return md;
  };

  const convertToHTML = (data: any, type: string): string => {
    return `
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>UAV Security Report - ${type}</title>
  <style>
    body { font-family: system-ui; max-width: 1200px; margin: 40px auto; padding: 20px; background: #0f172a; color: #e2e8f0; }
    h1 { color: #fb923c; border-bottom: 2px solid #fb923c; padding-bottom: 10px; }
    .stat { background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 15px; margin: 10px 0; }
    .critical { border-left: 4px solid #ef4444; }
    .high { border-left: 4px solid #f97316; }
    .medium { border-left: 4px solid #eab308; }
    table { width: 100%; border-collapse: collapse; margin: 20px 0; }
    th, td { padding: 12px; text-align: left; border-bottom: 1px solid #334155; }
    th { background: #1e293b; color: #fb923c; }
  </style>
</head>
<body>
  <h1>UAV Security Assessment Report</h1>
  <p><strong>Type:</strong> ${type}</p>
  <p><strong>Generated:</strong> ${new Date().toLocaleString()}</p>
  <div class="content">
    <pre>${JSON.stringify(data, null, 2)}</pre>
  </div>
</body>
</html>
    `;
  };

  return (
    <Dialog open={show} onOpenChange={onClose}>
      <DialogContent className="bg-slate-900 border-slate-700 max-w-2xl">
        <DialogHeader>
          <DialogTitle className="text-slate-100">Export Data</DialogTitle>
          <DialogDescription className="text-slate-400 text-xs">
            Choose a format to export your {dataType.replace('-', ' ')} data
          </DialogDescription>
        </DialogHeader>
        <div className="space-y-3">
          {exportFormats.map((format) => (
            <button
              key={format.id}
              onClick={() => handleExport(format.id)}
              className="w-full flex items-center gap-4 p-4 rounded-lg bg-slate-950/50 border border-slate-800 hover:border-cyan-500/50 hover:bg-slate-900/50 transition-all group text-left"
            >
              <div className="w-10 h-10 rounded-lg bg-cyan-900/20 border border-cyan-800/30 flex items-center justify-center group-hover:bg-cyan-900/30 transition-all">
                <format.icon className="w-5 h-5 text-cyan-400" />
              </div>
              <div className="flex-1">
                <div className="text-sm text-slate-200 font-medium mb-1">{format.name}</div>
                <div className="text-xs text-slate-500">{format.description}</div>
              </div>
              <Download className="w-4 h-4 text-slate-600 group-hover:text-cyan-400 transition-colors" />
            </button>
          ))}
        </div>

        <div className="mt-4 p-3 rounded-lg bg-gradient-to-br from-purple-950/30 to-slate-950 border border-purple-800/30">
          <div className="flex items-start gap-2">
            <CheckCircle className="w-4 h-4 text-purple-400 flex-shrink-0 mt-0.5" />
            <div className="text-xs text-slate-400">
              All exports are generated locally in your browser. No data is sent to external servers.
            </div>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
