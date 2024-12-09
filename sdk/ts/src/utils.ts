export async function getDomain(contract) {
  const { fields, name, version, chainId, verifyingContract, salt, extensions } = await contract.eip712Domain()

  if (extensions.length > 0) {
    throw Error('Extensions not implemented')
  }

  const domain = {
    name,
    version,
    chainId,
    verifyingContract,
    salt,
  }

  for (const [i, { name }] of eip712Types.EIP712Domain.entries()) {
    if (!(fields & (1 << i))) {
      delete domain[name]
    }
  }

  return domain
}

export function domainType(domain) {
  return eip712Types.EIP712Domain.filter(({ name }) => domain[name] !== undefined)
}

const mapValues = (obj, fn) => Object.fromEntries(Object.entries(obj).map(([k, v]) => [k, fn(v)]))
const formatType = (schema) => Object.entries(schema).map(([name, type]) => ({ name, type }))
export const eip712Types = mapValues(
  {
    EIP712Domain: {
      name: 'string',
      version: 'string',
      chainId: 'uint256',
      verifyingContract: 'address',
      salt: 'bytes32',
    },
    Permit: {
      owner: 'address',
      spender: 'address',
      value: 'uint256',
      nonce: 'uint256',
      deadline: 'uint256',
    },
    Ballot: {
      proposalId: 'uint256',
      support: 'uint8',
      voter: 'address',
      nonce: 'uint256',
    },
    ExtendedBallot: {
      proposalId: 'uint256',
      support: 'uint8',
      voter: 'address',
      nonce: 'uint256',
      reason: 'string',
      params: 'bytes',
    },
    OverrideBallot: {
      proposalId: 'uint256',
      support: 'uint8',
      voter: 'address',
      nonce: 'uint256',
      reason: 'string',
    },
    Delegation: {
      delegatee: 'address',
      nonce: 'uint256',
      expiry: 'uint256',
    },
    ForwardRequest: {
      from: 'address',
      to: 'address',
      value: 'uint256',
      gas: 'uint256',
      nonce: 'uint256',
      deadline: 'uint48',
      data: 'bytes',
    },
  },
  formatType
)
