import {reinstall_all} from "./src/tasks"
import logger from "node-color-log";
import {exec} from "child_process";

(async () => {
    exec("npx icdev install-canister")
    await reinstall_all({
        build: true,
        init: true,
        canisters: {
            token_WICP: {
                reinstall: false,
            },
            token_WUSD: {
                reinstall: false,
            },
            free_token: true,
        }
    });
})().then(() => {
    logger.info("reinstall_all.ts: All done.");
}).catch((err) => {
    console.error("reinstall_all.ts: Error:", err);
});
