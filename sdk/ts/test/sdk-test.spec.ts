import { prepRequest } from '../src/relay'
import { ethers } from 'ethers'

async function main() {
  let provider = new ethers.JsonRpcProvider('http://127.0.0.1:8545/')
  let signer = new ethers.Wallet('0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80', provider)
  let re = await prepRequest(
    signer,
    '0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512',
    BigInt(0),
    '0x3e6fec04',
    '0x5FbDB2315678afecb367f032d93F642f64180aa3'
  )
  console.log(re)
}

main().then().catch(console.error)
