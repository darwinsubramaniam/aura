import { useState } from "react";
import "./App.css";
import FiatRampCreateForm from "./components/fiatramp/fiatramp-create";
import FiatRampTable from "./components/fiatramp/fiatramp-table";

function App() {
  const [refreshKey, setRefreshKey] = useState(0);

  const handleRampCreated = () => {
    setRefreshKey(prev => prev + 1);
  };

  return (
    <>
      <div className="flex flex-row gap-4 p-4 max-w-7xl mx-auto w-full">
        <div className="flex-none">
          <FiatRampCreateForm onRampCreated={handleRampCreated} />
        </div>
        <div className="flex-grow bg-base-100 shadow-xl rounded-box p-4">
          <FiatRampTable refreshTrigger={refreshKey} />
        </div>
      </div>
    </>
  );
}

export default App;
