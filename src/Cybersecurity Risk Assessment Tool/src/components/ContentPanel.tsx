import { ReactNode } from 'react';

interface ContentPanelProps {
  title: string;
  subtitle?: string;
  actions?: ReactNode;
  children: ReactNode;
}

export function ContentPanel({ title, subtitle, actions, children }: ContentPanelProps) {
  return (
    <div className="flex flex-col h-full bg-[#1e2128]">
      {/* Header */}
      <div className="border-b border-[#2a2d35] px-6 py-4">
        <div className="flex items-center justify-between">
          <div>
            <h2 className="text-lg text-white">{title}</h2>
            {subtitle && <p className="text-sm text-slate-500 mt-0.5">{subtitle}</p>}
          </div>
          {actions && <div className="flex items-center gap-2">{actions}</div>}
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-auto">
        {children}
      </div>
    </div>
  );
}
