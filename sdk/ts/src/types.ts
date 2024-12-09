export interface IHelloWorld {
  hello_world: () => void
}

export interface RelayRequest {
  chain_id: bigint // Chain ID (e.g., 1 for Ethereum mainnet)
  from: string // Sender's address in hexadecimal format
  to: string // Recipient's address in hexadecimal format
  value: bigint // Transaction value in Wei
  gas: bigint // Gas limit for the transaction
  deadline: bigint // UNIX timestamp for the transaction deadline
  data: string // Encoded transaction data (e.g., a function call)
  nonce: bigint // Nonce for replay protection
  signature: string // Digital signature of the request
}

export interface RelayRequestNoSignature {
  from: string // Sender's address in hexadecimal format
  to: string // Recipient's address in hexadecimal format
  value: bigint // Transaction value in Wei
  data: string // Encoded transaction data (e.g., a function call)
  gas: bigint // Gas limit for the transaction
  deadline: bigint // UNIX timestamp for the transaction deadline
  nonce: bigint // Nonce for replay protection
}

export interface Domain {
  name: string // Name of the contract
  version: string // Contract version
  chainId: bigint // Chain ID (e.g., 1 for Ethereum mainnet)
  verifyingContract: string // Address of the contract
}
