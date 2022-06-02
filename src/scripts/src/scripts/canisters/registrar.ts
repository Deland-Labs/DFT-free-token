import '../setup'
import {canister} from "@deland-labs/ic-dev-kit"
import {ReInstallOptions} from '~/utils/canister'
import {reinstall_with_dev_ids} from './installUtils'

const build = () => {
    canister.build('registrar')
}

export const reinstall = async (options?: ReInstallOptions) => {
    if (options?.build) {
        build()
    }
    await reinstall_with_dev_ids('registrar')

    if (options?.init) {

    }
}
