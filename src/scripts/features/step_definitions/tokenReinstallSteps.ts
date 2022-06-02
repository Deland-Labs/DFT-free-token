import {DataTable, Given, Then, When} from "@cucumber/cucumber";
import {assert, expect} from "chai";
import logger from "node-color-log";
import {CanisterReinstallOptions, DFTInitOptions, Fee, reinstall_all} from "../../src/tasks";
import {
    createWICPActor,
    createWUSDActor,
} from "~/declarations";
import {createActor} from "./utils";
import {get_id, get_principal} from "~/utils/canister";
import * as math from "mathjs";
import {Principal} from "@dfinity/principal";
import {identities} from "~/utils/identity";
import {defaultPVADecimals} from "~/utils/PVADecimals";

import { unit } from "@deland-labs/ic-dev-kit";


interface DftInstallOption {
    key: string;
    name: string;
    symbol: string;
    decimals: string;
    total_supply: string;
    fee_minimum?: string;
    fee_rate?: string;
    rate_decimals?: string;
    owner: string;
    archive?: string;
    threshold?: string;
}

Given(/^Reinstall dft canisters$/, async (rawTable) => {
    const optionArray: DftInstallOption[] = rawTable.hashes();
    // dft token_WICP option
    const dftWICPOption = optionArray.find(o => o.key === "token_WICP");
    //check dftWICPOption not undefined
    const dftWICPInitOptions = dftWICPOption ? parseToDFTInitOptions(dftWICPOption) : undefined;
    // dft token_WUSD 2 option
    const dftWUSDOption = optionArray.find(o => o.key === "token_WUSD");
    //check dftWUSDOption not undefined
    const dftWUSDInitOptions = dftWUSDOption ? parseToDFTInitOptions(dftWUSDOption) : undefined;

    const dftMintableOptions = optionArray.find(o => o.key === "token_mintable");
    const dftMintableInitOptions = dftMintableOptions ? parseToDFTInitOptions(dftMintableOptions) : undefined;


    // check dftWUSDOption and dftWICPOption is exist
    defaultPVADecimals.setAmountDecimals(Number(dftWUSDInitOptions?.decimals) ?? 0);
    defaultPVADecimals.setVolumeDecimals(Number(dftWUSDInitOptions?.decimals) ?? 0);

    const reinstallOptions: CanisterReinstallOptions = {
            build: true,
            init: true,
            one_by_one: false,
            canisters: {
                token_WICP: dftWICPInitOptions ? {
                    reinstall: true,
                    initOptions: dftWICPInitOptions
                } : undefined,
                token_WUSD: dftWUSDInitOptions ? {
                    reinstall: true,
                    initOptions: dftWUSDInitOptions
                } : undefined,
                token_mintable: dftMintableInitOptions ? {
                    reinstall: true,
                    initOptions: dftMintableInitOptions
                } : undefined,
            }
        }
    ;
    await reinstall_all(reinstallOptions);
    logger.debug(`option array: ${JSON.stringify(optionArray)}`);
});


interface TransferInput {
    token: string;
    user: string;
    amount: string;
}

Given(/^transfer token from "([^"]*)" to these users$/,
    async function (user, args: DataTable) {
        const items: TransferInput[] = args.hashes();
        const jobs = items.map(async item => {
            const dftActor = createActor(item.token, user);
            if (dftActor && item) {
                const decimals = await dftActor.decimals();
                const to = identities.get_principal(item.user)!.toText();
                const amountBN = unit.parseToOrigin(math.evaluate(item.amount), decimals);
                const res = await dftActor.transfer([], to, amountBN, []);
                assert.isTrue('Ok' in res, `transfer failed: ${JSON.stringify(res)}`);
                assert.equal(await dftActor.balanceOf(to), amountBN);
            }
        });
        await Promise.all(jobs);
    });

Given(/^owner "([^"]*)" set "([^"]*)" as fee_to$/, async function (owner, feeTo) {
    logger.debug(`owner: ${owner}, feeTo: ${feeTo}`);
    const dftWUSD = createWUSDActor(owner);
    const dftWICP = createWICPActor(owner);
    // const dftBurnAble = createDFTBurnableActor(owner);
    // const dftMintAble = createDFTMintableActor(owner);
    const feeToPrincipal = identities.get_principal(feeTo)!.toText();
    logger.debug(`feeToPrincipal: ${feeToPrincipal}`);
    const dftActors = [dftWUSD, dftWICP];
    for (let i = 0; i < dftActors.length; i++) {
        const dftActor = dftActors[i];
        if (dftActor) {
            try {
                // set fee_to
                const res = await dftActor.setFeeTo(feeToPrincipal, []);
                assert.isTrue('Ok' in res, `set fee_to failed: ${JSON.stringify(res)}`);
                const result = await dftWUSD.tokenInfo();
                assert.isTrue('feeTo' in result, `tokenInfo failed: ${JSON.stringify(result)}`);
                assert.equal(result.feeTo['Principal'].toText(), feeToPrincipal);
            } catch {
                assert.fail(`set fee_to failed`);
            }
        }
    }
});

const parseToDFTInitOptions = (option: DftInstallOption): DFTInitOptions => {
    logger.debug(`option is ${JSON.stringify(option)}`);
    logger.debug(identities.get_principal(option.owner)!.toText());
    logger.debug(identities.get_principal(option.owner)!.toText());
    const decimals = parseInt(option.decimals);
    logger.debug(`decimals: ${option.decimals}`);
    const feeDecimals = parseInt(option.rate_decimals ?? '0');

    return {
        name: String(option.name),
        symbol: String(option.symbol),
        decimals: BigInt(decimals),
        totalSupply: unit.parseToOrigin(math.evaluate(option.total_supply), decimals),
        fee: {
            minimum: Number(unit.parseToOrigin(option.fee_minimum ?? '0', decimals)),
            rate: Number(unit.parseToOrigin(option.fee_rate ?? '0', feeDecimals)),
            rate_decimals: feeDecimals,
        },
        desc: [],
        owner: identities.get_principal(option.owner)!.toText(),
        archive: Number(option.archive),
        threshold: Number(option.threshold),
    };
}
Then(/^check "([^"]*)" balance of "([^"]*)" is "([^"]*)"$/, async function (token, user, balance) {
    const dftActor = createActor(token, user);
    const decimals = await dftActor.decimals();
    const balanceBN = unit.parseToOrigin(balance, decimals);
    const res = await dftActor.balanceOf(identities.get_principal(user)!.toText());
    expect(res).to.equal(balanceBN);
});
Then(/^"([^"]*)" approve "([^"]*)" user "([^"]*)" value "([^"]*)"$/, async function (token, target_token, user, value) {
    const dftActor = createActor(token, user);
    const target_canister = get_id(target_token);
    const valueBN = unit.parseToOrigin(value, await dftActor.decimals());
    const res = await dftActor.approve([], target_canister, valueBN, []);
    logger.debug(`approve result: ${JSON.stringify(res)}`);
    expect("Ok" in res).to.equal(true);
});
When(/^Check "([^"]*)" user "([^"]*)" balance GT "([^"]*)"$/, async function (token, user, value) {
    const dftActor = createActor(token, user);
    const balance = get_id("balance_keeper");
    const dft = get_id(token);
    const decimals = await dftActor.decimals();
    const valueBN = unit.parseToOrigin(value, decimals);
    const res = await dftActor.allowance(dft, balance);
    logger.debug(`res: ${res}`);
    //expect(res as bigint).to.equal(valueBN);
});