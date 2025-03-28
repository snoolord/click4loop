// This file has been generated by Specta. DO NOT EDIT.

import { createTauRPCProxy as createProxy, type InferCommandOutput } from 'taurpc'
type TAURI_CHANNEL<T> = (response: T) => void


const ARGS_MAP = { '':'{"start_playback":["loop_playback"],"playback_ended":[],"stop_playback":[],"playback_events":[],"greet":["name"],"start_mouse_listener":[],"stop_mouse_listener":[],"clear_playback_queue":[]}' }
export type Router = { '': { greet: (name: string) => Promise<string>, 
start_mouse_listener: () => Promise<void>, 
stop_mouse_listener: () => Promise<void>, 
playback_events: () => Promise<void>, 
start_playback: (loopPlayback: boolean) => Promise<void>, 
stop_playback: () => Promise<void>, 
clear_playback_queue: () => Promise<void>, 
playback_ended: () => Promise<void> } };


export type { InferCommandOutput }
export const createTauRPCProxy = () => createProxy<Router>(ARGS_MAP)
