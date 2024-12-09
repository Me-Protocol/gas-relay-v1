import { Domain } from './types'

export function getDomain(forwarderName: string, chainId: bigint, forwarderAddress: string): Domain {
  return {
    name: forwarderName,
    version: '1',
    chainId: chainId,
    verifyingContract: forwarderAddress,
  }
}

export const ForwardRequestType = [
  { name: 'from', type: 'address' },
  { name: 'to', type: 'address' },
  { name: 'value', type: 'uint256' },
  { name: 'gas', type: 'uint256' },
  { name: 'nonce', type: 'uint256' },
  { name: 'deadline', type: 'uint48' },
  { name: 'data', type: 'bytes' },
]
