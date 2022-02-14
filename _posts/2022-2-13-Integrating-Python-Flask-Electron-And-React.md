---
title: Integrating Python, Flask, Electron, and React
header:
  image: "/images/mountains.jpg"
categories:
  - Electron
  - React
tags:
  - Electron
  - vscode
published: true
---

- [A Flexible Web Architecture](#a-flexible-web-architecture)
  - [Prerequisites](#prerequisites)
  - [Setting up the Repo and Frontend](#setting-up-the-repo-and-frontend)
  - [Creating the Backend](#creating-the-backend)
    - [Flask](#flask)
    - [Installing Dependencies](#installing-dependencies)
    - [Adding a .gitignore](#adding-a-gitignore)
    - [Integrating the backend with NX](#integrating-the-backend-with-nx)
  - [Connecting the Backend](#connecting-the-backend)
    - [Testing the Proxied Backend](#testing-the-proxied-backend)
  - [Serving our Frontend using Electron](#serving-our-frontend-using-electron)
    - [Creating the Electron App](#creating-the-electron-app)
      - [Adding a package.json](#adding-a-packagejson)
      - [Adding the source](#adding-the-source)
      - [Integrating our Electron App with NX](#integrating-our-electron-app-with-nx)
    - [Running the Application](#running-the-application)

# A Flexible Web Architecture

This blog post explains how to setup a [python](https://www.python.org)/[flask](https://flask.palletsprojects.com/en/2.0.x/) web server backend and a [react](https://reactjs.org)/[electron](https://www.electronjs.org)/[typescript](https://www.typescriptlang.org/) frontend all in the form of an [NX](https://nx.dev/) monorepo!

> All of the associated code for this project can be found [here](https://github.com/matthewjberger/integration-example).

## Prerequisites

Ensure that the following dependencies are installed on your system.

* Python 3
* NodeJS
* Yarn

## Setting up the Repo and Frontend

Begin by creating an NX workspace. We'll use the `react` preset because it also scaffolds our web application. For this tutorial we'll use `integration-example` as the workspace name and `myapp-web` as the frontend application name.

```
npx create-nx-workspace --preset=react
```

## Creating the Backend

For our backend project, we'll use `python` and `flask`. We'll manually create this project and integrate it into our NX workspace.

```
code apps/backend/src/app.py
```

For our `app.py`, we can now create a small web server using flask. We'll also add a `/time` route that returns the current time as a json data structure.

```python
# apps/backend/src/app.py
import time
from flask import Flask
app = Flask(__name__)

@app.route('/time')
def get_current_time():
    return {'time': time.time()}

if __name__ == '__main__':
    app.run(host='127.0.0.1', port=5000)
```

### Flask

To configure flask, we'll add a `.flaskenv` file to our `apps/backend` directory.

```bash
FLASK_APP=src/app.py
FLASK_ENV=development
```

### Installing Dependencies

Install the python dependencies we need for the web server to function properly.

```
pip3 install flask python-dotenv
```

> `python-dotenv` is necessary for flask to load the `.flaskenv` file

### Adding a .gitignore

Add the [official python `.gitignore` file](https://github.com/github/gitignore/blob/main/Python.gitignore) to `apps/backend/.gitignore`.

```bash
wget https://raw.githubusercontent.com/github/gitignore/main/Python.gitignore -o apps/backend/.gitignore
```

### Integrating the backend with NX

To integrate our project with NX, we'll need to add a `project.json` file to the `apps/backend` directory. This file declares our project and its targets to NX.

```json
{
  "root": "apps/backend",
  "sourceRoot": "apps/backend/src",
  "projectType": "application",
  "targets": {
    "serve": {
      "executor": "@nrwl/workspace:run-commands",
      "options": {
        "commands": ["flask run --no-debugger"],
        "cwd": "apps/backend"
      }
    }
  },
  "tags": []
}
```

Then, to let NX know we added a `project.json` file we will need to declare the file in our `workspace.json` file at the root of the monorepo, under the `projects` section.

```json
{
  "version": 2,
  "projects": {
    ...
    "backend": "apps/backend"
  }
}
```

Now we can interact with our backend project like any other NX project! To run the backend project you can run `nx run backend:serve`.

## Connecting the Backend

To access the backend from our react application without CORS issues, we'll need to proxy it. This can be done very easily with NX by adding a file named `proxy.conf.json` to the `apps/myapp-web` directory. This contains the address to our local python backend web server.

```json
{
  "/": {
    "target": "http://localhost:5000",
    "secure": true,
    "changeOrigin": true
  }
}
```

Now to let NX know about our proxy when serving up the frontend, we'll modify the frontend's `serve` target in our `apps/myapp-web/project.json`. The field we are adding is called `proxyConfig` and points to our `proxy.conf.json` file.

```json
"serve": {
      "executor": "@nrwl/web:dev-server",
      "options": {
        "buildTarget": "myapp-web:build",
        "hmr": true,
        "proxyConfig": "apps/myapp-web/proxy.conf.json"
      },
      "configurations": {
        "production": {
          "buildTarget": "myapp-web:build:production",
          "hmr": false
        }
      }
    },
```

The frontend will now automatically proxy requests to the backend!

### Testing the Proxied Backend

We can test our backend proxy out by adding the following code to our `apps/myapp-web/src/app/app.tsx`.

```tsx
// apps/myapp-web/src/app/app.tsx
import { useState, useEffect } from "react";

function App() {
  const [currentTime, setCurrentTime] = useState(0);

  // This request is proxied to the backend
  useEffect(() => {
    fetch("/time")
      .then((res) => res.json())
      .then((data) => {
        setCurrentTime(data.time);
      });
  }, []);

  return (
    <div style={{background: "green", color: "white"}}>
        <p>The current date is '{new Date(currentTime).toString()}'.</p>
    </div>
  );
}

export default App;
```

## Serving our Frontend using Electron

Now that we can serve our frontend in the browser, it can easily be served via Electron as a standalone desktop app. Instead of creating a full electron app, we'll create a small electron app that serves up a [BrowserView](https://www.electronjs.org/docs/latest/api/browser-view) and points it to our web application. This enables features specific to electron such as a dedicated Kiosk mode and IPC with the main electron process for invoking node-only functionality.

### Creating the Electron App

The [nx-electron](https://github.com/bennymeg/nx-electron) nx plugin [doesn't support NX 13 yet](https://github.com/bennymeg/nx-electron/issues/120), but that is okay because we can create the project ourselves instead!

Begin by installing electron as a dev dependency.

```bash
yarn add -D electron
```

#### Adding a package.json

Now we can start creating our project structure. We'll start by adding our project's `package.json` to declare the entrypoint for the electron app.

```bash
code apps/myapp-desktop/package.json
```

Then add the following code to the `package.json`.

```json
{
  "name": "myapp-desktop",
  "main": "src/main.js"
}
```

#### Adding the source

To add the source for our electron app we will need two files, `index.html` and `main.js`.

```bash
code apps/myapp-desktop/src/main.js
```

The `main.js` will contain a `BrowserWindow` that hosts a `BrowserView`, pointing to our frontend web application.

```js
const { app, BrowserView, BrowserWindow } = require('electron');

app.whenReady().then(() => {
  const win = new BrowserWindow({ width: 800, height: 600 });

  const view = new BrowserView();
  win.setBrowserView(view);
  view.setBounds({ x: 0, y: 0, width: 800, height: 800 });
  view.setAutoResize({
    horizontal: true,
    vertical: true,
    width: true,
    height: true,
  });
  view.webContents.loadURL('http://127.0.0.1:4200');

  win.maximize();
  win.show();
});
```

Now to serve the `main.js`, we'll add create an `index.html`.

```bash
code apps/myapp-desktop/src/index.html
```

Our `index.html` will contain a minimal web page that loads our electron script.

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8">
    <!-- https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP -->
    <meta http-equiv="Content-Security-Policy" content="default-src 'self'; script-src 'self'">
    <meta http-equiv="X-Content-Security-Policy" content="default-src 'self'; script-src 'self'">
    <title>Hello World!</title>
  </head>
  <body>
    <h1>Hello World!</h1>
    We are using Node.js <span id="node-version"></span>,
    Chromium <span id="chrome-version"></span>,
    and Electron <span id="electron-version"></span>.
  </body>
</html>
```

#### Integrating our Electron App with NX

All that is left is to declare our electron app as an NX project! To achieve this, create a file named `project.json` under `apps/myapp-desktop`.

It will contain the following configuration.

```json
{
  "root": "apps/myapp-desktop",
  "sourceRoot": "apps/myapp-desktop/src",
  "projectType": "application",
  "targets": {
    "serve": {
      "executor": "@nrwl/workspace:run-commands",
      "options": {
        "commands": ["yarn electron apps/myapp-desktop"]
      }
    }
  },
  "tags": []
}
```

Now we have to let NX know about our project by adding the following line to the `workspace.json` file at the root of the monorepo.

```json
{
  "version": 2,
  "projects": {
    ...
    "backend": "apps/backend"
  }
}
```

### Running the Application

By this point, we have created three applications. A python backend web server, a react web application, and an electron app that hosts a `BrowserWindow` containing a `BrowserView`. NX makes it easy to run these applications with one command. We will add a wrapper for the command in the `scripts` section of our `package.json`.

To easily start our backend, frontend, and electron shell, add the following line to the `scripts` section of the root level `package.json`.

```json
"scripts": {
  ...
  "start": "nx run-many --target=serve --projects=myapp-web,myapp-desktop,backend --parallel"
}
```

Finally, we can run `yarn start` to serve all three of our projects! You should see the electron shell appear, running our web application.