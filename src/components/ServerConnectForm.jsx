import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { desktopDir } from "@tauri-apps/api/path";
const ServerConnectForm = () => {
  const [serverAddress, setServerAddress] = useState("");
  const [error, setError] = useState("");

  // Function to handle form submission
  const handleConnect = async (e) => {
    e.preventDefault();

    // Basic validation for IP address format
    const ipPattern =
      /^(25[0-5]|2[100-4][0-9]|1[0-9]{2}|[1-9]?[0-9])(\.(25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])){3}$/;
    if (!ipPattern.test(ipAddress)) {
      setError("Invalid IP address format");
      return;
    }
    setError("");
    const desktopPath = await desktopDir();
    try{
      await invoke("start_client", {
        serverAddress: serverAddress,
        localPath: desktopPath
      });
    }
    catch{
      console.log("Could not connect to address: ", ipAddress)
    }
    

    // Perform connection logic (e.g., fetch or WebSocket)
    console.log(`Connecting to server at ${ipAddress}...`);
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
