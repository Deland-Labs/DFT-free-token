import "~/setup"

import {canister} from "~/utils";
import {addMainAsController} from "~/utils/canister";
import logger from "node-color-log";
import {create_identities, identities} from "~/utils/identity";


create_identities();
// identities.json written to disk
logger.debug("Identities created");

canister.createAll();
addMainAsController()
    .then(() => {
        logger.info("Main controller added");
    })
