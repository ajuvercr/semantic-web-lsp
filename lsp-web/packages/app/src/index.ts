import "../assets/index.module.css";

import App from "./app";

const app = new App();
app.run().catch(console.error);

const toggleBtn = document.getElementById("toggle")!;
const sidePanel = document.getElementById("side_panel")!;

toggleBtn.addEventListener("click", () => {
  sidePanel.classList.toggle("hidden");
});
