import { logger } from "common";
import "../assets/index.module.css";

import App from "./app";
import { LogHandler } from "./tabs";

const logHandler = new LogHandler();

const cb = (st: string) => {
  const id = st.trim().split(" ")[1];
  if (id) {
    logHandler.getByLog(id).writeLine(st);
  }
};
logger.init(cb, cb);
logger.set(true);

const app = new App();
app.run().catch(console.error);

const toggleBtn = document.getElementById("toggle")!;
const sidePanel = document.getElementById("side_panel")!;

toggleBtn.addEventListener("click", () => {
  sidePanel.classList.toggle("hidden");
});
