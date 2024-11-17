import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function isValidEthereumAddress(address: string): boolean {
  return /^0x[a-fA-F0-9]{40}$/.test(address);
};

export function  isValidBitcoinAddress(address: string): boolean {
  // Legacy addresses: start with 1
  // P2SH addresses: start with 3
  // Bech32 addresses: start with bc1
  // P2SH-P2WPKH addresses: start with 3
  // Testnet addresses: start with m, n, or 2
  // Testnet Bech32: start with tb1
  return /^(1|3|bc1|[mn2]|tb1)[a-zA-Z0-9]{25,89}$/.test(address);
};