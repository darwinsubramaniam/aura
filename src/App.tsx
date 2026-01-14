import { useState } from "react";
import "./App.css";
import Sidebar from "./components/common/Sidebar";
import Funding from "./components/funding/funding";
import UserSetting from "./components/user-settings/user-setting";
import { HashRouter, Routes, Route, Navigate } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { ThemeProvider } from "@/components/theme-provider";
import { ModeToggle } from "@/components/mode-toggle";
import { PanelRightOpen } from 'lucide-react'

const AppRoutes = [
  { path: "/", element: <Navigate to="/funding" replace /> },
  { path: "/funding", element: <Funding /> },
  { path: "/user-settings", element: <UserSetting /> },
];

function AppContent() {
  const [sidebarVisible, setSidebarVisible] = useState(false);

  return (
    <ThemeProvider defaultTheme="dark">
      <Sidebar
        visible={sidebarVisible}
        onHide={() => setSidebarVisible(false)}
      />

      {/* Navbar with macOS traffic light padding */}
      <div 
        className="sticky top-0 z-50 bg-background pt-8 pb-4 px-4 shadow-sm flex items-center gap-2"
        data-tauri-drag-region
      >
        <Button onClick={() => setSidebarVisible(true)} variant="outline">
          <PanelRightOpen />
        </Button>
        <ModeToggle />
      </div>
      <div className="flex flex-col gap-4 p-4 max-w-7xl mx-auto w-full">
        <div className="flex-grow bg-background shadow-xl rounded-lg p-4 w-full">
          <Routes>
            {AppRoutes.map((route, index) => (
              <Route key={index} path={route.path} element={route.element} />
            ))}
          </Routes>
        </div>
      </div>
    </ThemeProvider>
  );
}

function App() {
  return (
    <HashRouter>
      <AppContent />
    </HashRouter>
  );
}

export default App;
