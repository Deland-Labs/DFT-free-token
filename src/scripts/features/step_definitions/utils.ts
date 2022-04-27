import "~/setup";
import {DataTable, Given, Then, When} from "@cucumber/cucumber";
import {reinstall_all} from "../../src/tasks";
import {canister} from "~/utils";
import {
    createFusionActor,
    createMintableActor,
    createWICPActor,
    createWUSDActor,
} from "~/declarations";
import path from "path";
import {existsSync, readFileSync} from "fs";
import {get_id, get_principal} from "~/utils/canister";
import {parseToOrigin} from "~/utils/uint";
import {expect} from "chai";
import logger from "node-color-log";
import * as math from "mathjs";


Then(/^Sleep for "([^"]*)" secs.$/, async function (sec: string) {
    // sleep for secs
    await new Promise(resolve => setTimeout(resolve, parseFloat(sec) * 1000));
});

export const reinstall_canisters = async (names: string[]): Promise<void> => {
    const canisters = {};
    for (const name of names) {
        canisters[name] = true;
    }

    console.info(`Reinstalling canisters: ${JSON.stringify(canisters)}`);

    await reinstall_all({
        build: false,
        init: true,
        canisters: canisters
    });
}


Given(/^Reinstall canisters$/,
    async function (data) {
        console.log(`Reinstalling canisters: ${JSON.stringify(data.rawTable)}`);
        const target_canisters = data.hashes();
        const names: string[] = [];
        for (const item of target_canisters) {
            names.push(item.name);
        }
        await reinstall_canisters(names);
    });
When(/^canister "([^"]*)" is down$/, async function (canister_name: string) {
    console.log(`Canister ${canister_name} is down`);
    await canister.uninstall_code(canister_name);
});

Given(/^Fusion init$/, async function (data) {
    const target_data = data.hashes();

    const amount_token = get_principal(target_data[0].amount_token);
    const volume_token = get_principal(target_data[0].volume_token);
    const actor = createFusionActor();
    const res = await actor.init_es(amount_token, Number(target_data[0].amount_decimals), volume_token, Number(target_data[0].volume_decimals));

    logger.debug(`Fusion init result: ${JSON.stringify(res)}`);
    expect("Ok" in res).be.true;
});


interface ApproveInput {
    token: string;
    owner: string;
    canister: string;
    amount: string;
}


Then(/^approve tokens from owner to canister in table$/,
    async function (dataTable) {
        const items: ApproveInput[] = dataTable.hashes();
        const jobs = items.map(async item => {
            const target_canisterId = get_id(item.canister);
            const actor = createActor(item.token, item.owner);
            const decimals = await actor.decimals();
            const value = parseToOrigin(math.evaluate(item.amount), decimals);
            const res = await actor.approve([], target_canisterId, value, []);
            logger.debug(`approve tokens from owner to spender result: ${JSON.stringify(res)}`);
            //expect("Ok" in res).be.true;
            assert_remote_result(res);
        });
        await Promise.all(jobs);
    });


export const createActor = (token, user?: string) => {
    switch (token) {
        case "token_WICP":
            return createWICPActor(user);
        case "token_WUSD":
            return createWUSDActor(user);
        case "token_mintable":
            return createMintableActor(user);
        default:
            throw new Error(`Stopping use this to create no token canister`);
    }
}

export const WICP_Decimals = 18;
export const WUSD_Decimals = 10;

export const WICP_Init_Amount = 1000000000000000000;
export const WUSD_Init_Volume = 1000000000000000000;


export class PVADecimals {
    public static PRICE_DECIMALS = 32;
    public volume_decimals: number;
    public amount_decimals: number;
    public price_decimals: number;

    constructor() {
        this.volume_decimals = WICP_Decimals;
        this.amount_decimals = WUSD_Decimals;
        this.price_decimals = PVADecimals.PRICE_DECIMALS;
    }

    public toPrice(value: string): bigint {
        return parseToOrigin(value, this.price_decimals);
    }

    public toVolume(value: string): bigint {
        return parseToOrigin(value, this.volume_decimals);
    }

    public toAmount(value: string): bigint {
        return parseToOrigin(value, this.amount_decimals);
    }

    public setPriceDecimals(value: number) {
        this.price_decimals = value;
    }

    public setVolumeDecimals(value: number) {
        this.volume_decimals = value;
    }

    public setAmountDecimals(value: number) {
        this.amount_decimals = value;
    }
}

export const defaultPVADecimals = new PVADecimals();

/**
 * assert the result of the operation
 * @param result
 * @param status null or "Ok" for success.
 */
export const assert_remote_result = (result: any, status?: string) => {
    if (!status || status === 'Ok') {
        expect('Ok' in result).to.be.true;
    } else {
        if ('Err' in result) {
            expect(result.Err.message).to.be.equal(status);
        } else {
            expect.fail(`Expected to be error but found ${JSON.stringify(result)}`);
        }
    }
}
export const assert_remote_result_array_not_empty = (result: any, status?: string) => {
    if(!status || status === 'Ok') {
        expect('Ok' in result).to.be.true;
        expect(result.Ok.length).to.be.greaterThan(0);
    } else {
        if('Err' in result) {
            expect(result.Err.message).to.be.equal(status);
        } else {
            expect.fail(`Expected to be error but found ${JSON.stringify(result)}`);
        }
    }
}
export const assert_remote_result_array_empty = (result: any, status?: string) => {
    if(!status || status === 'Ok') {
        expect('Ok' in result).to.be.true;
        expect(result.Ok.length).to.be.equal(0);
    } else {
        if('Err' in result) {
            expect(result.Err.message).to.be.equal(status);
        } else {
            expect.fail(`Expected to be error but found ${JSON.stringify(result)}`);
        }
    }
}


export const fileToByteArray = (filePath) => {
    const realPath = path.resolve(filePath);
    if (existsSync(filePath)) {
        const buffer = readFileSync(filePath);
        // buffer to Uint8Array
        const byteArray = new Uint8Array(buffer.byteLength);
        for (let i = 0; i < buffer.byteLength; i++) {
            byteArray[i] = buffer[i];
        }
        return byteArray;
    }
    return new Uint8Array();
};

interface RemoteResultAssertInput {
    req_id: string;
    status: string;
}

Then(/^assert remote result status$/,
    function (data: DataTable) {
        const items: RemoteResultAssertInput[] = data.hashes();
        for (const item of items) {
            remoteStatusStore.assertResult(item.req_id, item.status);
        }
    });

class RemoteStatusStore {
    private static instance: RemoteStatusStore;
    private statuses: Map<string, any>;

    private constructor() {
        this.statuses = new Map();
    }

    public static getInstance() {
        if (!RemoteStatusStore.instance) {
            RemoteStatusStore.instance = new RemoteStatusStore();
        }
        return RemoteStatusStore.instance;
    }

    public setResult(req_id: string, result: any) {
        this.statuses.set(req_id, result);
    }

    public assertResult(req_id: string, status: string) {
        if (this.statuses.has(req_id)) {
            const result = this.statuses.get(req_id);
            assert_remote_result(result, status);
        } else {
            expect.fail(`No result found for req_id: ${req_id}`);
        }
    }
}

export const remoteStatusStore = RemoteStatusStore.getInstance();
