import "phaser";
import { TestScene } from "./scenes/test.scene";

var config = {
    type: Phaser.AUTO,
    parent: "phaser-example",
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
var game = new Phaser.Game(config);

// 切换不同的scene演示不同的ui组件 
game.scene.add("uiScene", TestScene, true, { x: 0, y: 0 });