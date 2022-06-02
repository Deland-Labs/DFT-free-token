import { DataTable, Given, Then, When } from "@cucumber/cucumber";
import { assert_remote_result, createActor } from "./utils";
import { Principal } from "@dfinity/principal";
import { canister, identity } from "@deland-labs/ic-dev-kit";
import logger from "node-color-log";
import { CanisterReinstallOptions, FreeTokenInitOptions, reinstall_all } from "../../src/tasks";
import { createFreeTokenActor } from "~/declarations";
import { expect } from "chai";

Given(/^Reinstall freeToken canisters$/, async function (dataTable) {
    const targetTable = dataTable.hashes();

    const users: string[] = targetTable?.map((x) => identity.identityFactory.getPrincipal(x.user)) ?? [];
    const freeTokenInitOptions: FreeTokenInitOptions = {
        mintable: canister.get_id("token_mintable"),
        amount: BigInt(100),
        unlimitedUsers: [...new Set(users)],
    };

    const reinstallOptions: CanisterReinstallOptions = {
        build: false,
        init: false,
        one_by_one: false,
        canisters: {
            free_token: freeTokenInitOptions
                ? {
                      reinstall: true,
                      initOptions: freeTokenInitOptions,
                  }
                : undefined,
        },
    };
    await reinstall_all(reinstallOptions);
});

Given(/^mintable "([^"]*)" add minter "([^"]*)"$/, async function (minter, freeToken) {
    const mintActor = createActor(minter, "dft_main");
    const freeTokenPrincipal = Principal.fromText(canister.get_id(freeToken));
    logger.debug(`freeTokenPrincipal: ${freeTokenPrincipal}`);
    const res = await mintActor.addMinter(freeTokenPrincipal, []);

    logger.debug(`add minter result: ${JSON.stringify(res)}`);
});
Then(/^Users receive tokens for free$/, async function (dataTable) {
    const target_table = dataTable.hashes();
    logger.debug(`target_table: ${JSON.stringify(target_table)}`);
    for (const target of target_table) {
        const actor = createFreeTokenActor(target.user);
        const res = await actor!.receive_free_token();
        logger.debug(`freeToken result: ${JSON.stringify(res)}`);

        assert_remote_result(res);
    }
});
