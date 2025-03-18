// This file has been generated by Specta. DO NOT EDIT.

import { createTauRPCProxy as createProxy, type InferCommandOutput } from 'taurpc'
type TAURI_CHANNEL<T> = (response: T) => void


const ARGS_MAP = { '':'{"playback_events":[],"greet":["name"],"clear_playback_queue":[],"start_mouse_listener":[],"stop_mouse_listener":[],"start_playback":["loop_playback"],"stop_playback":[]}' }
export type Router = { '': { greet: (name: string) => Promise<string>, 
start_mouse_listener: () => Promise<void>, 
stop_mouse_listener: () => Promise<void>, 
playback_events: () => Promise<void>, 
start_playback: (loopPlayback: boolean) => Promise<void>, 
stop_playback: () => Promise<void>, 
clear_playback_queue: () => Promise<void> } };


export type { InferCommandOutput }
export const createTauRPCProxy = () => createProxy<Router>(ARGS_MAP)
