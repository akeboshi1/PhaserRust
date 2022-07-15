import { RPCPeer, Export } from "webworker-rpc";
import { ILauncherConfig } from "utils";

export class MainWorker extends RPCPeer {
    protected launcher_config: ILauncherConfig;
    constructor(peerKey: string) {
        super(peerKey);
    }

    @Export()
    createGame(config: ILauncherConfig) {
        this.launcher_config = config;
        const js = import("../../pkg/wasm");
        js.then(js => {
            js.greet("WebAssembly");
        });
    }
}