import React, { useState } from 'react';

import Folder from './components/Folder';

import './App.css';

function App() {
  const [folderPaths, setFolderPaths] = useState([
    {
      id: Math.random(),
      path: null,
    },
  ]);

  return (
    <main className="container">
      <h1>CCTV Watcher</h1>

      {folderPaths.map((path, index) => {
        return (
          <Folder
            key={path.id}
            id={path.id}
            folderPath={path}
            setFolderPaths={setFolderPaths}
            isLastOne={index === folderPaths.length - 1}
          />
        );
      })}
    </main>
  );
}

export default App;
