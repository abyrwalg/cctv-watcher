import { defaultWindowIcon } from '@tauri-apps/api/app';
import { TrayIcon } from '@tauri-apps/api/tray';
import { getCurrentWindow } from '@tauri-apps/api/window';

async function createTray() {
  try {
    const trayId = 'cctv-watcher-tray';

    const existingTray = await TrayIcon.getById(trayId);
    if (existingTray) {
      console.log('Tray icon already exists', existingTray);
      await existingTray.close();
    }

    const appWindow = getCurrentWindow();

    appWindow.onResized(async () => {
      const minimized = await appWindow.isMinimized();

      if (minimized) {
        await appWindow.hide();
      }
    });

    const options = {
      icon: await defaultWindowIcon(),
      id: trayId,
      action: async (event) => {
        if (event.type === 'Click') {
          await appWindow.unminimize();
          await appWindow.show();
          await appWindow.setFocus();
        }
      },
    };
    await TrayIcon.new(options);
  } catch (error) {
    console.log('Failed to create tray icon:', error);
  }
}

await createTray();
