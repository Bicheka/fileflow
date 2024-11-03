import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
function Downloads() {
  const [pathToDownload, setPathToDownload] = useState("");
  const [error, setError] = useState("");
  const handleDownload = async (e) => {
    e.preventDefault();
    setError("");
    try{
      await invoke("download", {
        pathToDownload: pathToDownload
      });
    }
    catch{
      setError("Could not download file/s");
    }
  }
  return (
    <div>
      <form
        className="shadow-xs rounded bg-white px-1 py-4 xl:p-4"
        onSubmit={handleDownload}
      >
        <label className="mb-2 block text-sm font-bold text-gray-700">
          File path
        </label>
        <input
          type="text"
          value={pathToDownload}
          onChange={(e) => setPathToDownload(e.target.value)}
          className="mb-4 w-full rounded-lg border border-gray-300 px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="Enter file path"
        />
        {error && <p className="mb-4 text-red-500">{error}</p>}
        <button
          type="submit"
          className="w-full rounded-lg bg-blue-500 py-2 font-bold text-white hover:bg-blue-700"
        >
          Download
        </button>
      </form>
    </div>
  );
}

export default Downloads;
