import { Domain } from './types'

/**
 * Constructs the EIP-712 domain separator for the trusted forwarder contract.
 *
 * @param forwarderName - The name of the trusted forwarder.
 * @param chainId - The chain ID of the blockchain network.
 * @param forwarderAddress - The address of the trusted forwarder contract.
 * @returns A `Domain` object representing the EIP-712 domain separator.
 */
export function getDomain(forwarderName: string, chainId: bigint, forwarderAddress: string): Domain {
  return {
    name: forwarderName,
    version: '1',
    chainId: chainId,
    verifyingContract: forwarderAddress,
  }
}

/**
 * Defines the structure of the `ForwardRequest` type for EIP-712 signing.
 *
 * The `ForwardRequest` type describes the properties of a transaction relayed
 * through a trusted forwarder, including the sender, recipient, transaction
 * value, gas, nonce, deadline, and payload data.
 */
export const ForwardRequestType = [
  { name: 'from', type: 'address' }, // The address of the transaction sender.
  { name: 'to', type: 'address' }, // The address of the transaction recipient.
  { name: 'value', type: 'uint256' }, // The amount of Ether (in wei) being transferred.
  { name: 'gas', type: 'uint256' }, // The gas limit for the transaction.
  { name: 'nonce', type: 'uint256' }, // The unique nonce to prevent replay attacks.
  { name: 'deadline', type: 'uint48' }, // The deadline timestamp for the transaction.
  { name: 'data', type: 'bytes' }, // The calldata payload of the transaction.
]
