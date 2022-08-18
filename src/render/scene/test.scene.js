export class TestScene extends Phaser.Scene {
    constructor(config) {
        super(config);
    }

    init(data) {
        this.worker = data.worker;
    }

    preload(){
        // this.load.image("test","assets/test.png");
    }

    create() {
        const grap = this.add.graphics(undefined);
        grap.fillStyle(0xffcc00, 1);
        grap.fillCircle(100, 100, 15);

        // this.add.image(450,350,"test");

        setTimeout(() => {
            console.log("setTimeout send to worker");
            this.worker.postMessage(["wasm"]);
        }, 1000); //1秒后发送消息，用于在编译完成后使 worker 收到消息。
        this.worker.onmessage = (m) => {
            const data = m.data;
            console.log('data from wasm: ' + data); //拿到 wasm 计算的结果
            if(m === "test.png"){
                //this.textures.addImage("test.png",);
            }
        }
    }
}