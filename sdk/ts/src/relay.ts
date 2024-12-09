import { RelayRequest, RelayRequestNoSignature } from './types'
import axios from 'axios'
import { ethers } from 'ethers'
import { trustedForwarderContract as tfc } from './contracts'
import { domainType, eip712Types, getDomain } from './utils'

export async function prepRequest(
  signer: ethers.Signer,
  to: string,
  value: bigint,
  data: string,
  rpcUrl: string,
  trustedForwardAddress: string
): Promise<RelayRequest> {
  let trustedForwardContract = tfc(trustedForwardAddress, signer)
  let nonce = await trustedForwardContract.nonces(await signer.getAddress())
  let chainId = 1
  let gas = await estimateGas(to, data, await signer.getAddress(), value, rpcUrl, trustedForwardAddress)
  let deadline = BigInt(Math.floor(Date.now() / 1000) + 60 * 20)
  let request: RelayRequestNoSignature = {
    chain_id: chainId,
    from: await signer.getAddress(),
    to,
    value,
    gas,
    deadline,
    data,
    nonce,
  }
  let domain = await getDomain(trustedForwardContract)
  let signature = await signer.signTypedData(domain, eip712Types.ForwardRequest, request)

  return { ...request, signature }
}

export async function gaslessRelay(request: RelayRequest, relayUrl: string) {
  try {
    const response = await axios.post(relayUrl, request)
    return response.data
  } catch (error) {
    console.error(error)
    return null
  }
}

export async function estimateGas(
  to: string,
  data: string,
  from: string,
  value: bigint,
  rpcUrl: string,
  trustedFowarder: string
): Promise<bigint> {
  const provider = new ethers.JsonRpcProvider(rpcUrl)
  let gas = provider.estimateGas({
    from: trustedFowarder,
    to,
    data: ethers.solidityPacked(['bytes', 'address'], [data, from]),
    value,
  })

  return gas
}
