export function TerminalHeader() {
  return (
    <div className="border-b border-green-500/30 pb-4 mb-4">
      <pre className="text-green-400 text-xs leading-tight">
{`
██╗   ██╗ █████╗ ██╗   ██╗    ██████╗ ██╗███████╗██╗  ██╗
██║   ██║██╔══██╗██║   ██║    ██╔══██╗██║██╔════╝██║ ██╔╝
██║   ██║███████║██║   ██║    ██████╔╝██║███████╗█████╔╝ 
██║   ██║██╔══██║╚██╗ ██╔╝    ██╔══██╗██║╚════██║██╔═██╗ 
╚██████╔╝██║  ██║ ╚████╔╝     ██║  ██║██║███████║██║  ██╗
 ╚═════╝ ╚═╝  ╚═╝  ╚═══╝      ╚═╝  ╚═╝╚═╝╚══════╝╚═╝  ╚═╝
         Cybersecurity Risk Assessment System v2.4.1
`}
      </pre>
      <div className="text-green-500/70 text-sm mt-2">
        <p>Unmanned Aerial Vehicle Security Analysis Platform</p>
        <p className="text-xs mt-1">Type 'help' for available commands</p>
      </div>
    </div>
  );
}
