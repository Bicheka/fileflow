import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { desktopDir } from "@tauri-apps/api/path";
const ServerConnectForm = () => {
  const [serverAddress, setServerAddress] = useState("");
  const [error, setError] = useState("");

  // Function to handle form submission
  const handleConnect = async (e) => {
    e.preventDefault();
    setError("");
    const desktopPath = await desktopDir();
    console.log("local path: ", desktopPath);
    try{
      await invoke("start_client", {
        serverAddress: serverAddress,
        localPath: desktopPath,
      });
    }
    catch{
      setError("Could not start client: ", serverAddress)
    }

    try{
      await invoke("connect");
    }
    catch{
      setError("Could not connect to server")
    }
    

    // Perform connection logic (e.g., fetch or WebSocket)
    console.log(`Connecting to server at ${serverAddress}...`);
    // Insert connection logic here
  };

  return (
    <form
      className="shadow-xs rounded bg-white px-1 py-4 xl:p-4"
      onSubmit={handleConnect}
    >
      <label className="mb-2 block text-sm font-bold text-gray-700">
        Server IP Address
      </label>
      <input
        type="text"
        value={serverAddress}
        onChange={(e) => setServerAddress(e.target.value)}
        className="mb-4 w-full rounded-lg border border-gray-300 px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
        placeholder="Enter IP address"
      />
      {error && <p className="mb-4 text-red-500">{error}</p>}
      <button
        type="submit"
        className="w-full rounded-lg bg-blue-500 py-2 font-bold text-white hover:bg-blue-700"
      >
        Connect
      </button>
    </form>
  );
};

export default ServerConnectForm;
