import electron from "electron";
import * as path from "path";

const { app, BrowserWindow } = electron

let mainWindow;
let win;

const mode = process.env.NODE_ENV;

function reloadOnChange(win) {
  // if (mode !== 'development') return { close: () => {} };

  // console.log(win)
  // const chokidar = require('chokidar');//.watch(path.join(__dirname, '**'), { ignoreInitial: true });

  // console.log(watcher)

  // chokidar.watch('./src', {ignoreInitial:true}).on('change', (event, path) => {
  // console.log(event, path);
  // win.reload();
  // });

  // watcher.on('all', () => {
  //   console.log("Change detected, reloading..")
  //   win.reload();
  // });

  // return watcher;
}



function createWindow() {
  // Create the browser window.
  mainWindow = new BrowserWindow({
    height: 600,
    webPreferences: {
      preload: path.join(__dirname, "preload.js"),
    },
    width: 800,
  });


  console.log(mainWindow)

  // and load the index.html of the app.
  mainWindow.loadFile(path.join(__dirname, "./index.html"));
  // mainWindow.loadFile(path.join(__dirname, "../../@gtm-app/dist/index.html"));

  // Open the DevTools.
  mainWindow.webContents.openDevTools();

  // const watcher = 
  reloadOnChange(mainWindow);

  // Emitted when the window is closed.
  mainWindow.on("closed", () => {
    // Dereference the window object, usually you would store windows
    // in an array if your app supports multi windows, this is the time
    // when you should delete the corresponding element.
    mainWindow = null;
    // watcher.close()
  });
}

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.on("ready", createWindow);

// Quit when all windows are closed.
app.on("window-all-closed", () => {
  // On OS X it is common for applications and their menu bar
  // to stay active until the user quits explicitly with Cmd + Q
  if (process.platform !== "darwin") {
    app.quit();
  }
});

app.on("activate", () => {
  // On OS X it"s common to re-create a window in the app when the
  // dock icon is clicked and there are no other windows open.
  if (mainWindow === null) {
    createWindow();
  }
});

// In this file you can include the rest of your app"s specific main process
// code. You can also put them in separate files and require them here.