import { useState } from "react";
import "./App.css";
import Sidebar from "./components/common/Sidebar";
import { Button } from "primereact/button";
import Funding from "./components/funding/funding";
import UserSetting from "./components/user-settings/user-setting";
import { HashRouter, Routes, Route, Navigate } from "react-router-dom";

const AppRoutes = [
  { path: "/", element: <Navigate to="/funding" replace /> },
  { path: "/funding", element: <Funding /> },
  { path: "/user-settings", element: <UserSetting /> },
];

function AppContent() {
  const [sidebarVisible, setSidebarVisible] = useState(false);

  return (
    <>
      <Sidebar
        visible={sidebarVisible}
        onHide={() => setSidebarVisible(false)}
      />

      <div className="sticky top-0 z-50 bg-base-100 p-4 shadow-sm">
        <Button
          icon="pi pi-bars"
          onClick={() => setSidebarVisible(true)}
          className="p-button-text"
        />
      </div>
      <div className="flex flex-col gap-4 p-4 max-w-7xl mx-auto w-full">
        <div className="flex-grow bg-base-100 shadow-xl rounded-box p-4 w-full">
          <Routes>
            {AppRoutes.map((route, index) => (
              <Route key={index} path={route.path} element={route.element} />
            ))}
          </Routes>
        </div>
      </div>
    </>
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
