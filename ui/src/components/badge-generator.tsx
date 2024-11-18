"use client";

import { useState, useEffect } from "react";
import { Wallet, Coins } from "lucide-react";
import { EthereumBadge, BitcoinBadge } from "cryptocons";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { ChainSelector, Chain } from "./chain-selector";
import { isValidBitcoinAddress, isValidEthereumAddress } from "@/lib/utils";

const BASE_URL = window.location.origin;
const BADGE_BASE_URL = `${BASE_URL}/badge`;
const LINK_BASE_URL = `${BASE_URL}/scanner`;

export default function BadgeGenerator() {
  const [selectedChain, setSelectedChain] = useState("ethereum");
  const [queryType, setQueryType] = useState("eth");
  const [address, setAddress] = useState("");
  const [tokenAddress, setTokenAddress] = useState("");
  const [badgeUrl, setBadgeUrl] = useState("");
  const [badgeLinkUrl, setBadgeLinkUrl] = useState("");
  const [evmChain, setEvmChain] = useState<Chain | null>(null);
  const [btcNetwork, setBtcNetwork] = useState("mainnet");
  const [addressError, setAddressError] = useState<string>("");
  const [tokenAddressError, setTokenAddressError] = useState<string>("");

  useEffect(() => {
    let isValid = true;

    // Reset errors initially
    setAddressError("");
    setTokenAddressError("");

    // Validate wallet address
    if (!address) {
      isValid = false;
    } else if (selectedChain === "ethereum") {
      if (!isValidEthereumAddress(address)) {
        setAddressError("Invalid Ethereum address format");
        isValid = false;
      }
    } else if (selectedChain === "bitcoin") {
      if (!isValidBitcoinAddress(address)) {
        setAddressError("Invalid Bitcoin address format");
        isValid = false;
      }
    }

    // Validate ERC20 token address
    if (selectedChain === "ethereum" && queryType === "erc20") {
      if (!tokenAddress) {
        setTokenAddressError("Token address is required");
        isValid = false;
      } else if (!isValidEthereumAddress(tokenAddress)) {
        setTokenAddressError("Invalid ERC20 token address format");
        isValid = false;
      }
    }

    if (!isValid) {
      setBadgeUrl("");
      return;
    }

    // Generate URL
    let url = "";
    switch (selectedChain) {
      case "ethereum":
        switch (queryType) {
          case "eth":
            url = `evm/${evmChain?.chainId}/balance/${address}`;
            break;
          case "erc20":
            url = `evm/${evmChain?.chainId}/erc20_balance/${tokenAddress}/${address}`;
            break;
        }
        break;
      case "bitcoin":
        url = `btc/${btcNetwork}/balance/${address}`;
        break;
    }
    setBadgeUrl(`${BADGE_BASE_URL}/${url}`);
    setBadgeLinkUrl(`${LINK_BASE_URL}/${url}`);
  }, [selectedChain, queryType, address, tokenAddress, evmChain, btcNetwork]);

  return (
    <Card className="max-w-2xl mx-auto bg-white">
      <CardHeader>
        <CardTitle className="text-2xl font-bold text-gray-900">
          Generate Your Crypto Badge
        </CardTitle>
        <CardDescription className="text-gray-500">
          Create a dynamic badge showing cryptocurrency balances for any address
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-6">
        <div className="space-y-2">
          <Label className="text-sm font-medium text-gray-700">
            Select Blockchain
          </Label>
          <RadioGroup
            defaultValue="ethereum"
            onValueChange={setSelectedChain}
            className="grid grid-cols-2 gap-4"
          >
            <Label
              htmlFor="ethereum"
              className="flex flex-col items-center justify-between rounded-md border-2 border-gray-200 bg-white p-4 hover:bg-gray-50 [&:has([data-state=checked])]:border-blue-600"
            >
              <RadioGroupItem
                value="ethereum"
                id="ethereum"
                className="sr-only"
              />
              <EthereumBadge className="mb-3 h-8 w-8" />
              <span className="text-sm font-medium text-gray-900">
                Ethereum (EVM)
              </span>
            </Label>
            <Label
              htmlFor="bitcoin"
              className="flex flex-col items-center justify-between rounded-md border-2 border-gray-200 bg-white p-4 hover:bg-gray-50 [&:has([data-state=checked])]:border-blue-600"
            >
              <RadioGroupItem
                value="bitcoin"
                id="bitcoin"
                className="sr-only"
              />
              <BitcoinBadge className="mb-3 h-8 w-8" />
              <span className="text-sm font-medium text-gray-900">Bitcoin</span>
            </Label>
          </RadioGroup>
        </div>

        {selectedChain === "ethereum" && (
          <>
            <div className="space-y-2">
              <Label className="text-sm font-medium text-gray-700">
                Select Network
              </Label>
              <ChainSelector onSelect={setEvmChain} />
            </div>

            <div className="space-y-2">
              <Label className="text-sm font-medium text-gray-700">
                Query Type
              </Label>
              <Select
                defaultValue={queryType || "eth"}
                onValueChange={setQueryType}
              >
                <SelectTrigger className="w-full bg-white border-gray-300">
                  <SelectValue placeholder="Select query type" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="eth">
                    {evmChain?.nativeCurrency.symbol} Balance
                  </SelectItem>
                  <SelectItem value="erc20">ERC20 Token Balance</SelectItem>
                </SelectContent>
              </Select>
            </div>

            {queryType === "erc20" && (
              <div className="space-y-2">
                <Label
                  htmlFor="tokenAddress"
                  className="text-sm font-medium text-gray-700"
                >
                  ERC20 Token Contract Address
                </Label>
                <div className="relative">
                  <Coins className="absolute left-2 top-2.5 h-5 w-5 text-gray-400" />
                  <Input
                    id="tokenAddress"
                    placeholder="Enter ERC20 token contract address"
                    value={tokenAddress}
                    onChange={(e) => setTokenAddress(e.target.value)}
                    className={`pl-9 bg-white border-gray-300 ${
                      tokenAddressError ? "border-red-500" : ""
                    }`}
                  />
                </div>
                {tokenAddressError && (
                  <p className="text-sm text-red-500">{tokenAddressError}</p>
                )}
              </div>
            )}
          </>
        )}

        {selectedChain === "bitcoin" && (
          <div className="space-y-2">
            <Label className="text-sm font-medium text-gray-700">
              Select Network
            </Label>
            <Select defaultValue="mainnet" onValueChange={setBtcNetwork}>
              <SelectTrigger className="w-full bg-white border-gray-300">
                <SelectValue placeholder="Select Bitcoin network" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="mainnet">Mainnet</SelectItem>
                <SelectItem value="testnet">Testnet</SelectItem>
                <SelectItem value="signet">Signet</SelectItem>
              </SelectContent>
            </Select>
          </div>
        )}

        <div className="space-y-2">
          <Label
            htmlFor="address"
            className="text-sm font-medium text-gray-700"
          >
            Wallet Address
          </Label>
          <div className="relative">
            <Wallet className="absolute left-2 top-2.5 h-5 w-5 text-gray-400" />
            <Input
              id="address"
              placeholder={`Enter ${selectedChain} address`}
              value={address}
              onChange={(e) => setAddress(e.target.value)}
              className={`pl-9 bg-white border-gray-300 ${
                addressError ? "border-red-500" : ""
              }`}
            />
          </div>
          {addressError && (
            <p className="text-sm text-red-500">{addressError}</p>
          )}
        </div>

        <Tabs defaultValue="preview" className="w-full">
          <TabsList className="grid w-full grid-cols-2 bg-gray-100">
            <TabsTrigger
              value="preview"
              className="data-[state=active]:bg-white"
            >
              Preview
            </TabsTrigger>
            <TabsTrigger
              value="markdown"
              className="data-[state=active]:bg-white"
            >
              Markdown
            </TabsTrigger>
          </TabsList>
          <TabsContent
            value="preview"
            className="space-y-4 bg-white border border-gray-200 rounded-b-lg p-4"
          >
            <div className="flex justify-center py-4">
              {badgeUrl ? (
                <a target="_blank" href={badgeLinkUrl}>
                  <img src={badgeUrl} alt="Crypto Balance Badge" />
                </a>
              ) : (
                <p className="text-gray-500">
                  Enter an address to generate a badge
                </p>
              )}
            </div>
          </TabsContent>
          <TabsContent
            value="markdown"
            className="space-y-4 bg-white border border-gray-200 rounded-b-lg p-4"
          >
            <Input
              readOnly
              value={
                badgeUrl
                  ? `[![${selectedChain} Balance](${badgeUrl})](${badgeLinkUrl})`
                  : ""
              }
              onClick={(e) => (e.target as HTMLInputElement).select()}
              className="bg-gray-50 border-gray-300"
            />
            <p className="text-sm text-gray-500 text-center">
              {badgeUrl
                ? "Click to copy the markdown code"
                : "Enter an address to generate markdown"}
            </p>
          </TabsContent>
        </Tabs>

        <div className="text-center text-sm text-gray-500">
          <p>
            CryptoShield: Your go-to tool for dynamic cryptocurrency balance
            badges
          </p>
        </div>
      </CardContent>
    </Card>
  );
}
