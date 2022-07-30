
import { Enum } from '../../utils/enum';
import { TestScene } from './test.scene';
const SceneEnum =  Object.freeze({
        TestScene : "testScene",
});

const SceneClassDic = {
    "testScene":TestScene
}

class SceneManager {
    constructor() {

    }
    static init(game, worker) {
        this.game=game;
        this.worker=worker;
    }

    static add(sceneName, data = null) {
        var cls = SceneClassDic[sceneName];
        // 切换不同的scene
        this.game.scene.add(sceneName, cls, true, { x: 0, y: 0, worker: this.worker });
    }
}
export { SceneManager, SceneEnum };