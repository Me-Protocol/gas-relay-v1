import { RelayRequest, RelayRequestNoSignature } from './types'
import axios from 'axios'
import { ethers } from 'ethers'
import { trustedForwarderContract as tfc } from './contracts'
import { ForwardRequestType, getDomain } from './utils'

export async function prepRequest(
  signer: ethers.Signer,
  to: string,
  value: bigint,
  data: string,
  trustedForwardAddress: string
): Promise<RelayRequest> {
  let trustedForwardContract = tfc(trustedForwardAddress, signer)
  let nonce = await trustedForwardContract.nonces(await signer.getAddress())
  let chainId = BigInt(31337)
  let gas = await estimateGas(to, data, await signer.getAddress(), value, trustedForwardAddress, signer)
  let deadline = BigInt(Math.floor(Date.now() / 1000) + 60 * 20)
  let request: RelayRequestNoSignature = {
    from: await signer.getAddress(),
    to,
    value,
    data,
    gas,
    deadline,
    nonce,
  }
  let domain = await getDomain('ERC2771Forwarder', chainId, trustedForwardAddress)
  let signature = await signer.signTypedData(domain, { ForwardRequest: ForwardRequestType }, request)

  return { ...request, signature, chain_id: chainId }
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
  trustedFowarder: string,
  provider: ethers.Signer
): Promise<bigint> {
  let gas = provider.provider!.estimateGas({
    from: trustedFowarder,
    to,
    data: ethers.solidityPacked(['bytes', 'address'], [data, from]),
    value,
  })

  return gas
}
