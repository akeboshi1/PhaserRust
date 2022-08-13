export class TestScene extends Phaser.Scene {
    constructor(config) {
        super(config);
    }

    init(data) {
        this.worker = data.worker;
    }

    preload(){
        this.load.image("test","assets/test.png");
    }

    create() {
        const grap = this.add.graphics(undefined);
        grap.fillStyle(0xffcc00, 1);
        grap.fillCircle(100, 100, 15);

        this.add.image(450,350,"test");

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