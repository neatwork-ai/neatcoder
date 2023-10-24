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

## Publish extension

To login: `vsce login NeatworkAi`

Create a token at: `https://dev.azure.com/neatwork-ai/_usersSettings/tokens`
