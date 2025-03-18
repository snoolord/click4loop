import { useEffect, useState } from "react";
import { getAllWindows, getCurrentWindow } from "@tauri-apps/api/window";

import "./App.css";
import { rpc } from "./main";
import { register, unregister } from "@tauri-apps/plugin-global-shortcut";
import {
	isPermissionGranted,
	requestPermission,
	sendNotification,
} from "@tauri-apps/plugin-notification";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { confirm, message } from "@tauri-apps/plugin-dialog";
import { hide, show } from "@tauri-apps/api/app";
import { getNotificationPermissions } from "@/lib/tauri-utils";
import {
	checkAccessibilityPermission,
	requestAccessibilityPermission,
} from "tauri-plugin-macos-permissions-api";
import { attachConsole } from "@tauri-apps/plugin-log";

function App() {
	const [greetMsg, setGreetMsg] = useState("");
	const [name, setName] = useState("");
	const [shouldLoop, setShouldLoop] = useState(false);

	const [isRecordingSaved, setIsRecordingSaved] = useState(false);

	const [isPlayingBack, setIsPlayingBack] = useState(false);
	useEffect(() => {
		async function mount() {
			const detach = await attachConsole();
		}
		mount();
	}, []);

	async function greet() {
		// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
		const greetMsg = await rpc.greet(name);
		setGreetMsg(greetMsg);
	}

	const startRecording = async () => {
		let accessibilityGranted = await checkAccessibilityPermission();

		if (!accessibilityGranted) {
			await requestAccessibilityPermission();
			accessibilityGranted = await checkAccessibilityPermission();
		}

		if (!accessibilityGranted) {
			await message(
				"click4loop cannot work without accessibility permissions",
				{
					title: "Click Start Recording again to re-request permissions",
					kind: "error",
				},
			);
			return;
		}
		await rpc.start_mouse_listener();
		const isPermissionGranted = await getNotificationPermissions();

		// Once permission has been granted we can send the notification
		if (isPermissionGranted) {
			sendNotification({
				title: "Recording Started",
				body: "Press Space to stop recording",
			});
		}
		hide();
		await register("Space", async () => {
			await unregister("Space");
			stopRecording();
			if (isPermissionGranted) {
				sendNotification({
					title: "Recording Stopped",
					body: "Press Start Playback to start recording",
				});
			}
			show();
		});
		setIsRecordingSaved(true);
	};
	const stopRecording = async () => {
		await rpc.stop_mouse_listener();
	};
	const handleStartStopRecording = async () => {
		if (!isRecordingSaved) {
			startRecording();
			return;
		}
		const confirmation = await confirm("This cannot be undone.", {
			title: "Discard the previous recording?",
			kind: "warning",
		});
		if (confirmation) {
			await rpc.clear_playback_queue();
			setIsRecordingSaved(false);
		}
		return;
	};

	const startPlayback = async () => {
		setIsPlayingBack(true);
		const startTime = Date.now();

		const isPermissionGranted = await getNotificationPermissions();

		// Once permission has been granted we can send the notification
		if (isPermissionGranted) {
			sendNotification({
				title: "Playback Started",
				body: "Press Space to stop playback",
			});
		}
		hide();
		await register("Space", async () => {
			console.log("Shortcut triggered");

			const endTime = Date.now();
			const elapsedTime = endTime - startTime; // in milliseconds
			const seconds = Math.floor((elapsedTime / 1000) % 60);
			const minutes = Math.floor((elapsedTime / (1000 * 60)) % 60);
			const hours = Math.floor(elapsedTime / (1000 * 60 * 60));

			// Format elapsed time
			const formattedTime = `${hours}h ${minutes}m ${seconds}s`;

			alert(`Playback stopped. Clicked for: ${formattedTime}`);
			await unregister("Space");

			if (isPermissionGranted) {
				sendNotification({
					title: "Playback stopped",
					body: "Press Start Playback to Start Playback or Discard Recording to record a new recording",
				});
			}
			await rpc.stop_playback();
			setIsPlayingBack(false);
			show();
		});
		await rpc.start_playback(shouldLoop);

		setIsPlayingBack(false);
	};

	return (
		<main className="h-full flex flex-col gap-2 items-center justify-center">
			<Button
				type="button"
				onClick={handleStartStopRecording}
				disabled={isPlayingBack}
			>
				{isRecordingSaved ? "Discard Recording" : "Start Recording"}
			</Button>

			<Button type="button" onClick={startPlayback} disabled={isPlayingBack}>
				{isPlayingBack ? "Playing back right now" : "Start Playback"}
			</Button>
			<div className="flex items-center gap-1 justify-center">
				<input
					id="loopCheckbox"
					type="checkbox"
					onChange={() => setShouldLoop(!shouldLoop)}
					checked={shouldLoop}
				/>
				<label
					htmlFor="loopCheckbox"
					className={cn("font-medium cursor-pointer select-none")}
				>
					Loop playback
				</label>
			</div>
			{isPlayingBack && <p>Press Space to stop playing back</p>}
		</main>
	);
}

export default App;
