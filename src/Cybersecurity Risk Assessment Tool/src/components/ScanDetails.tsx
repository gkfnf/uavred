import { useState, useEffect } from 'react';

interface ScanStep {
  id: string;
  name: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  timestamp?: string;
  output?: string;
}

interface ScanDetailsProps {
  vulnerability: string;
  target: string;
  onClose?: () => void;
}

export function ScanDetails({ vulnerability, target, onClose }: ScanDetailsProps) {
  const [steps, setSteps] = useState<ScanStep[]>([
    { id: '1', name: 'AI PoC Generation', status: 'completed', timestamp: '14:32:01', output: 'Generated exploitation payload using GPT-4 security model' },
    { id: '2', name: 'Infrastructure Preparation', status: 'completed', timestamp: '14:32:03', output: 'Isolated test environment initialized on container cluster' },
    { id: '3', name: 'Network Reconnaissance', status: 'completed', timestamp: '14:32:05', output: 'Target ports: 22, 80, 443, 8080 | OS: Linux 5.15.0' },
    { id: '4', name: 'Vulnerability Probing', status: 'running', output: 'Testing buffer overflow vector in telemetry parser...' },
    { id: '5', name: 'Exploit Execution', status: 'pending' },
    { id: '6', name: 'Privilege Escalation', status: 'pending' },
    { id: '7', name: 'Impact Analysis', status: 'pending' },
    { id: '8', name: 'Report Generation', status: 'pending' },
  ]);

  const [currentStep, setCurrentStep] = useState(3);

  useEffect(() => {
    const interval = setInterval(() => {
      setCurrentStep(prev => {
        if (prev < steps.length - 1) {
          setSteps(current => 
            current.map((step, idx) => {
              if (idx === prev) {
                return { ...step, status: 'completed', timestamp: new Date().toLocaleTimeString() };
              }
              if (idx === prev + 1) {
                return { ...step, status: 'running' };
              }
              return step;
            })
          );
          return prev + 1;
        }
        return prev;
      });
    }, 3000);

    return () => clearInterval(interval);
  }, []);

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'running': return 'text-cyan-400';
      case 'completed': return 'text-green-400';
      case 'failed': return 'text-red-400';
      case 'pending': return 'text-green-700';
      default: return 'text-green-500';
    }
  };

  const getStatusSymbol = (status: string) => {
    switch (status) {
      case 'running': return '⟳';
      case 'completed': return '✓';
      case 'failed': return '✗';
      case 'pending': return '○';
      default: return '•';
    }
  };

  return (
    <div className="space-y-4">
      <div className="border border-cyan-500/50 bg-cyan-950/20 p-3">
        <div className="flex items-center justify-between mb-2">
          <div className="text-cyan-400">Automated Security Assessment</div>
          {onClose && (
            <button onClick={onClose} className="text-green-700 hover:text-green-400">
              [Close]
            </button>
          )}
        </div>
        <div className="grid grid-cols-2 gap-x-4 gap-y-1 text-sm">
          <div className="flex justify-between">
            <span className="text-green-500">Target:</span>
            <span className="text-green-400">{target}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-green-500">Vulnerability:</span>
            <span className="text-green-400">{vulnerability}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-green-500">Method:</span>
            <span className="text-green-400">AI-Generated PoC</span>
          </div>
          <div className="flex justify-between">
            <span className="text-green-500">Execution:</span>
            <span className="text-green-400">Infrastructure-Level</span>
          </div>
        </div>
      </div>

      <div className="border border-green-600 p-3">
        <div className="text-green-400 mb-3">Scan Progress</div>
        <div className="space-y-2">
          {steps.map((step, idx) => (
            <div key={step.id} className="space-y-1">
              <div className="flex items-center gap-2">
                <span className={`${getStatusColor(step.status)} ${step.status === 'running' ? 'animate-spin' : ''} w-5`}>
                  {getStatusSymbol(step.status)}
                </span>
                <span className={`flex-1 ${getStatusColor(step.status)}`}>
                  {step.name}
                </span>
                {step.timestamp && (
                  <span className="text-green-700 text-sm">[{step.timestamp}]</span>
                )}
              </div>
              {step.output && (
                <div className="ml-7 text-sm text-green-500 font-mono bg-black/50 p-2 border-l-2 border-green-900">
                  {step.output}
                </div>
              )}
            </div>
          ))}
        </div>
      </div>

      <div className="border border-green-600 p-3">
        <div className="text-green-400 mb-2">AI PoC Details</div>
        <div className="space-y-1 text-sm font-mono">
          <div className="text-yellow-400">// AI-Generated Proof of Concept</div>
          <div className="text-green-500">exploit_vector: buffer_overflow</div>
          <div className="text-green-500">target_function: telemetry_parser()</div>
          <div className="text-green-500">payload_size: 1024 bytes</div>
          <div className="text-green-500">shellcode: reverse_tcp_shell</div>
          <div className="text-green-500">callback_host: 10.0.0.1:4444</div>
          <div className="text-yellow-400 mt-2">// Infrastructure Execution Environment</div>
          <div className="text-green-500">runtime: docker_container_isolated</div>
          <div className="text-green-500">network: vlan_segmented_100</div>
          <div className="text-green-500">monitoring: real_time_pcap</div>
        </div>
      </div>
    </div>
  );
}
