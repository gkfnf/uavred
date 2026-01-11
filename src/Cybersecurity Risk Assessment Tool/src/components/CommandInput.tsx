import { useState, KeyboardEvent } from 'react';

interface CommandInputProps {
  onCommand: (command: string) => void;
}

export function CommandInput({ onCommand }: CommandInputProps) {
  const [input, setInput] = useState('');

  const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && input.trim()) {
      onCommand(input.trim());
      setInput('');
    }
  };

  return (
    <div className="flex items-center gap-2 mt-4">
      <span className="text-green-400">$</span>
      <input
        type="text"
        value={input}
        onChange={(e) => setInput(e.target.value)}
        onKeyDown={handleKeyDown}
        className="flex-1 bg-transparent border-none outline-none text-green-300 placeholder:text-green-700"
        placeholder="Enter command..."
        autoFocus
      />
      <span className="text-green-400 animate-pulse">_</span>
    </div>
  );
}
