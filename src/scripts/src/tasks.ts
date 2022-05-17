import "./scripts/setup"
import {reinstall as reinstallWICP} from "~/canisters/token_WICP";
import {reinstall as reinstallWUSD} from "~/canisters/token_WUSD";
import {reinstall as reinstallTokenMintable} from "~/canisters/token_mintable";
import {reinstall as reinstallFreeToken} from "~/canisters/free_token";
import {reinstall as reinstallRegistrar} from "~/canisters/registrar";


export const reinstall_all = async (options?: CanisterReinstallOptions) => {
    // recode time of cost
    const start = Date.now();
    const get_jobs = function* () {
        yield step1();

        function* step1() {
            // dft token_WUSD
            if (options && options.canisters?.token_WICP?.reinstall) {
                yield reinstallWICP({
                    ...options,
                }, options.canisters.token_WICP.initOptions);
            }
            // dft token_WICP
            if (options && options.canisters?.token_WUSD?.reinstall) {
                yield reinstallWUSD({...options,},
                    options.canisters.token_WUSD.initOptions);
            }
            //dft token_mintable
            if (options && options.canisters?.token_mintable?.reinstall) {
                yield  reinstallTokenMintable({...options,}, options.canisters.token_mintable.initOptions);
            }
            // free_token
            if (options && options.canisters?.free_token) {
                yield reinstallFreeToken({
                    ...options,
                });
            }
            if (options && options.canisters?.registrar) {
                yield reinstallRegistrar({
                    ...options,
                });
            }
        }

    };

    if (options && options.one_by_one) {
        for (const job_step of get_jobs()) {
            for (const job of job_step) {
                await job;
            }
        }
    } else {
        console.info("reinstall all in parallel");
        for (const job_step of get_jobs()) {
            await Promise.all(job_step);
        }
    }

    const end = Date.now();
    console.info(`reinstall all in ${end - start} ms`);
    // sleep for 3 seconds to waiting code to be available
    await new Promise((resolve) => setTimeout(resolve, 3000));
}

export interface Fee {
    minimum: number,
    rate: number,
    rate_decimals: number
}

export interface DFTInitOptions {
    name: string;
    symbol: string;
    decimals: bigint;
    totalSupply: bigint;
    fee?: Fee;
    desc?: Array<[string, string]>;
    owner: string;
    archive?: number;
    threshold?: number;
}

export interface FreeTokenInitOptions {
    mintable: string;
    amount: bigint;
    unlimitedUsers: string[];
}

export interface CommonInstallOptions {
    reinstall: boolean;
}

export interface FreeTokenInstallOptions extends CommonInstallOptions {
    initOptions: FreeTokenInitOptions;
}

export interface DFTInstallOptions extends CommonInstallOptions {
    initOptions?: DFTInitOptions;
}

export interface CanisterReinstallOptionsCanisters {
    token_WICP?: DFTInstallOptions;
    token_WUSD?: DFTInstallOptions;
    token_mintable?: DFTInstallOptions;
    free_token?: boolean;
    registrar?: boolean;
}

export interface CanisterReinstallOptions {
    build?: boolean;
    init?: boolean;
    one_by_one?: boolean;
    canisters?: CanisterReinstallOptionsCanisters;
}
