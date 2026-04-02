import React, { useState } from 'react';

import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

import './App.css';

function App() {
  const [folderPath, setFolderPath] = useState('');

  async function selectFolder() {
    const folder = await open({
      directory: true,
      multiple: false,
    });

    setFolderPath(folder);
    invoke('start_watching_folder', { path: folder });
  }

  return (
    <main className="container">
      <h1>CCTV Watcher</h1>

      {folderPath ? (
        <div>
          <p>Watching folder: {folderPath}</p>
        </div>
      ) : (
        <div>
          <button onClick={selectFolder}>Select Folder</button>
        </div>
      )}
    </main>
  );
}

export default App;
