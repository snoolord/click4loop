import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import "./App.css";
import { rpc } from "./main";

function App() {
	const [greetMsg, setGreetMsg] = useState("");
	const [name, setName] = useState("");

	const [isRecording, setIsRecording] = useState(false);

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

			<button type="button" onClick={handleStartStopRecording}>
				{isRecording ? "Stop Recording" : "Start Recording"}
			</button>
			<p>{greetMsg}</p>
		</main>
	);
}

export default App;
