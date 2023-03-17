import "./styles.css";
import App from "./App.svelte";
import 'simple-notify/dist/simple-notify.min.css';

const app = new App({
  target: document.getElementById("app"),
});

export default app;
