import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

import { PrimeReactProvider } from 'primereact/api';
import { NotificationProvider } from './components/common/NotificationProvider';

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <PrimeReactProvider>
      <NotificationProvider>
        <App />
      </NotificationProvider>
    </PrimeReactProvider>
  </React.StrictMode>,
);
