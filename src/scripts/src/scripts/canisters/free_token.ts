import "../setup"
import { ReInstallOptions} from "~/utils/canister";
import {DFTInitOptions, FreeTokenInitOptions} from "../../tasks";
import logger from "node-color-log";
import {canister} from "@deland-labs/ic-dev-kit"
import {reinstall_with_dev_ids} from "~/canisters/installUtils";

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

    await reinstall_with_dev_ids('free_token');
}
