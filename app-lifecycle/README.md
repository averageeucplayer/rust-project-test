
# Getting started

```sh
git clone git@github.com:averageeucplayer/rust-project-test.git
cd rust-project-test/app-lifecycle
cargo make dev # needs elevated pwsh/cmd
```

# 🏗️ Architecture

`cargo make dev` spins up 3 apps

This command spins up three applications simultaneously:
- `tcp-client-server` – A CLI client/server for testing TCP communication.
- `ipc-server` – Captures TCP packets using the specified sniffer (enabled via feature flags) and relays them to named pipes or local sockets.
- `tauri-app` – The frontend interface that connects to the IPC server and processes data.

```
cargo tauri dev
```