import "../assets/index.module.css";

import App from "./app";
import { LogHandler } from "./tabs";

const _global = global /* node */ as any;

const logHandler = new LogHandler();
_global.logit = (st: string) => {
  const id = st.trim().split(" ")[1];
  if (id) {
    logHandler.getByLog(id).writeLine(st);
  }
};

const app = new App();
app.run().catch(console.error);

const toggleBtn = document.getElementById("toggle")!;
const sidePanel = document.getElementById("side_panel")!;

toggleBtn.addEventListener("click", () => {
  sidePanel.classList.toggle("hidden");
});
