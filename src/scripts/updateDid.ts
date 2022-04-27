import {exec} from "shelljs";
import {canisters} from "~/canisters";
import fs from "fs";
import logger from "node-color-log";


const download_did = async (canister) => {
    const command = `dfx canister call ${canister} __get_candid_interface_tmp_hack`;
    logger.debug(`download_did : ${command}`);
    const result = exec(command, {silent: true});
    if (result.code !== 0) {
        logger.error(`${canister} : ${result.stderr}`);
        process.exit(1);
    }
    const source_content = result.stdout;
    // substring from first " to last "
    const start = source_content.indexOf("\"") + 1;
    const end = source_content.lastIndexOf("\"");
    let did_content = source_content.substring(start, end);
    // replace \\n with \n
    did_content = did_content.replace(/\\n/g, "\n");
    return did_content;
};

(async () => {

    // for each canister
    canisters
        .map(async ([name, config]) => {
            if(config.pack_config?.exclude_in_package == true) {
                return;
            }
            const did_file = `${config.candid}`;
            logger.debug(` ${name}: did_file: ${did_file}`);
            const did_content = await download_did(name);
            fs.writeFileSync(did_file, did_content);
        });

    logger.info("Did update complete");
})();
