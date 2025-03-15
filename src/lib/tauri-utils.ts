import {
	isPermissionGranted,
	requestPermission,
} from "@tauri-apps/plugin-notification";

export const getNotificationPermissions = async () => {
	let permissionGranted = await isPermissionGranted();

	// If not we need to request it
	if (!permissionGranted) {
		const permission = await requestPermission();
		permissionGranted = permission === "granted";
	}
	return permissionGranted;
};
