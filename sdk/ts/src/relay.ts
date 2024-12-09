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
  trustedForwardAddress: string,
  trustedFowarderName: string,
  chainId: bigint
): Promise<RelayRequest> {
  let trustedForwardContract = tfc(trustedForwardAddress, signer);
  let nonce = await trustedForwardContract.nonces(await signer.getAddress());
  let gas = await estimateGas(to, data, await signer.getAddress(), value, trustedForwardAddress, signer);
  let deadline = BigInt(Math.floor(Date.now() / 1000) + 60 * 20);
  let request: RelayRequestNoSignature = {
    from: await signer.getAddress(),
    to,
    value,
    data,
    gas,
    deadline,
    nonce,
  };
  let domain = await getDomain(trustedFowarderName, chainId, trustedForwardAddress);
  let signature = await signer.signTypedData(domain, { ForwardRequest: ForwardRequestType }, request);

  return { ...request, signature, chain_id: chainId }
}

export async function gaslessRelay(request: RelayRequest, relayUrl: string) {
    let requestSerialized = {
    chain_id: Number(request.chain_id),
    from: request.from,
    to: request.to,
    value: Number(request.value),
    gas: Number(request.gas),
    deadline: Number(request.deadline),
    data: request.data,
    nonce: Number(request.nonce),
    signature: request.signature,
    };
  try {
    const response = await axios.post(relayUrl, requestSerialized);
    return response.data;
  } catch (error) {
    console.error(error);
    return null;
  }
}

// TODO: Under Development
export async function batchedGaslessRelay(requests: RelayRequest[], payableAccout: string, relayUrl: string) {
    let requestSerialized = requests.map((request) => {
        return {
        chain_id: Number(request.chain_id),
        from: request.from,
        to: request.to,
        value: Number(request.value),
        gas: Number(request.gas),
        deadline: Number(request.deadline),
        data: request.data,
        nonce: Number(request.nonce),
        signature: request.signature,
        };
    });
    try {
        const response = await axios.post(relayUrl, { requests: requestSerialized, refund_receiver: payableAccout });
        return response.data;
    } catch (error) {
        console.error(error);
        return null;
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
