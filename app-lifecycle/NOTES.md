
```rust
Command::new("sc").args(["stop", "windivert"]).status();
Command::new("sc").args(["delete", "windivert"]).status();
```

```log
SERVICE_NAME: windivert
        TYPE               : 1  KERNEL_DRIVER
        STATE              : 3  STOP_PENDING
                                (STOPPABLE, NOT_PAUSABLE, IGNORES_SHUTDOWN)
        WIN32_EXIT_CODE    : 0  (0x0)
        SERVICE_EXIT_CODE  : 0  (0x0)
        CHECKPOINT         : 0x0
        WAIT_HINT          : 0x0
[2025-09-10][19:17:55][app_lifycycle_lib::setup][INFO] stopped driver
[SC] DeleteService FAILED 1072:

The specified service has been marked for deletion.
```

After app restart:

```log
Error: The service cannot be started, either because it is disabled or because it has no enabled devices associated with it. (os error 1058)
```

Workaround

```sh
sc.exe create Windivert64 type= kernel binPath= "WinDivert64.sys" start= demand
[SC] CreateService SUCCESS
```