import { canister } from "@deland-labs/ic-dev-kit";
import {ReInstallOptions} from "~/utils/canister";
import {DFTInitOptions, FreeTokenInitOptions} from "../../tasks";
import logger from "node-color-log";

const build = () => {
    canister.build("free_token");
}

const reinstall_by_dfx = async (args: string) => {
    await canister.reinstall("free_token", args);
}

export const reinstall = async (options?: ReInstallOptions, initOption?: FreeTokenInitOptions) => {
    if (options?.build) {
        build();
    }
    const unlimitedUsers = initOption?.unlimitedUsers && initOption?.unlimitedUsers.length > 0 ? `opt vec { ${initOption?.unlimitedUsers.map(u => `principal "${u}"`).join(";")} }` : "null";
    const args = `'(principal "${initOption?.mintable}", ${initOption?.amount}: nat, ${unlimitedUsers})'`;
    logger.debug(`Reinstall by dfx: ${args}`);
    await reinstall_by_dfx(args);
}
