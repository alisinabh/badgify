import BadgeGenerator from "@/components/badge-generator";
import { Shield } from "lucide-react";

function App() {
  return (
    <div className="min-h-screen bg-gradient-to-b from-blue-100 to-white w-full">
      <header className="w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
        <div className="container mx-auto flex h-14 items-center">
          <div className="mr-4 flex">
            <a className="mr-6 flex items-center space-x-2" href="#">
              <Shield className="h-6 w-6 text-blue-500" />
              <span className="font-bold text-xl">Badgify</span>
            </a>
          </div>
        </div>
      </header>

      <main className="container mx-auto py-10 flex justify-center">
        <BadgeGenerator />
      </main>
    </div>
  );
}

export default App;
