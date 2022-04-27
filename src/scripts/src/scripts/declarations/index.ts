import {createActor as createWICP} from "~/declarations/token_WICP";
import {createActor as createWUSD} from "~/declarations/token_WUSD";
import {createActor as createMintable} from "~/declarations/token_mintable";
import {createActor as createFreeToken} from "~/declarations/free_token";
import {identityFactory} from "~/utils/identity";
import {get_id} from "~/utils/canister";
import logger from "node-color-log";

const createWICPActor = (user?: string) => {
    const canisterId = get_id("token_WICP");
    if (user === undefined) {
        return createWICP(canisterId, {
            agentOptions: {host: identityFactory.getDefaultHost()},
        });
    }
    const identityInfo = identityFactory.getIdentity(user)!;
    return createWICP(canisterId, {
        agentOptions: identityInfo.agentOptions,
    });
};

// create a dft_basic2 actor
const createWUSDActor = (user?: string) => {
    const canisterId = get_id("token_WUSD");
    if (user === undefined) {
        return createWUSD(canisterId, {
            agentOptions: {host: identityFactory.getDefaultHost()},
        });
    }
    const identityInfo = identityFactory.getIdentity(user)!;
    return createWUSD(canisterId, {
        agentOptions: identityInfo.agentOptions,
    });
};


const createMintableActor = (user?: string) => {
    const canisterId = get_id("token_mintable");
    if (user === undefined) {
        return createMintable(canisterId, {
            agentOptions: {host: identityFactory.getDefaultHost()},
        });
    }
    const identityInfo = identityFactory.getIdentity(user)!;
    return createMintable(canisterId, {
        agentOptions: identityInfo.agentOptions,
    });
};

const createFreeTokenActor = (user?: string) => {
    const canisterId = get_id("free_token");
    if (user === undefined) {
        return createFreeToken(canisterId, {
            agentOptions: {host: identityFactory.getDefaultHost()},
        });
    }
    const identityInfo = identityFactory.getIdentity(user)!;
    return createFreeToken(canisterId, {
        agentOptions: identityInfo.agentOptions,
    });
};


export {
    createWICPActor,
    createWUSDActor,
    createMintableActor,
    createFreeTokenActor,
};
