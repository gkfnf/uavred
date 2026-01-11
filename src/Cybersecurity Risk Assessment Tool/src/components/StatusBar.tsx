interface KeyBinding {
  key: string;
  description: string;
}

interface StatusBarProps {
  keyBindings: KeyBinding[];
  status?: string;
}

export function StatusBar({ keyBindings, status }: StatusBarProps) {
  return (
    <div className="border-t border-green-600 bg-black px-3 py-2 flex items-center justify-between">
      <div className="flex gap-4">
        {keyBindings.map((binding, idx) => (
          <div key={idx} className="text-sm">
            <span className="text-yellow-400">{binding.key}</span>
            <span className="text-green-500"> {binding.description}</span>
          </div>
        ))}
      </div>
      {status && (
        <div className="text-green-500 text-sm">{status}</div>
      )}
    </div>
  );
}
