import {createActor as createWICP} from "~/declarations/token_WICP";
import {createActor as createWUSD} from "~/declarations/token_WUSD";
import {createActor as createMintable} from "~/declarations/token_mintable";
import {createActor as createFreeToken} from "~/declarations/free_token";
import {createActor as createRegistrar} from "~/declarations/registrar";
import {get_id} from "~/utils/canister";
import logger from "node-color-log";
import {identities} from "~/utils/identity";

const createWICPActor = (user?: string) => {
    const canisterId = get_id("token_WICP");
    if (user === undefined) {
        return createWICP(canisterId, {
            agentOptions: {host: identities.main.agentOptions.host},
        });
    }
    const identity_info = identities.get_identity_info(user);
    return createWICP(canisterId, {
        agentOptions: identity_info.agentOptions,
    });
};

// create a dft_basic2 actor
const createWUSDActor = (user?: string) => {
    const canisterId = get_id("token_WUSD");
    if (user === undefined) {
        return createWUSD(canisterId, {
            agentOptions: {host: identities.main.agentOptions.host},
        });
    }
    const identity_info = identities.get_identity_info(user);
    return createWUSD(canisterId, {
        agentOptions: identity_info.agentOptions,
    });
};


const createMintableActor = (user?: string) => {
    const canisterId = get_id("token_mintable");
    if (user === undefined) {
        return createMintable(canisterId, {
            agentOptions: {host: identities.main.agentOptions.host},
        });
    }
    const identity_info = identities.get_identity_info(user);
    return createMintable(canisterId, {
        agentOptions: identity_info.agentOptions,
    });
};

const createFreeTokenActor = (user?: string) => {
    const canisterId = get_id("free_token");
    if (user === undefined) {
        return createFreeToken(canisterId, {
            agentOptions: {host: identities.main.agentOptions.host},
        });
    }
    const identity_info = identities.get_identity_info(user);
    return createFreeToken(canisterId, {
        agentOptions: identity_info.agentOptions,
    });
};
export const createRegistrarActor = (user?: string) => {
    const canisterId = get_id('registrar');
    if (user === undefined) {
        return createRegistrar(canisterId, {
            agentOptions: {host: identities.main.agentOptions.host},
        });
    }
    const identity_info = identities.get_identity_info(user);
    return createRegistrar(canisterId, {
        agentOptions: identity_info.agentOptions,
    });
}


export {
    createWICPActor,
    createWUSDActor,
    createMintableActor,
    createFreeTokenActor,
};
