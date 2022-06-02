import {createActor as createWICP} from "~/declarations/token_WICP";
import {createActor as createWUSD} from "~/declarations/token_WUSD";
import {createActor as createMintable} from "~/declarations/token_mintable";
import {createActor as createFreeToken} from "~/declarations/free_token";
import {createActor as createRegistrar} from "~/declarations/registrar";
import logger from "node-color-log";
import {canister, identity} from '@deland-labs/ic-dev-kit'


const createWICPActor = (user?: string) => {
    const canisterId = canister.get_id("token_WICP");
    if (user === undefined) {
        return createWICP(canisterId, {
            agentOptions: {host: identity.identityFactory.getDefaultHost()},
        });
    }
    const identity_info = identity.identityFactory.getIdentity(user)!;
    return createWICP(canisterId, {
        agentOptions: identity_info.agentOptions,
    });
};

// create a dft_basic2 actor
const createWUSDActor = (user?: string) => {
    const canisterId = canister.get_id("token_WUSD");
    if (user === undefined) {
        return createWUSD(canisterId, {
            agentOptions: {host: identity.identityFactory.getDefaultHost()},
        });
    }
    const identity_info = identity.identityFactory.getIdentity(user)!;
    return createWUSD(canisterId, {
        agentOptions: identity_info.agentOptions,
    });
};


const createMintableActor = (user?: string) => {
    const canisterId = canister.get_id("token_mintable");
    if (user === undefined) {
        return createMintable(canisterId, {
            agentOptions: {host: identity.identityFactory.getDefaultHost()},
        });
    }
    const identity_info = identity.identityFactory.getIdentity(user)!;
    return createMintable(canisterId, {
        agentOptions: identity_info.agentOptions,
    });
};

const createFreeTokenActor = (user?: string) => {
    const canisterId = canister.get_id("free_token");
    if (user === undefined) {
        return createFreeToken(canisterId, {
            agentOptions: {host: identity.identityFactory.getDefaultHost()},
        });
    }
    const identity_info = identity.identityFactory.getIdentity(user)!;
    return createFreeToken(canisterId, {
        agentOptions: identity_info.agentOptions,
    });
};
export const createRegistrarActor = (user?: string) => {
    const canisterId = canister.get_id('registrar');
    if (user === undefined) {
        return createRegistrar(canisterId, {
            agentOptions: {host: identity.identityFactory.getDefaultHost()},
        });
    }
    const identity_info = identity.identityFactory.getIdentity(user)!;
    logger.debug(JSON.stringify(identity_info));
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
