/**
 * Represents a relay request including all parameters required for relayed transactions.
 */
export interface RelayRequest {
  chain_id: bigint // The ID of the blockchain network (e.g., 1 for Ethereum mainnet).
  from: string // The sender's address in hexadecimal format.
  to: string // The recipient's address in hexadecimal format.
  value: bigint // The transaction value in Wei (1 Ether = 10^18 Wei).
  gas: bigint // The gas limit for the transaction.
  deadline: bigint // The UNIX timestamp by which the transaction must be executed.
  data: string // Encoded transaction data (e.g., a function call or payload).
  nonce: bigint // Unique nonce for preventing replay attacks.
  signature: string // The digital signature of the request, signed by the sender.
  access_key: string // The access key for the relay server.
}

/**
 * Represents a relay request without a signature, used for preparing transactions
 * before signing.
 */
export interface RelayRequestNoSignature {
  from: string // The sender's address in hexadecimal format.
  to: string // The recipient's address in hexadecimal format.
  value: bigint // The transaction value in Wei (1 Ether = 10^18 Wei).
  data: string // Encoded transaction data (e.g., a function call or payload).
  gas: bigint // The gas limit for the transaction.
  deadline: bigint // The UNIX timestamp by which the transaction must be executed.
  nonce: bigint // Unique nonce for preventing replay attacks.
}

/**
 * Represents the EIP-712 domain used for signing typed data.
 */
export interface Domain {
  name: string // The name of the contract being interacted with.
  version: string // The version of the contract.
  chainId: bigint // The ID of the blockchain network (e.g., 1 for Ethereum mainnet).
  verifyingContract: string // The address of the contract being used for verification.
}

/**
 * Represents a serialized relay request where numeric values are converted
 * to the `number` type, making it suitable for JSON serialization and API requests.
 */
export interface RelayRequestSerialized {
  chain_id: number // The ID of the blockchain network (e.g., 1 for Ethereum mainnet).
  from: string // The sender's address in hexadecimal format.
  to: string // The recipient's address in hexadecimal format.
  value: number // The transaction value in Wei (1 Ether = 10^18 Wei).
  gas: number // The gas limit for the transaction.
  deadline: number // The UNIX timestamp by which the transaction must be executed.
  data: string // Encoded transaction data (e.g., a function call or payload).
  nonce: number // Unique nonce for preventing replay attacks.
  signature: string // The digital signature of the request, signed by the sender.
  access_key: string // The access key for the relay server.
}
