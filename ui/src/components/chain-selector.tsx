"use client";

import { useState, useEffect, useMemo } from "react";
import { Check, ChevronsUpDown, FlaskConical } from "lucide-react";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";

export type Chain = {
  name?: string;
  chain?: string;
  title?: string;
  network?: string;
  chainId: number;
  testnet: boolean;
  nativeCurrency: NativeCurrency;
};

export type NativeCurrency = {
  name?: string;
  symbol?: string;
  decimals?: number;
};

type ChainSelectorProps = {
  onSelect: (chain: Chain) => void;
};

export function ChainSelector({ onSelect }: ChainSelectorProps) {
  const [open, setOpen] = useState(false);
  const [searchValue, setSearchValue] = useState<string>("");
  const [chains, setChains] = useState<Chain[]>([]);
  const [selectedChain, setSelectedChain] = useState<Chain | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchChains = async () => {
      try {
        setIsLoading(true);
        const response = await fetch("https://chainid.network/chains.json");
        if (!response.ok) {
          throw new Error("Failed to fetch chain data");
        }
        let data: Chain[] = await response.json();
        data = data.map((item) => {
          item.testnet =
            item.name?.toLowerCase().includes("test") ||
            item.title?.toLowerCase().includes("test") ||
            item.network?.toLowerCase().includes("test") ||
            item.name?.toLowerCase().includes("devnet") ||
            item.title?.toLowerCase().includes("devnet") ||
            item.network?.toLowerCase().includes("devnet") ||
            false;
          return item;
        });

        const defaultChain = data.filter((c) => c.chainId == 1)[0];
        setSelectedChain(defaultChain);
        setChains(data);
        onSelect(defaultChain);
      } catch (err) {
        setError("Error loading chain data. Please try again later.");
        console.error("Error fetching chain data:", err);
      } finally {
        setIsLoading(false);
      }
    };

    fetchChains();
  }, []);

  const handleSelect = (chain: Chain) => {
    setSelectedChain(chain);
    setOpen(false);
    onSelect(chain);
  };

  const handleSearch = (value: string) => {
    setSearchValue(value);
  };

  const filteredChains = useMemo(() => {
    const q = searchValue.toLowerCase();
    if (q.trim().length == 0) {
      return chains;
    } else {
      return chains
        .filter(
          (item) =>
            item.name?.toLowerCase().includes(q) ||
            item.title?.toLowerCase().includes(q) ||
            item.chainId.toString().includes(q),
        )
        .sort((a, b) => {
          if ((a.name || a.title)?.toLowerCase() === q) {
            return -1;
          }

          if ((b.name || b.title)?.toLowerCase() === q) {
            return 1;
          }

          return 0;
        });
    }
  }, [searchValue, open]);

  if (isLoading) {
    return (
      <Button variant="outline" className="w-full">
        Loading networks...
      </Button>
    );
  }

  if (error) {
    return (
      <Button variant="outline" className="w-full text-red-500">
        {error}
      </Button>
    );
  }

  const chainListEntry = (chain: Chain) => {
    return (
      <CommandItem
        key={chain.chainId}
        onSelect={() => handleSelect(chain)}
        className="cursor-pointer"
      >
        <Check
          className={cn(
            "mr-2 h-4 w-4",
            selectedChain?.chainId === chain.chainId
              ? "opacity-100"
              : "opacity-0",
          )}
        />
        {chain.name || chain.title}
        <div className="text-slate-400">{chain.chainId}</div>
        <div className=""></div>
        <FlaskConical
          className={cn(
            "mr-2 h-4 w-4 text-pink-700",
            chain?.testnet ? "opacity-100" : "opacity-0",
          )}
        />
      </CommandItem>
    );
  };

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="outline"
          role="combobox"
          aria-expanded={open}
          className="w-full justify-between"
        >
          {selectedChain
            ? `${selectedChain.name || selectedChain.title}`
            : "Select network..."}
          <div className="text-slate-400">{selectedChain?.chainId}</div>
          <FlaskConical
            className={cn(
              "mr-2 h-4 w-4 text-pink-700",
              selectedChain?.testnet ? "opacity-100" : "opacity-0",
            )}
          />
          <div className="w-full"></div>
          <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-80 p-0">
        <Command shouldFilter={false}>
          <CommandInput
            value={searchValue}
            onValueChange={handleSearch}
            placeholder="Search network..."
          />
          <CommandGroup heading="Networks">
            <CommandList>
              {filteredChains
                .slice(0, 25)
                .map((chain) => chainListEntry(chain))}
              <CommandEmpty>No networks found.</CommandEmpty>
            </CommandList>
          </CommandGroup>
        </Command>
      </PopoverContent>
    </Popover>
  );
}
