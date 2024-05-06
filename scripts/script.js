import 'dotenv/config'
import { SigningCosmWasmClient, Secp256k1HdWallet } from "cosmwasm"
import * as fs from "fs";
import { Decimal } from "@cosmjs/math";

// This is your rpc endpoint
const rpcEndpoint = "https://testnet-rpc.orai.io"

const mnemonic = process.env.MNEMONIC

async function main() {
    const wallet = await Secp256k1HdWallet.fromMnemonic(mnemonic, { prefix: "orai" })
    const client = await SigningCosmWasmClient.connectWithSigner(
        rpcEndpoint,
        wallet,
        {
            gasPrice: {
                denom: "orai",
                //minimum fee per gas
                amount: Decimal.fromUserInput("0.001", 6)
            }
        }
    );
    const account = await wallet.getAccounts()
    const address = account[0].address
    // get orai balance
    console.log(await client.getBalance(address, "orai"))

    // địa chỉ ví contract sau khi đã deploy
    const contract_address = process.env.CONTRACT_ADDRESS

    const fee = "auto"
    //=====================================DEPLOY========================================

    // //wasm -> wasmCode
    // const path = "./artifacts/cosmwasm-base.wasm"
    // const wasmCode = new Uint8Array(fs.readFileSync(path))

    // //upload code on chain
    // const upload = await client.upload(deploy_address, wasmCode, fee)
    // console.log(upload)

    // //instantiate msg
    // const instantiate_msg = {
    //     owner: address
    // }
    // const res = await client.instantiate(address, upload.codeId, instantiate_msg, "cosmwasm-base", fee)
    // console.log(res)

    //===================================================================================


    //=====================================EXECUTE=======================================

    // const execute_example = await client.execute(
    //     address, contract_address, { example: { a: 5, b: 6 } }, fee)
    // console.log(execute_example)

    //===================================================================================

    //======================================QUERY========================================

    // const query_example = await client.queryContractSmart(
    //     contract_address, {example: {}})
    // console.log(query_example)

    //===================================================================================
}


main();