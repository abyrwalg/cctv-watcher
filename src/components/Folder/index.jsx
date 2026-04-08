import React from 'react';

import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

import classes from './styles.module.scss';

export default function Folder({ folderPath, setFolderPaths, isLastOne }) {
  async function selectFolder() {
    try {
      const folder = await open({
        directory: true,
        multiple: false,
      });

      if (!folder) {
        return;
      }

      setFolderPaths((prevFolders) => [
        ...prevFolders,
        { id: Math.random(), path: folder },
        { id: Math.random(), path: null },
      ]);

      await invoke('start_watching_folder', { path: folder });
    } catch (error) {
      console.log('Error selecting folder:', error);
      alert('Failed to select folder. Please try again.');
    }
  }

  async function stopWatching() {
    try {
      await invoke('stop_watching_folder', { path: folderPath.path });
      setFolderPaths((prevFolders) =>
        prevFolders.filter((folder) => folder.id !== folderPath.id)
      );
    } catch (error) {
      console.log('Error stopping folder watch:', error);
      alert('Failed to stop watching folder. Please try again.');
    }
  }

  const { path } = folderPath;

  return (
    <div className={classes.Folder}>
      {path && (
        <div className={classes.path}>
          <p>Watching folder: {path}</p>
          <button className={classes.stopButton} onClick={stopWatching}>
            Stop
          </button>
        </div>
      )}
      {isLastOne && (
        <div className={classes.selectButtonContainer}>
          <button onClick={selectFolder}>Select Folder</button>
        </div>
      )}
    </div>
  );
}
