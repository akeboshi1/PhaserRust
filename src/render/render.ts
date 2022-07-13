import { WORKER_PEER, WORKER_NAME, WORKER_URL, RENDER_PEER } from 'structure';
import { ILauncherConfig } from 'utils';
import { RPCPeer } from "webworker-rpc";
import { Game } from 'phaser';
import { TestScene } from './test.scene';

export class Render extends RPCPeer {
    protected launcher_config: ILauncherConfig;
    protected phaser_config: Phaser.Types.Core.GameConfig;
    protected phaser_game: Phaser.Game;
    constructor(config: ILauncherConfig, completeFunc?: Function) {
        super(RENDER_PEER);
        // this.mRenderParam = { key: PICA_RENDER_PEER, url: "", name: PICA_RENDER_NAME };
        // this.mMainPeerParam = { key: PICA_MAIN_WORKER, url: PICA_MAIN_WORKER_URL, name: PICA_MAIN_NAME };
        this.launcher_config = config;
        this.initWorker();
    }

    protected initWorker() {
        // Logger.log("startLink mainpeer", this.mMainPeerParam.key, this.mMainPeerParam.url);
        const key = WORKER_PEER;
        const peerName = WORKER_NAME;
        const url = WORKER_URL;
        this.attach(key, url, true).onceReady(() => {
            // this.mainPeer = this.remote[key][peerName];
            if (!this.mainWorker) {
                console.error("no workerPeer", key, peerName);
                return;
            }
            this.createGame();
            console.debug("worker onReady");
        });
    }

    protected createGame() {
        this.newPhaserGame().then(() => {
            // this.createManager();
            this.mainWorker.createGame(this.launcher_config);
        });
    }

    protected newPhaserGame(): Promise<Phaser.Game> {
        return new Promise((resolve, reject) => {
            if (this.phaser_game) {
                resolve(this.phaser_game);
            }
            this.phaser_config = {
                type: Phaser.AUTO,
                parent: this.launcher_config.parent,
                disableContextMenu: true,
                transparent: false,
                backgroundColor: 0x0,
                fps: {
                    target: 60,
                },
                dom: {
                    createContainer: true,
                },
                plugins: {
                    scene: [
                        // {
                        //     key: "DragonBones",
                        //     plugin: dragonBones.phaser.plugin.DragonBonesScenePlugin,
                        //     mapping: "dragonbone",
                        // }
                    ]
                },
                render: {
                    pixelArt: true,
                    roundPixels: true,
                },
                scale: {
                    mode: Phaser.Scale.NONE,
                    width: this.launcher_config.gameWidth * this.launcher_config.gameDpr,
                    height: this.launcher_config.gameHeight * this.launcher_config.gameDpr,
                    zoom: 1 / this.launcher_config.gameDpr,
                },
            };
            Object.assign(this.phaser_config, this.launcher_config);
            this.phaser_game = new Game(this.phaser_config);
            this.phaser_game.scene.add("testScene", TestScene, true, { x: 0, y: 0 });
            resolve(this.phaser_game);
        })
    }

    get mainWorker() {
        if (!this.remote[WORKER_PEER]) return null;
        const peer = this.remote[WORKER_PEER][WORKER_NAME];
        if (!peer) {
            // throw new Error("can't find main worker");
            return null;
        }
        return peer;
    }
}