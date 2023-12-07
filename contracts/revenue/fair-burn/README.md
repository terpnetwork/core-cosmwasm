# Terp Fair Burn

The Terp Fair Burn contract is a CosmWasm smart contract deployed on the Terp chain. It is responsible for handlintg fees paid by other contracts. Fees can be paid in multiple denoms. The Fair Burn contract performs the following logic:

- If the funds transferred are in STARS, then a percentage of the funds are burned, and the remaining funds are sent either to the treasury, or a specified recipient address.
- If the funds transferred are not in STARS, then a percentage of the funds are sent to the treasury, and the remaining funds are sent either to the treasury, or a specified recipient address.

## Addresses
