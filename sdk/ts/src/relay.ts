import { RelayRequest } from './types'
import axios from 'axios'
import { ethers } from 'ethers'

export async function prepRequest(signer: ethers.Signer, to: string, value: number, data: string) {}

export async function gaslessRelay(request: RelayRequest, relayUrl: string) {
  try {
    const response = await axios.post(relayUrl, request)
    return response.data
  } catch (error) {
    console.error(error)
    return null
  }
}

export async function estimateGas(request: RelayRequest, rpcUrl: string, trustedFowarder: string): Promise<bigint> {
  const provider = new ethers.JsonRpcProvider(rpcUrl)
  let gas = provider.estimateGas({
    from: trustedFowarder,
    to: request.to,
    data: ethers.solidityPacked(['bytes', 'address'], [request.data, request.from]),
    value: request.value,
    gasLimit: request.gas,
  })

  return gas
}
