
import { Enum } from '../../utils/enum';
import { TestScene } from './test.scene';

const SceneName = new Enum(
    TestScene = "Scene1",
)

const SceneClassDic = {
    
}

class SceneManager {
    constructor() {

    }
    static init(game, worker) {
        this.game;
        this.worker;
    }

    static add(sceneName, data = null) {
        // 切换不同的scene
        game.scene.add("testScene", TestScene, true, { x: 0, y: 0, worker: this.worker });
    }
}
export { SceneManagergfn, SceneName };