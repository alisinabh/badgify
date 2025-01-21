"use client";

import { useState, useEffect } from "react";
import { Wallet, Coins, ChevronDown, ChevronRight } from "lucide-react";
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
import { Checkbox } from "@/components/ui/checkbox";
import { CopyableInput } from "@/components/ui/copyable-input";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip";
import { cn } from "@/lib/utils";

const BASE_URL = process.env.NODE_ENV === 'development' 
  ? 'http://localhost:8080' 
  : window.location.origin;
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
  const [linkToBrowser, setLinkToBrowser] = useState(true);
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [badgeColor, setBadgeColor] = useState<string>("");
  const [warningThreshold, setWarningThreshold] = useState<string>("");
  const [icon, setIcon] = useState<string>("");

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

    // Add color parameter if specified
    const params = new URLSearchParams();
    if (badgeColor) {
      params.append('color', badgeColor);
    }
    if (warningThreshold) {
      params.append('warning_threshold', warningThreshold);
    }
    if (icon) {
      params.append('icon', icon);
    }
    const queryString = params.toString();
    
    setBadgeUrl(`${BADGE_BASE_URL}/${url}${queryString ? `?${queryString}` : ''}`);
    setBadgeLinkUrl(`${LINK_BASE_URL}/${url}`);
  }, [selectedChain, queryType, address, tokenAddress, evmChain, btcNetwork, badgeColor, warningThreshold, icon]);

  return (
    <Card className="max-w-2xl mx-auto bg-white">
      <CardHeader>
        <CardTitle className="text-2xl font-bold text-gray-900">
          Generate Your Crypto Badge
        </CardTitle>
        <CardDescription className="text-gray-500">
          Create dynamic badges that display real-time cryptocurrency balances
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
            htmlFor="wallet_address"
            className="text-sm font-medium text-gray-700"
          >
            Wallet Address
          </Label>
          <div className="relative">
            <Wallet className="absolute left-2 top-2.5 h-5 w-5 text-gray-400" />
            <Input
              id="wallet_address"
              name="wallet_address"
              placeholder={`Enter ${selectedChain} address`}
              value={address}
              autoComplete="eth-wallet eth-wallet-address wallet-address btc-address btc-wallet-address ethereum btc"
              onChange={(e) => setAddress(e.target.value)}
              className={`pl-9 ${addressError ? "border-red-500" : ""}`}
            />
          </div>
          {addressError && (
            <p className="text-sm text-red-500">{addressError}</p>
          )}
        </div>

        {/* Advanced Configuration Section */}
        <div className="space-y-4">
          <button
            onClick={() => setShowAdvanced(!showAdvanced)}
            className="flex items-center gap-2 text-sm text-gray-600 hover:text-gray-900"
          >
            {showAdvanced ? <ChevronDown className="h-4 w-4" /> : <ChevronRight className="h-4 w-4" />}
            Advanced Configuration
          </button>
          
          {showAdvanced && (
            <div className="space-y-4 border rounded-lg p-4 bg-gray-50">
              <div className="space-y-2">
                <Label
                  htmlFor="icon"
                  className="text-sm font-medium text-gray-700"
                >
                  Custom Icon
                </Label>
                <Input
                  type="text"
                  id="icon"
                  placeholder="Enter icon name from simpleicons.org"
                  value={icon}
                  onChange={(e) => setIcon(e.target.value)}
                  className="bg-white"
                />
                <p className="text-xs text-gray-500">
                  Use any icon name from <a href="https://simpleicons.org" target="_blank" rel="noopener noreferrer" className="text-blue-500 hover:underline">simpleicons.org</a>
                </p>
              </div>

              <div className="space-y-2">
                <Label
                  htmlFor="badge_color"
                  className="text-sm font-medium text-gray-700"
                >
                  Badge Color Override
                </Label>
                <Input
                  type="text"
                  id="badge_color"
                  placeholder="Enter color name or hex code (e.g., blue, #ff0000)"
                  value={badgeColor}
                  onChange={(e) => setBadgeColor(e.target.value)}
                  className="bg-white"
                />
                <p className="text-xs text-gray-500">
                  Customize your badge with any valid CSS color value
                </p>
              </div>

              <div className="space-y-2">
                <Label
                  htmlFor="warning_threshold"
                  className="text-sm font-medium text-gray-700"
                >
                  Warning Threshold
                </Label>
                <TooltipProvider>
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <div>
                        <Input
                          type="text"
                          id="warning_threshold"
                          placeholder="e.g., 0.1 or 1.234"
                          value={warningThreshold}
                          onChange={(e) => setWarningThreshold(e.target.value)}
                          disabled={!!badgeColor}
                          className={cn(
                            "bg-white",
                            badgeColor && "cursor-not-allowed opacity-50"
                          )}
                        />
                      </div>
                    </TooltipTrigger>
                    <TooltipContent side="top" className={cn(!badgeColor && "hidden")}>
                      <p>Warning threshold is unavailable when a custom color is set</p>
                    </TooltipContent>
                  </Tooltip>
                </TooltipProvider>
                <p className="text-xs text-gray-500">
                  Set a threshold value to determine when the badge changes to a warning state
                </p>
              </div>

              <div className="space-y-2">
                <div className="flex items-center space-x-2">
                  <Checkbox 
                    id="browser-link" 
                    checked={linkToBrowser}
                    onCheckedChange={(checked: boolean) => setLinkToBrowser(checked)}
                    className="border-gray-300"
                  />
                  <Label htmlFor="browser-link" className="text-sm text-gray-600 flex items-center gap-1">
                    Link to block explorer
                    <span className="text-xs text-gray-400">(recommended)</span>
                  </Label>
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Preview section */}
        <div className="space-y-2 bg-white border border-gray-200 rounded-lg p-3">
          <div className="relative">
            <span className="absolute top-0 right-0 text-xs text-gray-400 mr-1">Live Preview</span>
            <div className="flex justify-center py-3">
              {badgeUrl ? (
                linkToBrowser ? (
                  <a target="_blank" href={badgeLinkUrl}>
                    <img src={badgeUrl} alt={`${selectedChain} Balance Badge`} />
                  </a>
                ) : (
                  <img src={badgeUrl} alt={`${selectedChain} Balance Badge`} />
                )
              ) : (
                <p className="text-gray-500">
                  Enter an address to preview your badge
                </p>
              )}
            </div>
          </div>
        </div>

        <Tabs defaultValue="markdown" className="w-full">
          <TabsList className="grid w-full grid-cols-3 bg-gray-100">
            <TabsTrigger
              value="markdown"
              className="data-[state=active]:bg-white"
            >
              Markdown
            </TabsTrigger>
            <TabsTrigger
              value="html"
              className="data-[state=active]:bg-white"
            >
              HTML
            </TabsTrigger>
            <TabsTrigger
              value="image"
              className="data-[state=active]:bg-white"
            >
              Image URL
            </TabsTrigger>
          </TabsList>
          <TabsContent
            value="markdown"
            className="space-y-4 bg-white border border-gray-200 rounded-b-lg p-4"
          >
            {badgeUrl ? (
              <CopyableInput
                value={
                  linkToBrowser 
                    ? `[![${selectedChain} Balance](${badgeUrl})](${badgeLinkUrl})`
                    : `![${selectedChain} Balance](${badgeUrl})`
                }
                className="bg-gray-50 border-gray-300"
              />
            ) : null}
            <p className="text-sm text-gray-500 text-center">
              {badgeUrl
                ? "Click to copy the markdown code"
                : "Enter an address to generate markdown"}
            </p>
          </TabsContent>
          <TabsContent
            value="html"
            className="space-y-4 bg-white border border-gray-200 rounded-b-lg p-4"
          >
            {badgeUrl ? (
              <CopyableInput
                value={
                  linkToBrowser
                    ? `<a href="${badgeLinkUrl}" target="_blank"><img src="${badgeUrl}" alt="${selectedChain} Balance"></a>`
                    : `<img src="${badgeUrl}" alt="${selectedChain} Balance">`
                }
                className="bg-gray-50 border-gray-300"
              />
            ) : null}
            <p className="text-sm text-gray-500 text-center">
              {badgeUrl
                ? "Click to copy the HTML code"
                : "Enter an address to generate HTML"}
            </p>
          </TabsContent>
          <TabsContent
            value="image"
            className="space-y-4 bg-white border border-gray-200 rounded-b-lg p-4"
          >
            {badgeUrl ? (
              <CopyableInput
                value={badgeUrl}
                className="bg-gray-50 border-gray-300"
              />
            ) : null}
            <p className="text-sm text-gray-500 text-center">
              {badgeUrl
                ? "Click to copy the direct image URL"
                : "Enter an address to generate image URL"}
            </p>
          </TabsContent>
        </Tabs>
      </CardContent>
    </Card>
  );
}
