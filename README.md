# DFT-free-token

## Development

Open `src` folder with Visual Studio Code with Remote Dev Tools extension, and load the source code in the container.

everything should be install when container started.

### Build on Windows

If you want to build this project on Windows, please install something below:

#### OpenSSL

install vcpkg https://vcpkg.io/en/getting-started.html

```bash
./vcpkg.exe install openssl-windows:x64-windows
./vcpkg.exe install openssl:x64-windows-static
```

set env:

OPENSSL_DIR="\installed\x64-windows-static"

#### Enable pre-commit hook

Run in repo root:

```bash
pip install pre-commit
pre-commit install
```
