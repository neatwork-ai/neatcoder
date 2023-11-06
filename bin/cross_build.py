import subprocess
import os
import shutil


def move(src, dest):
    if os.path.isdir(src):
        shutil.move(src, dest)
    else:
        shutil.copy2(src, dest)
        os.remove(src)


def run_command(cmd, working_dir=None):
    print(f"Executing in directory: {working_dir}")
    print(f"Executing command: {cmd}")

    process = subprocess.Popen(
        cmd, shell=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        cwd=working_dir,
        text=True,
        encoding='utf-8'
    )

    # Stream output in real-time
    while True:
        output = process.stdout.readline()
        if output == '' and process.poll() is not None:
            break
        if output:
            print(output.strip())

    rc = process.poll()
    return rc


def build():
    script_dir = os.path.dirname(os.path.realpath(__file__))
    project_root = os.path.dirname(script_dir)

    vsce_dir = os.path.join(project_root, 'vsce')
    crates_dir = os.path.join(project_root, 'crates', 'neatcoder')
    webview_dir = os.path.join(project_root, 'webview')

    # Building WASM
    run_command("wasm-pack build --target nodejs --dev",
                working_dir=crates_dir)

    # Workaround: Copy neatcoder.d.ts to expected location by create_interface.py
    dest_dir = os.path.join(project_root, "pkg")
    # exist_ok=True ensures no error if directory already exists
    os.makedirs(dest_dir, exist_ok=True)

    shutil.copy2(os.path.join(crates_dir, "pkg", "neatcoder.d.ts"),
                 os.path.join(dest_dir, "neatcoder.d.ts"))

    run_command("python3 bin/create_interface.py", working_dir=project_root)

    # Move the output file to desired location
    move(os.path.join(project_root, "neatcoderInterface.d.ts"),
         os.path.join(webview_dir, "wasm", "neatcoderInterface.d.ts"))

    # Clean up by removing the copied file
    # os.remove(os.path.join(project_root, "pkg", "neatcoder.d.ts"))

    # Remove the existing pkg directory in vsce_dir if it exists
    dest_pkg_dir = os.path.join(vsce_dir, "pkg")
    if os.path.exists(dest_pkg_dir):
        shutil.rmtree(dest_pkg_dir)

    # Now move the pkg directory from crates_dir to vsce_dir
    move(os.path.join(crates_dir, "pkg"), dest_pkg_dir)

    # Building WebView
    run_command("npm install", working_dir=webview_dir)
    run_command("npm run build", working_dir=webview_dir)
    dest_webview_build_dir = os.path.join(vsce_dir, "webview", "build")
    if os.path.exists(dest_webview_build_dir):
        shutil.rmtree(dest_webview_build_dir)
    move(os.path.join(webview_dir, "build"), dest_webview_build_dir)

    # Building Client
    run_command("yarn install", working_dir=vsce_dir)
    run_command("vsce package --out ../vsix", working_dir=vsce_dir)


if __name__ == '__main__':
    build()
