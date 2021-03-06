import "./setup";
import {DataTable, Given, Then, When} from "@cucumber/cucumber";
import logger from "node-color-log";
import {createMintableActor} from "~/declarations";
import {createActor} from "./utils";
import {expect} from "chai";
import {unit, utils, identity} from "@deland-labs/ic-dev-kit";

When(/^Owner "([^"]*)" mint to users$/, async function (owner, dataTable) {

    const target_data = dataTable.hashes();
    logger.debug(target_data);
    const mintActor = createMintableActor(owner);
    const decimals = await mintActor.decimals();
    for (const target of target_data) {
        const {user, amount} = target;
        const userId = identity.identityFactory.getPrincipal(user)!.toText();
        const res = await mintActor.mint(userId, unit.parseToOrigin(amount, decimals), []);
        logger.debug(res);
    }
});
Then(/^Check trade history$/, async function (dataTable) {
    const target_data = dataTable.hashes();
    logger.debug(target_data);
    for (const target of target_data) {
        const {user, token, amount} = target;
        const actor = createActor(token);
        const decimals = await actor.decimals();
        const userId = identity.identityFactory.getPrincipal(user)!.toText();
        const res = await actor.blocksByQuery(BigInt(0), BigInt(target_data.length));
        logger.debug(res);
    }
});
Then(/^Check "([^"]*)" mintable translation history$/, async function (token, dataTable) {
    const target_data = dataTable.hashes();

    const actor = createActor(token);
    const decimals = await actor.decimals();
    const validate = target_data.map(({user, amount}) => {

        return {
            userId: utils.principalToAccountID(identity.getPrincipal(user)!),
            amount: unit.parseToOrigin(amount, decimals)
        }
    });

    const res = await actor.blocksByQuery(BigInt(1), BigInt(target_data.length));
    logger.debug(res);

    const transfers = res.blocks.map(block => {
        if ('Transfer' in block.transaction.operation) {
            return {
                userId: block.transaction.operation.Transfer.to,
                amount: block.transaction.operation.Transfer.value
            };
        }
    }).filter(transfer => transfer !== undefined);
    logger.debug(transfers);
    logger.debug(validate);
    expect(transfers.length).to.equal(target_data.length);
    for (const {userId, amount} of validate) {
        const transfer = transfers.find(transfer => transfer?.userId === userId && transfer?.amount === amount);
        expect(transfer).to.not.be.undefined;
    }
});
