import { ILauncherConfig } from 'utils';

export class Launcher {
    private config: ILauncherConfig = {
        gameWidth: 800,
        gameHeight: 600,
        gameDpr: 1
    };
    private render: any;
    private mCompleteFunc: Function;
    public static start(config?): Launcher {
        return new this(config);
    }
    constructor(config?: ILauncherConfig) {
        this.mCompleteFunc = this.completeFunc;
        import(/* webpackChunkName: "render" */ "./src/render/render").then((game) => {
            if (!game) {
                // tslint:disable-next-line:no-console
                console.log("no game error");
                return;
            }
            this.render = new game.Render(this.config, this.mCompleteFunc);
        }).catch((error) => {
            // tslint:disable-next-line:no-console
            console.log("import game error", error);
        });
    }
    public registerComplete(func: Function) {
        this.mCompleteFunc = func;
    }
    protected completeFunc() {
        // todo completehandler
    }
}
