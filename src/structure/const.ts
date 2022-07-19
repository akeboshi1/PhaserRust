export const RENDER_PEER = "renderPeer";
export const WORKER_PEER = "workerPeer";
// webworker_rpc中直接使用的传入路径  支持相对路径
export const WORKER_URL = "./js/worker.ts";
// https://webpack.docschina.org/configuration/optimization/#optimizationconcatenatemodules
// production环境下， webpack concatenateModules默认true。
// 合并模块名导致production和development work name不一致
// export const RENDER_NAME = "render_Render";
// export const NAME = "peer_MainPeer";
export const RENDER_NAME = "Render";
export const WORKER_NAME = "MainWorker";