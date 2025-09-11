import { writable, type Writable } from "svelte/store";

export const data: Writable<Record<string, unknown>> = writable({});