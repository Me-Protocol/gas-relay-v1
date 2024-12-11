import { RelayRequest, RelayRequestNoSignature, RelayRequestSerialized } from './types'
import axios from 'axios'
import { ethers } from 'ethers'
import { trustedForwarderContract as tfc } from './contracts'
import { ForwardRequestType, getDomain } from './utils'

/**
 * Prepares a relay request with the necessary parameters, nonce, gas estimate, and a signed message.
 *
 * @param signer - An ethers.js Signer object for signing transactions.
 * @param to - The recipient address of the transaction.
 * @param value - The amount of Ether to send (in wei).
 * @param data - The data payload of the transaction.
 * @param trustedForwardAddress - Address of the trusted forwarder contract.
 * @param trustedFowarderName - Name of the trusted forwarder.
 * @param chainId - The chain ID of the blockchain network.
 * @returns A Promise resolving to a `RelayRequest` object containing the request details and signature.
 */
export async function prepRequest(
  signer: ethers.Signer,
  to: string,
  value: bigint,
  data: string,
  trustedForwardAddress: string,
  trustedFowarderName: string,
  chainId: bigint,
  access_key: string
): Promise<RelayRequest> {
  const trustedForwardContract = tfc(trustedForwardAddress, signer)
  const nonce = await trustedForwardContract.nonces(await signer.getAddress())
  const gas = await estimateGas(to, data, await signer.getAddress(), value, trustedForwardAddress, signer)
  const deadline = BigInt(Math.floor(Date.now() / 1000) + 60 * 20) // Set deadline to 20 minutes from now.

  const request: RelayRequestNoSignature = {
    from: await signer.getAddress(),
    to,
    value,
    data,
    gas,
    deadline,
    nonce,
  }

  const domain = await getDomain(trustedFowarderName, chainId, trustedForwardAddress)
  const signature = await signer.signTypedData(domain, { ForwardRequest: ForwardRequestType }, request)

  return { ...request, signature, chain_id: chainId, access_key }
}

/**
 * Sends a relay request to a gasless relay server.
 *
 * @param request - A `RelayRequest` object containing transaction details.
 * @param relayUrl - The URL of the relay server.
 * @returns A Promise resolving to the response from the relay server or `null` in case of an error.
 */
export async function gaslessRelay(request: RelayRequest, relayUrl: string) {
  const requestSerialized: RelayRequestSerialized = {
    chain_id: Number(request.chain_id),
    from: request.from,
    to: request.to,
    value: Number(request.value),
    gas: Number(request.gas),
    deadline: Number(request.deadline),
    data: request.data,
    nonce: Number(request.nonce),
    signature: request.signature,
    access_key: request.access_key,
  }

  try {
    const response = await axios.post(relayUrl, requestSerialized)
    return response.data
  } catch (error) {
    console.error(error)
    return null
  }
}

/**
 * Sends multiple relay requests in a single batched transaction.
 *
 * @note This function is under development and may have issues with nonce, signature, and deadline synchronization.
 * @note This batch funtion still has a problem, if one of the requests fails, it would be skipped and no useful error message would be returned.
 * @param requests - An array of `RelayRequest` objects to be sent.
 * @param payableAccount - Address to receive refunds from the relay server.
 * @param relayUrl - The URL of the relay server.
 * @returns A Promise resolving to the response from the relay server or `null` in case of an error.
 */
export async function batchedGaslessRelay(requests: RelayRequest[], payableAccount: string, relayUrl: string) {
  const requestSerialized = requests.map((request) => ({
    chain_id: Number(request.chain_id),
    from: request.from,
    to: request.to,
    value: Number(request.value),
    gas: Number(request.gas),
    deadline: Number(request.deadline),
    data: request.data,
    nonce: Number(request.nonce),
    signature: request.signature,
  }))

  try {
    const response = await axios.post(relayUrl, { requests: requestSerialized, refund_receiver: payableAccount })
    return response.data
  } catch (error) {
    console.error(error)
    return null
  }
}

/**
 * Estimates the gas required for a transaction relayed through a trusted forwarder.
 *
 * @param to - The recipient address of the transaction.
 * @param data - The data payload of the transaction.
 * @param from - The sender's address.
 * @param value - The amount of Ether to send (in wei).
 * @param trustedFowarder - The trusted forwarder's address.
 * @param provider - An ethers.js Signer object for gas estimation.
 * @returns A Promise resolving to the estimated gas as a bigint.
 */
export async function estimateGas(
  to: string,
  data: string,
  from: string,
  value: bigint,
  trustedFowarder: string,
  provider: ethers.Signer
): Promise<bigint> {
  const gas = await provider.provider!.estimateGas({
    from: trustedFowarder,
    to,
    data: ethers.solidityPacked(['bytes', 'address'], [data, from]),
    value,
  })

  return gas
}
