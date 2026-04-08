import { defaultWindowIcon } from '@tauri-apps/api/app';
import { Menu } from '@tauri-apps/api/menu';
import { TrayIcon } from '@tauri-apps/api/tray';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { exit } from '@tauri-apps/plugin-process';

async function unMinimize(appWindow) {
  try {
    await appWindow.unminimize();
    await appWindow.show();
    await appWindow.setFocus();
  } catch (error) {
    console.log('Error unminimizing window:', error);
  }
}

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

    const menu = await Menu.new({
      items: [
        {
          id: 'show',
          text: 'Show',
          action: () => {
            unMinimize(appWindow);
          },
        },
        {
          id: 'quit',
          text: 'Quit',
          action: () => {
            exit();
          },
        },
      ],
    });

    const options = {
      icon: await defaultWindowIcon(),
      id: trayId,
      action: async (event) => {
        if (
          event.type === 'Click' &&
          event.button === 'Left' &&
          event.buttonState === 'Up'
        ) {
          await unMinimize(appWindow);
        }
      },
      menu,
      menuOnLeftClick: false,
    };
    await TrayIcon.new(options);
  } catch (error) {
    console.log('Failed to create tray icon:', error);
  }
}

await createTray();
