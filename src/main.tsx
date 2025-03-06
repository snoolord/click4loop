import React from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import { createTauRPCProxy } from '../bindings'; // Adjust the path if necessary

export const rpc = createTauRPCProxy()
createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
