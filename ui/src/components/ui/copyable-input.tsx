import { useState } from "react";
import { Input } from "./input";

interface CopyableInputProps {
  value: string;
  className?: string;
}

export function CopyableInput({ value, className }: CopyableInputProps) { 
  const [copiedTimeout, setCopiedTimeout] = useState<NodeJS.Timeout | null>(null);
  const [showCopied, setShowCopied] = useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText(value);
    
    if (copiedTimeout) {
      clearTimeout(copiedTimeout);
    }
    
    setShowCopied(true);
    
    const timeout = setTimeout(() => {
      setShowCopied(false);
    }, 2000);
    
    setCopiedTimeout(timeout);
  };

  return (
    <div className="relative">
      <Input
        readOnly
        value={value}
        onClick={handleCopy}
        className={`cursor-pointer ${className || ''}`}
      />
      {showCopied && (
        <div className="absolute -top-8 left-1/2 transform -translate-x-1/2 bg-gray-800 text-white px-2 py-1 rounded text-sm">
          Copied!
        </div>
      )}
    </div>
  );
} 