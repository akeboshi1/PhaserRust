import "phaser";
import { SceneEnum, SceneManager } from "./render/scene/scene.manager";
import 'phaser/plugins/spine/dist/SpinePlugin';

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
    import("../other/pkg/other").then(other => {
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
            plugins: {
                scene: [
                    { key: 'SpinePlugin', plugin: window.SpinePlugin, mapping: 'spine' }
                ]
            }
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

        console.log(other.OtherFun("hahaha!!!"));
        let foo = new other.Foo(100);
        console.log(foo.test());
    });

    // test add lua
    // const factory = new LuaFactory();
    // await factory.mountFile('./lua/test.lua', 
    // `return print("other lua")
    //  function otherLua()
    //      return print("other functionlua")`);

    //     const lua = await factory.createEngine();
    //     try {
    //         const value = await lua.doString('return require("./lua/test.lua")')
    //     } finally {
    //         lua.global.close()
    //     } 
}

    // {
    //   "mContentBuf": { "type": "Buffer", "data": [] }, 
    //   "mHeader": 
    //   { 
    //     "_head_len": 26, "_len": 0, 
    //     "_buf": { "type": "Buffer", "data": [] }, 
    //     "_offset": 0, 
    //     "_opcode": 1446041, 
    //     "_param": 0, 
    //     "_timestamp": 0, 
    //     "_magic": 21316, 
    //     "_uuid": 0 
    //   }, 
    //   "_opcode_str": "_OP_CLIENT_REQ_VIRTUAL_WORLD_PKT_SYNC_PACKAGE", 
    //   "mContent": { "packageName": "FurniturePackage" } 
    // }