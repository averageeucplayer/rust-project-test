import { invoke } from '@tauri-apps/api/core';
import { AppRecord } from 'web-core';

export const getRecords = async () => await invoke<AppRecord[]>("get_records"); 