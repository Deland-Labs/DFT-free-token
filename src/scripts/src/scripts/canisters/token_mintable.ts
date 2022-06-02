import { canister } from "@deland-labs/ic-dev-kit";
import { ReInstallOptions } from "~/utils/canister";
import { DFTInitOptions } from "../../tasks";
import logger from "node-color-log";
import { defaultPVADecimals } from "~/utils/PVADecimals";

const build = () => {
    canister.build("token_mintable");
};

const reinstall_by_dfx = async (args: string) => {
    await canister.reinstall("token_mintable", args);
};

export const reinstall = async (options?: ReInstallOptions, initOption?: DFTInitOptions) => {
    if (options?.build) {
        build();
    }
    const name = initOption?.name ?? "mintable";
    const symbol = initOption?.symbol ?? "mintable";
    const decimals = initOption?.decimals ?? 18;
    // const supply = new BigNumber(parseToCommon(initOption?.totalSupply ?? 1000000000000000000000000n)).toFixed();

    const supply = defaultPVADecimals.toAmount(initOption?.totalSupply.toString() ?? "1000000000000000000000000");

    const fee = initOption?.fee ?? {
        rate: 0n,
        minimum: 0n,
        rate_decimals: 0,
    };
    const archiveArgs =
        initOption?.archive && initOption?.threshold
            ? `opt record { num_blocks_to_archive = ${initOption?.archive} : nat32;
         trigger_threshold = ${initOption?.threshold} :nat32;
         max_message_size_bytes = null;
         cycles_for_archive_creation = null;
         node_max_memory_size_bytes = null;
          }`
            : "null";

    const owner = initOption?.owner ? `opt principal "${initOption?.owner}"` : "null";
    const args = `'(null ,null ,"${name}", "${symbol}", ${decimals}:nat8, ${supply}:nat, record { minimum =${fee.minimum} : nat; rate = ${fee.rate} : nat32; rateDecimals= ${fee.rate_decimals}:nat8 } , ${owner}, ${archiveArgs})'`;
    logger.debug(`Reinstall by dfx: ${args}`);
    await reinstall_by_dfx(args);
};
