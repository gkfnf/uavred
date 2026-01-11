import { ReactNode } from 'react';

interface BorderedPanelProps {
  title?: string;
  children: ReactNode;
  className?: string;
  focused?: boolean;
  height?: string;
}

export function BorderedPanel({ title, children, className = '', focused = false, height }: BorderedPanelProps) {
  const borderColor = focused ? 'border-cyan-400' : 'border-green-600';
  
  return (
    <div className={`border ${borderColor} ${className}`} style={height ? { height } : undefined}>
      {title && (
        <div className={`border-b ${borderColor} px-3 py-1 ${focused ? 'bg-cyan-950/30 text-cyan-300' : 'bg-green-950/30 text-green-400'}`}>
          {title}
        </div>
      )}
      <div className="p-3">
        {children}
      </div>
    </div>
  );
}
