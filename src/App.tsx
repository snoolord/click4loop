import { useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";

import "./App.css";
import { rpc } from "./main";
import { register, unregister } from "@tauri-apps/plugin-global-shortcut";
import { Button } from "@/components/ui/button";

function App() {
	const [greetMsg, setGreetMsg] = useState("");
	const [name, setName] = useState("");
	const [shouldLoop, setShouldLoop] = useState(false);

	const [isRecording, setIsRecording] = useState(false);

	const [isPlayingBack, setIsPlayingBack] = useState(false);

	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
		const greetMsg = await rpc.greet(name);
		setGreetMsg(greetMsg);
	}

	const startRecording = async () => {
		await rpc.start_mouse_listener();

		await register("Space", async () => {
			console.log("Shortcut triggered");
			alert("unregistering and stopping recording");
			unregister("Space");
			stopRecording();
		});
		setIsRecording(true);
	};
	const stopRecording = async () => {
		await rpc.stop_mouse_listener();
		setIsRecording(false);
	};
	const handleStartStopRecording = () => {
		if (isRecording) {
			stopRecording();
			return;
		}
		startRecording();
	};

	const startPlayback = async () => {
		setIsPlayingBack(true);
		const appWindow = getCurrentWindow();
		appWindow.hide();
		await register("Space", async () => {
			console.log("Shortcut triggered");
			alert("unregistering and stopping playback");
			unregister("Space");
			await rpc.stop_playback();
			setIsPlayingBack(false);
		});
		await rpc.start_playback(shouldLoop);

		setIsPlayingBack(false);
	};

	return (
		<main className="h-full flex flex-col gap-2 items-center justify-center">
			<Button type="button" onClick={handleStartStopRecording}>
				{isRecording ? "Stop Recording" : "Start Recording"}
			</Button>

			<p>{greetMsg}</p>
			<Button type="button" onClick={startPlayback} disabled={isPlayingBack}>
				{isPlayingBack ? "Playing back right now" : "Start Playback"}
			</Button>
			<div className="flex items-center gap-1 justify-center">
				<input
					type="checkbox"
					onChange={() => setShouldLoop(!shouldLoop)}
					checked={shouldLoop}
				/>
				<label htmlFor="loopCheckbox">Loop playback</label>
			</div>
		</main>
	);
}

export default App;
