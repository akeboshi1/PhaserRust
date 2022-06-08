import { WORKER_PEER } from 'structure';
import { MainWorker } from "./main.worker";

export * from "./main.worker";
const context: MainWorker = new MainWorker(WORKER_PEER);
