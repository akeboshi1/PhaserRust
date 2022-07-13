
export class TestScene extends Phaser.Scene {
    constructor(config) {
        super(config);
    }
    create() {
        // const js = import("../../lib/Rust_wasm.js");
        // js.then(js => {
        //     js.greet("WebAssembly");
        // });
        const grap = this.add.graphics(undefined);
        grap.fillStyle(0xffcc00, 1);
        grap.fillCircle(100, 100, 15);
    }
}