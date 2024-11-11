"use client";

import { useState, useEffect, useMemo } from "react";
import { Check, ChevronsUpDown } from "lucide-react";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import {
  Command,
  CommandEmpty,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";

type Chain = {
  name?: string;
  chain?: string;
  title?: string;
  network?: string;
  chainId: number;
  testnet: boolean;
};

type ChainSelectorProps = {
  onSelect: (chainId: number) => void;
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

        setChains(data);
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
    onSelect(chain.chainId);
  };

  const handleSearch = (value: string) => {
    setSearchValue(value);
  };

  const filteredChains = useMemo(() => {
    const q = searchValue.toLowerCase();
    return chains
      .filter(
        (item) =>
          item.name?.toLowerCase().includes(q) ||
          item.title?.toLowerCase().includes(q) ||
          item.chainId.toString().includes(q),
      )
      .slice(0, 20);
  }, [searchValue]);

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
            ? `${selectedChain.name || selectedChain.title} (${selectedChain.chainId})`
            : "Select network..."}
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
          <CommandList>
            {filteredChains.map((chain) => (
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
                {chain.name || chain.title} ({chain.chainId})
              </CommandItem>
            ))}
            {filteredChains.length === 0 && (
              <CommandEmpty>No networks found.</CommandEmpty>
            )}
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );
}
