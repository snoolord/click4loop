import React from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import { createTauRPCProxy } from "../bindings"; // Adjust the path if necessary
import { ThemeProvider } from "@/components/theme-provider";

export const rpc = createTauRPCProxy();
createRoot(document.getElementById("root") as HTMLElement).render(
	<React.StrictMode>
		<ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
			<App />
		</ThemeProvider>
	</React.StrictMode>,
);
