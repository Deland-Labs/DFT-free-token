import {DataTable, Given, Then, When} from "@cucumber/cucumber";
import {assert_remote_result, createActor} from "./utils";
import {Principal} from "@dfinity/principal";
import {get_id, get_principal} from "~/utils/canister";
import logger from "node-color-log";
import {CanisterReinstallOptions, FreeTokenInitOptions, reinstall_all} from "../../src/tasks";
import {createFreeTokenActor, createRegistrarActor} from "~/declarations"
import {assert, expect} from "chai";
import {QuotaType, RewardType} from "~/declarations/free_token/free_token.did";
import {identities} from "~/utils/identity";
import * as math from "mathjs";


Given(/^Reinstall freeToken and registrar canisters$/, async function () {

    const reinstallOptions: CanisterReinstallOptions = {
            build: false,
            init: false,
            one_by_one: false,
            canisters: {
                free_token: true,
                registrar: true,
            }
        }
    ;
    await reinstall_all(reinstallOptions);
});

Given(/^mintable "([^"]*)" add minter "([^"]*)"$/, async function (minter, freeToken) {
    const mintActor = createActor(minter, 'main');
    const freeTokenPrincipal = Principal.fromText(get_id(freeToken));
    logger.debug(`freeTokenPrincipal: ${freeTokenPrincipal}`);
    const res = await mintActor.addMinter(freeTokenPrincipal, []);

    logger.debug(`add minter result: ${JSON.stringify(res)}`);
});
When(/^add reward token "([^"]*)" code "([^"]*)"$/, async function (canister, code, dataTable) {
    const target_table = dataTable.hashes();

    const users: Principal[] = target_table.map(target => identities.get_principal(target.user));
    const actor = createFreeTokenActor('main');

    const dftWICP = Principal.fromText(get_id('token_WICP'));
    const dftMint = Principal.fromText(get_id('token_mintable'));
    const icnaming = Principal.fromText(get_id('registrar'));

    const reward1: RewardType = {
        'TokenTransferRewardPackage': {
            'canister': dftWICP, 'amount': parseToOrigin('1000', 0)
        }
    };
    const reward2: RewardType = {
        'TokenMintRewardPackage': {
            'canister': dftMint, 'amount': parseToOrigin('500', 0)
        }
    };
    const reward3: RewardType = {
        'QuotaRewardPackage': {
            'diff': 5,
            'canister': icnaming,
            'quota_type': {
                'LenGte': 3
            },
        }
    };
    const rewardList: RewardType[] = [reward1, reward2, reward3];


    const res = await actor!.add_reward(code, rewardList, [users]);
    logger.debug(`add reward result: ${JSON.stringify(res)}`);
});
Then(/^Users receive tokens for free code "([^"]*)"$/, async function (code, dataTable) {

    const target_table = dataTable.hashes();
    logger.debug(`target_table: ${JSON.stringify(target_table)}`);
    for (const target of target_table) {
        const actor = createFreeTokenActor(target.user);
        const res = await actor!.receive_free_token(code);
        logger.debug(`freeToken result: ${JSON.stringify(res)}`);

        assert_remote_result(res);
    }
});
Given(/^give blind_box some quotas from "([^"]*)"$/, async function (user, dataTable) {
    const targetTable = dataTable.hashes();
    const admin = identities.get_principal(user);

    const icNamingActor = createRegistrarActor(user);

    for (const target of targetTable) {
        let quota: QuotaType;
        if (target.quota_type === 'LenEq') {
            quota = {LenEq: Number(target.len)};
        } else {
            quota = {LenGte: Number(target.len)};
        }
        const result = await icNamingActor.add_quota(Principal.fromText(get_id('free_token'))!, quota, Number(target.diff));
        if ('Ok' in result) {
            logger.debug(JSON.stringify(result.Ok));
        } else {
            assert(false, JSON.stringify(result.Err));
            assert(false, "add quota failed");
        }
    }

});
Then(/^Users receive tokens for free code "([^"]*)" should failed, message expect "([^"]*)"$/, async function (code, message, dataTable) {
    const target_table = dataTable.hashes();
    logger.debug(`target_table: ${JSON.stringify(target_table)}`);
    for (const target of target_table) {
        const actor = createFreeTokenActor(target.user);
        const res = await actor!.receive_free_token(code);
        logger.debug(`freeToken result: ${JSON.stringify(res)}`);

        if ('Err' in res) {
            expect(res.Err.message).to.equal(message);
        } else {
            assert(false, `expect Err, but got Ok`);
        }
    }
});
Given(/^transfer token from "([^"]*)" to canister$/, async function (admin, dataTable) {
    const targetTable = dataTable.hashes();
    for (const target of targetTable) {
        const dftActor = createActor(target.token, admin);
        const canister = get_id(target.canister);
        const value = parseToOrigin(math.evaluate(target.amount), await dftActor.decimals());
        const res = await dftActor.transfer([], canister, value, []);
        logger.debug(`transfer result: ${JSON.stringify(res)}`);
        expect("Ok" in res).to.equal(true);
    }
});
When(/^add reward token "([^"]*)"$/, async function (canister, dataTable) {
    const target_table = dataTable.hashes();

    for (const target of target_table) {
        const users: Principal[] = [identities.get_principal(target.user)];
        const actor = createFreeTokenActor('main');

        const dftWICP = Principal.fromText(get_id(target.dicp_canister));
        const dftMint = Principal.fromText(get_id(target.mint_canister));
        const icnaming = Principal.fromText(get_id(target.quota_canister));

        const reward1: RewardType = {
            'TokenTransferRewardPackage': {
                'canister': dftWICP, 'amount': parseToOrigin(target.dicp_amount, 0)
            }
        };
        const reward2: RewardType = {
            'TokenMintRewardPackage': {
                'canister': dftMint, 'amount': parseToOrigin(target.mint_amount, 0)
            }
        };
        const reward3: RewardType = {
            'QuotaRewardPackage': {
                'diff': Number(target.diff),
                'canister': icnaming,
                'quota_type': {
                    'LenGte': Number(target.len)
                },
            }
        };
        const rewardList: RewardType[] = [reward1, reward2, reward3];


        const res = await actor!.add_reward(target.code, rewardList, [users]);
        logger.debug(`add reward result: ${JSON.stringify(res)}`);
    }
});