use std::time::Duration;

use ipc_server_lib::IpcClient;
use log::*;
use tokio::time::sleep;

pub fn check_updates(ipc_client: IpcClient) {
    tauri::async_runtime::spawn(async move {
        info!("fake update check");

        let delay = Duration::from_secs(30);

        info!("stopping app");

        sleep(delay).await;

        ipc_client.stop();

        // let status = Command::new("sc").args(["stop", "windivert"]).status();

        // if status.is_ok_and(|status| status.success()) {
        //     info!("stopped driver");
        // } else {
        //     warn!("could not execute command to stop driver");
        // }

        // let status = Command::new("sc").args(["delete", "windivert"]).status();

        // status.expect("unable to delete driver");
    });
}
