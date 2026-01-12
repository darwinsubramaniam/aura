import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

import { NotificationProvider } from './components/common/NotificationProvider';

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <NotificationProvider>
      <App />
    </NotificationProvider>
  </React.StrictMode>,
);
