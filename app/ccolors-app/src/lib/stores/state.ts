import { writable, readable } from 'svelte/store';

export const totalColors = writable(0);
export const colorsPerPage = readable(16);
export const totalPages = writable(0);