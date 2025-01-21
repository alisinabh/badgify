import { Shield, Wallet, Coins, ArrowRight } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Link } from "react-router-dom";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

export default function IntroPage() {
  return (
    <div className="max-w-4xl mx-auto space-y-8 py-8">
      <div className="text-center space-y-4">
        <h1 className="text-4xl font-bold tracking-tight">
          Welcome to Badgify
        </h1>
        <p className="text-xl text-muted-foreground">
          Create beautiful, real-time cryptocurrency badges for your projects
        </p>
      </div>

      <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Shield className="h-5 w-5 text-blue-500" />
              Dynamic Badges
            </CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-muted-foreground">
              Generate real-time badges showing wallet balances, token holdings,
              and more for any blockchain
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Wallet className="h-5 w-5 text-blue-500" />
              Multi-Chain Support
            </CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-muted-foreground">
              Support for Ethereum, Bitcoin, and multiple EVM-compatible
              networks including testnets
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Coins className="h-5 w-5 text-blue-500" />
              Token Tracking
            </CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-muted-foreground">
              Track ERC20 token balances with custom thresholds and styling
              options
            </p>
          </CardContent>
        </Card>
      </div>

      <Card className="mt-8">
        <CardHeader>
          <CardTitle>How It Works</CardTitle>
          <CardDescription>
            Get started with Badgify in three simple steps
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          <div className="space-y-2">
            <h3 className="font-semibold">1. Choose Your Network</h3>
            <p className="text-muted-foreground">
              Select from a wide range of supported networks including Ethereum
              Mainnet, Bitcoin, or any EVM-compatible chain
            </p>
          </div>

          <div className="space-y-2">
            <h3 className="font-semibold">2. Enter Wallet Address</h3>
            <p className="text-muted-foreground">
              Input the wallet address you want to track. For ERC20 tokens,
              you'll also need the token contract address
            </p>
          </div>

          <div className="space-y-2">
            <h3 className="font-semibold">3. Customize & Deploy</h3>
            <p className="text-muted-foreground">
              Customize your badge with different styles, colors, and
              thresholds. Copy the generated markdown or HTML code to your
              project
            </p>
          </div>
        </CardContent>
      </Card>

      <Card className="mt-8 bg-blue-50">
        <CardContent className="p-6">
          <div className="flex flex-col md:flex-row items-center justify-between gap-4">
            <div className="space-y-2">
              <h3 className="text-xl font-semibold">
                Ready to create your first badge?
              </h3>
              <p className="text-muted-foreground">
                Start generating dynamic cryptocurrency badges for your projects
              </p>
            </div>
            <Link to="/generator">
              <Button size="lg" className="gap-2">
                Get Started
                <ArrowRight className="h-4 w-4" />
              </Button>
            </Link>
          </div>
        </CardContent>
      </Card>

      <div className="text-center text-sm text-muted-foreground">
        <p>
          Badgify is an open-source project dedicated to making blockchain data
          more accessible and visual
        </p>
      </div>
    </div>
  );
}
