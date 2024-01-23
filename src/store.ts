import { writable } from "svelte/store";

export const refreshContainers = writable(Date.now());
