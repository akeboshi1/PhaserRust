import "phaser";
import {SceneEnum, SceneManager} from "./render/scene/scene.manager";

if (window.Worker) {
    const myWorker = new Worker(new URL('./js/worker.js', import.meta.url));
    // setTimeout(() => {
    //     myWorker.postMessage(["wasm"]);
    //     console.log("setTimeout send to worker");
    // }, 1000); //1秒后发送消息，用于在编译完成后使 worker 收到消息。
    // myWorker.onmessage = (m) => {
    //     const data = m.data;
    //     console.log('data from wasm: ' + data); //拿到 wasm 计算的结果
    // }

    const config = {
        type: Phaser.AUTO,
        parent: "phaser-rust-example",
        scale: {
            mode: Phaser.Scale.NONE,
            width: 800,
            height: 600,
        },
        render: {
            pixelArt: true,
            roundPixels: true,
        },
        dom: {
            createContainer: true
        },
        backgroundColor: "#4488aa",
        // fps: {
        //     target: 60,
        //     forceSetTimeOut: true
        // }
        //pipeline: { "Color": ColorShaderPipeline }
    };
    // @ts-ignore
    const game = new Phaser.Game(config);

    SceneManager.init(game, myWorker);
    console.dir(SceneEnum);
    SceneManager.add(SceneEnum.TestScene);
}