import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface UnifiedWindow {
  id: string;
  name: string;
  app_id: string;
  is_hidden: boolean;
}

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [windows, setWindows] = useState<UnifiedWindow[]>([]);
  const [error, setError] = useState<string>("");

  // Check if Tauri API is available
  useEffect(() => {
    if (typeof window !== "undefined" && !(window as any).__TAURI__) {
      setError("⚠️ Tauri API not available. Make sure you're running with 'npm run tauri dev' not 'npm run dev'");
    }
  }, []);

  async function greet() {
    try {
      // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
      setGreetMsg(await invoke("greet", { name }));
    } catch (error) {
      console.error("Error greeting:", error);
      setError(`Error: ${error}`);
    }
  }

  async function fetchWindows() {
    try {
      setError("");
      const result = await invoke<UnifiedWindow[]>("get_all_windows");
      setWindows(result);
      console.log("Windows:", result);
    } catch (error) {
      console.error("Error fetching windows:", error);
      setError(`Error fetching windows: ${error}`);
    }
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>

      {error && (
        <div style={{ 
          marginTop: "1rem", 
          padding: "1rem", 
          backgroundColor: "#ffebee", 
          color: "#c62828",
          borderRadius: "4px"
        }}>
          {error}
        </div>
      )}

      <div className="row" style={{ marginTop: "2rem" }}>
        <button onClick={fetchWindows}>Get All Windows</button>
      </div>

      {windows.length > 0 && (
        <div style={{ marginTop: "1rem" }}>
          <h2>Windows ({windows.length})</h2>
          <ul style={{ textAlign: "left", maxHeight: "400px", overflow: "auto" }}>
            {windows.map((window, index) => (
              <li key={index} style={{ marginBottom: "0.5rem" }}>
                <strong>ID:</strong> {window.id}<br />
                <strong>Name:</strong> {window.name || "(no name)"}<br />
                <strong>App ID:</strong> {window.app_id || "(no app)"}<br />
                <strong>Hidden:</strong> {window.is_hidden ? "Yes" : "No"}
                <hr />
              </li>
            ))}
          </ul>
        </div>
      )}
    </main>
  );
}

export default App;
