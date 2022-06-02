import "../setup"
import {canister} from "../utils";
import {reinstall_code, ReInstallOptions} from "~/utils/canister";
import {DFTInitOptions, FreeTokenInitOptions} from "../../tasks";
import logger from "node-color-log";
import {defaultPVADecimals} from "../../../features/step_definitions/utils";

const build = () => {
    canister.build("free_token");
}

const reinstall_by_dfx = async () => {
    await canister.reinstall("free_token");
}

export const reinstall = async (options?: ReInstallOptions) => {
    if (options?.build) {
        build();
    }
    //await reinstall_by_dfx();

    await reinstall_code('free_token');
}
