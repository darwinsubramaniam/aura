import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [currencies, setCurrencies] = useState([]);
  const [currencySymbol, setCurrencySymbol] = useState("");

  async function get_currencies() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    invoke("get_available_currencies", { symbol: currencySymbol, limit: 50, offset: 0 }).then((currencies: any) => {
      setCurrencies(currencies);
    }).catch((error: any) => {
      console.error(error);
    });
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          get_currencies();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setCurrencySymbol(e.currentTarget.value)}
          placeholder="Enter a currency symbol..."
        />
        <button type="submit">Get Currencies</button>
      </form>
      <p>{JSON.stringify(currencies)}</p>
    </main>
  );
}

export default App;
