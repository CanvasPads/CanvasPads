import MainWindow from "./editor/MainWindow";
import init, { start } from "./wasm/canvaspads_web";

init().then(() => { start() });

import { render } from "preact";

let wrapper = document.getElementById("app")!;
render(<MainWindow />, wrapper);
