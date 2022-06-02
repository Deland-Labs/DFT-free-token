import { unit } from "@deland-labs/ic-dev-kit";

export const WICP_Decimals = 18;
export const WUSD_Decimals = 10;

export const WICP_Init_Amount = 1000000000000000000;
export const WUSD_Init_Volume = 1000000000000000000;

export class PVADecimals {
    public static PRICE_DECIMALS = 32;
    public volume_decimals: number;
    public amount_decimals: number;
    public price_decimals: number;

    constructor() {
        this.volume_decimals = WICP_Decimals;
        this.amount_decimals = WUSD_Decimals;
        this.price_decimals = PVADecimals.PRICE_DECIMALS;
    }

    public toPrice(value: string): bigint {
        return unit.parseToOrigin(value, this.price_decimals);
    }

    public toVolume(value: string): bigint {
        return unit.parseToOrigin(value, this.volume_decimals);
    }

    public toAmount(value: string): bigint {
        return unit.parseToOrigin(value, this.amount_decimals);
    }

    public setPriceDecimals(value: number) {
        this.price_decimals = value;
    }

    public setVolumeDecimals(value: number) {
        this.volume_decimals = value;
    }

    public setAmountDecimals(value: number) {
        this.amount_decimals = value;
    }
}

export const defaultPVADecimals = new PVADecimals();
