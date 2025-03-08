import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { getCurrentWindow } from "@tauri-apps/api/window";

import "./App.css";
import { rpc } from "./main";
import { register, unregister } from "@tauri-apps/plugin-global-shortcut";
import { Button } from "@/components/ui/button";

function App() {
	const [greetMsg, setGreetMsg] = useState("");
	const [name, setName] = useState("");

	const [isRecording, setIsRecording] = useState(false);

	const [isPlayingBack, setIsPlayingBack] = useState(false);

	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
		const greetMsg = await rpc.greet(name);
		setGreetMsg(greetMsg);
	}

	const startRecording = async () => {
		await rpc.start_mouse_listener();
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
		await rpc.start_playback(false);

		setIsPlayingBack(false);
	};

	return (
		<main className="container">
			<h1>Welcome to Tauri + React</h1>

			<p>Click on the Tauri, Vite, and React logos to learn more.</p>

			<form
				className="row"
				onSubmit={(e) => {
					e.preventDefault();
					greet();
				}}
			>
				<input
					id="greet-input"
					onChange={(e) => setName(e.currentTarget.value)}
					placeholder="Enter a name..."
				/>
				<button type="submit">Greet</button>
			</form>

			<Button type="button" onClick={handleStartStopRecording}>
				{isRecording ? "Stop Recording" : "Start Recording"}
			</Button>
			<p>{greetMsg}</p>
			<Button type="button" onClick={startPlayback} disabled={isPlayingBack}>
				{isPlayingBack ? "Playing back right now" : "Start Playback"}
			</Button>
		</main>
	);
}

export default App;
