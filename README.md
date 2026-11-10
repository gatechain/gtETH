# gtETH Solana Smart Contracts

This Anchor project is based on gtBTCStaking and contains only the Solana smart contract source code and build configuration.
It retains the original `gtbtc` naming, business logic, PDA seeds, and program ID.

## Important Notes

- The only code change from gtBTCStaking is the token decimal configuration:
  - gtDOGE: 8 decimals
  - gtXRP: 6 decimals
  - gtBTC: 8 decimals
  - gtETH: 9 decimals
- With no other code changes, this project reuses gtBTC's contract logic and references its existing security audit report.
