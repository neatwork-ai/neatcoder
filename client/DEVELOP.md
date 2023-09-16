While development and testing. For running the extension locally:

Compile Your Extension: Make sure your extension is compiled and ready for packaging. This might include running a build command like `npm run compile` or `yarn compile``, depending on your setup.

Package the Extension: You can use the vsce command-line tool to package your extension into a `.vsix` file. First, you'll need to install it:

```bash
npm install -g vsce
```

Then navigate to your extension's directory and run:
```bash
vsce package
```

This will create a `.zvsix` file in your extension's directory.

Install the Extension Locally: In VS Code, go to the Extensions view by clicking on the Extensions icon in the Activity Bar on the side of the window. Then, click on the ... (More Actions) button at the top of the Extensions view, select `Install from VSIX...`, and choose the `.vsix` file you created.

Reload VS Code: After installing the extension, you may need to reload the window or restart VS Code to activate it.

### Run the MockServer

`cd server-test/`

// transpile
`tsc mockServer.ts`

// run server
`node mockServer.js`

// to kill the server if not correctly shutdown
`lsof -i :1895`

`kill -9 <PID>`


// To listen in
`sudo tcpdump -i any port 1895`

### TEMP

- specs (OK)
- scaffold (TBD) --> at application start send everything
- jobs (OK) --> at application start send everything
- interfaces --> at application start send everything
- codebase (TBD) --> at application start send everything..

To add to Package JSON:

"menus": {
      "view/title": [
        {
          "command": "extension.stopJob",
          "when": "view == taskPoolView",
          "group": "navigation@2"
        },
        {
          "command": "extension.retryJob",
          "when": "view == taskPoolView",
          "group": "navigation@3"
        },
        {
          "command": "extension.removeTask",
          "when": "view == taskPoolView",
          "group": "navigation@4"
        },
      ],
    }



commands: [{
        "command": "extension.stopJob",
        "title": "Stop Job",
        "icon": "$(primitive-square)"
      },
      {
        "command": "extension.retryJob",
        "title": "Retry Job",
        "icon": "$(refresh)"
      },
      {
        "command": "extension.removeTask",
        "title": "Remove Job",
        "icon": "$(trash)"
      },]



`npm install copy-webpack-plugin --save-dev`
