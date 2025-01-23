import { batchedGaslessRelay, gaslessRelay, prepRequest } from '../src/relay'
import { ethers } from 'ethers'

const RELAYER_TEST_CONTRACT_ADDRESS = "0xF119D4E7A0F6651901d52Db7322e9C89C61D5AF0";
const DATA_FOR_TEST_CONTRACT = "0x82ab890a00000000000000000000000000000000000000000000000000000000000001f7";
const DATA_FOR_TEST_CONTRACT_WITH_ERROR = "0xe736a0940000000000000000000000000000000000000000000000000000000000000017";
const TRUSTED_FOWARDER = "0x2B3673aD2104b8E282F56a9f8a70cb45Ac11E97f";


async function just_relayer() {
    let provider = new ethers.JsonRpcProvider('https://gateway.tenderly.co/public/sepolia');
    let signer = new ethers.Wallet('0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80', provider); // this account those not have to gas payment token

    let prepared_request = await prepRequest(
        signer,
        RELAYER_TEST_CONTRACT_ADDRESS,
        BigInt(0),
        DATA_FOR_TEST_CONTRACT,
        TRUSTED_FOWARDER,
        'ERC2771Forwarder',
        BigInt(11155111),
        '0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356'
      );

      let responds = await gaslessRelay(prepared_request, 'http://127.0.0.1:8010/relay');
      console.log(responds);
}

async function relayer_with_error() {
    let provider = new ethers.JsonRpcProvider('https://gateway.tenderly.co/public/sepolia');
    let signer = new ethers.Wallet('0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80', provider); // this account those not have to gas payment token

    let prepared_request = await prepRequest(
        signer,
        RELAYER_TEST_CONTRACT_ADDRESS,
        BigInt(0),
        DATA_FOR_TEST_CONTRACT_WITH_ERROR,
        TRUSTED_FOWARDER,
        'ERC2771Forwarder',
        BigInt(11155111),
        '0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356'
      );

      let responds = await gaslessRelay(prepared_request, 'http://127.0.0.1:8010/relay');
      console.log(responds);
}

async function main() {
    await just_relayer();
    // await relayer_with_error();
}


main().then().catch(console.error)