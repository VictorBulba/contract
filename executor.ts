import { readFileSync } from 'fs';
import {get_lcd_config_with_wallet, create_contract, execute_contract, LCDConfig} from './../utils';
import { LCDClient, Wallet } from '@terra-money/terra.js';

interface Config {
	lcd_client: LCDConfig,
	psi_token_initial_owner: string
}

const CONFIG_PATH: string = '/Users/bulba/Downloads/contracts_scripts-master/src/inc/config.json';

const CONTRACT_PATH: string = '/Users/bulba/Downloads/services-contracts-1.4.2/artifacts/inc.wasm';

async function run() {
	const config: Config = JSON.parse(readFileSync(CONFIG_PATH, 'utf-8'))
	const [lcd_client, sender] = await get_lcd_config_with_wallet(config.lcd_client);

    // const contract = await create_contract(lcd_client, sender, "INC", CONTRACT_PATH, {});

    // INC uploaded
    //     code_id: 19
    // INC instantiated
    //     address: terra1q7d3msna9an4eyvhm6ttur8jc3f4z38hsv9mm7

    const contract_addr = 'terra1q7d3msna9an4eyvhm6ttur8jc3f4z38hsv9mm7';

    await inc_counter(lcd_client, sender, contract_addr);

    // await set_counter(lcd_client, sender, contract_addr, 15);

    const current_value = await get_current_value(lcd_client, contract_addr);
    console.log("Current value:", current_value);
}

async function get_current_value(lcd_client: LCDClient, contract_addr: string): Promise<number> {
    const msg = {
        current_counter_value: {}
    };

    let resp = await lcd_client.wasm.contractQuery(contract_addr, msg);

    return resp as number;
}

async function inc_counter(lcd_client: LCDClient, sender: Wallet, contract_addr: string) {
    const msg = {
        inc_counter: {}
    };

    const result = await execute_contract(lcd_client, sender, contract_addr, msg);

    if (result == undefined) {
        console.error("Result is undefined")
    }
}

async function set_counter(lcd_client: LCDClient, sender: Wallet, contract_addr: string, new_value: number) {
    const msg = {
        set_counter: { new_value }
    };

    const result = await execute_contract(lcd_client, sender, contract_addr, msg);

    if (result == undefined) {
        console.error("Result is undefined")
    }
}

run()
    .then(text => {
        console.log(text);
    })
	.catch(err => {
        console.log(err);
    });
