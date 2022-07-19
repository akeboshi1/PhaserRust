import { WORKER_URL } from "structure";

export class TestScene extends Phaser.Scene {
    constructor(config) {
        super(config);
    }
    create() {
        const worker = new Worker(new URL(WORKER_URL, import.meta.url));
        setTimeout(() => {
            worker.postMessage(["wasm"]);
            console.log("setTimeout send to worker");
        });
        // const js = import("../../lib/Rust_wasm.js");
        // js.then(js import { WORKER_URL } from 'structure';
        //     js.greet("WebAssembly");
        // });
        const grap = this.add.graphics(undefined);
        grap.fillStyle(0xffcc00, 1);
        grap.fillCircle(100, 100, 15);
    }
}