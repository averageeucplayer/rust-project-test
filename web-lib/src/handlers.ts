import { invoke } from '@tauri-apps/api/core';
import { AppRecord } from './types';

export const getRecords = async () => await invoke<AppRecord[]>("get_records");