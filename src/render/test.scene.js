export class TestScene extends Phaser.Scene {
    constructor(config) {
        super(config);
    }

    init(data) {
        this.worker = data.worker;
    }

    create() {
        const grap = this.add.graphics(undefined);
        grap.fillStyle(0xffcc00, 1);
        grap.fillCircle(100, 100, 15);

        setTimeout(() => {
            this.worker.postMessage(["wasm"]);
            console.log("setTimeout send to worker");
        }, 1000); //1秒后发送消息，用于在编译完成后使 worker 收到消息。
        this.worker.onmessage = (m) => {
            const data = m.data;
            console.log('data from wasm: ' + data); //拿到 wasm 计算的结果
        }
    }
}