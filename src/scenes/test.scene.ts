
export class TestScene extends Phaser.Scene {
    constructor(config) {
        super(config);
    }
    create() {
        const js = import("./../../lib/Rust_wasm.js");
        js.then(js => {
            js.greet("WebAssembly");
        });
    }
}