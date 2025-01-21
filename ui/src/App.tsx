import { BrowserRouter as Router, Routes, Route, Link, useLocation } from "react-router-dom";
import BadgeGenerator from "@/components/badge-generator";
import IntroPage from "@/components/intro-page";
import { Shield, Plus } from "lucide-react";
import { Button } from "@/components/ui/button";

const NavigationButton = () => {
  const location = useLocation();
  return location.pathname === "/generator" ? (
    <Link to="/" className="text-gray-600 hover:text-gray-900 transition-colors">
      About Badgify
    </Link>
  ) : (
    <Link to="/generator">
      <Button className="bg-gradient-to-r from-pink-100 to-blue-200 hover:from-pink-200 hover:to-blue-300 text-gray-700 border-none">
        <Plus className="mr-2 h-4 w-4" />
        Create Badge
      </Button>
    </Link>
  );
};

function App() {
  return (
    <Router>
      <div className="min-h-screen bg-gradient-to-b from-blue-100 to-white w-full">
        <header className="w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
          <div className="container mx-auto flex h-14 items-center justify-between">
            <div className="flex items-center">
              <Link to="/" className="mr-6 flex items-center space-x-2">
                <Shield className="h-6 w-6 text-blue-500" />
                <span className="font-bold text-xl">Badgify</span>
              </Link>
            </div>
            <div className="flex items-center space-x-4">
              <NavigationButton />
            </div>
          </div>
        </header>

        <main className="container mx-auto py-10">
          <Routes>
            <Route path="/" element={<IntroPage />} />
            <Route path="/generator" element={<BadgeGenerator />} />
          </Routes>
        </main>
      </div>
    </Router>
  );
}

export default App;
