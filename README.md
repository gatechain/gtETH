# GT Staking Token - Upgradeable ERC20 Token

This repository contains a feature-complete upgradeable ERC20 token implementation that supports EIP3009 and EIP2612 standards, with advanced features including blacklisting, pausing, exchange rate management, and more.

## Table of Contents

- [Project Overview](#project-overview)
- [Contract Architecture](#contract-architecture)
- [Key Features](#key-features)
- [Technical Features](#technical-features)
- [Installation and Compilation](#installation-and-compilation)
- [Deployment Guide](#deployment-guide)
- [Usage Instructions](#usage-instructions)
- [Security Features](#security-features)
- [Development Guide](#development-guide)
- [Testing](#testing)

## Project Overview

This is an ERC20 token built on OpenZeppelin's upgradeable contract standards with the following characteristics:

- **Upgradeability**: Uses the UUPS proxy pattern to support contract upgrades
- **Modern Standards**: Implements EIP3009 and EIP2612 for gasless operations
- **Security Features**: Blacklisting, pausing, permission management, etc.
- **Exchange Rate Management**: Supports dynamic exchange rate adjustments
- **Fund Recovery**: Built-in fund rescue functionality
- **Maximum Supply Cap**: Enforces a maximum token supply limit

## Contract Architecture

### Inheritance Structure

```
FiatToken
├── BaseFiatToken
│   ├── AbstractBaseFiatToken
│   └── Blacklistable
├── EIP3009
│   ├── EIP712Domain
│   └── EIP712
├── EIP2612
│   ├── EIP712Domain
│   └── EIP712
└── UUPSUpgradeable
```

### Core Components

- **FiatToken**: Main contract implementing ERC20 with additional features
- **BaseFiatToken**: Base implementation with core token functionality
- **Blacklistable**: Manages address blacklisting for compliance
- **EIP712Domain**: Manages domain separator for typed structured data
- **EIP3009**: Implements gasless transfers with authorization
- **EIP2612**: Implements permit function for gasless approvals

## Key Features

1. **UUPS Upgradeable Pattern**
   - Supports contract upgrades while preserving state
   - Only authorized addresses can perform upgrades

2. **Blacklisting System**
   - Restrict specific addresses from transferring tokens
   - Compliance tool for regulatory requirements

3. **Pausable Functionality**
   - Emergency pause for all token transfers
   - Protection against potential security incidents

4. **Exchange Rate Management**
   - Dynamic exchange rate adjustments
   - Rate change restrictions to prevent manipulation

5. **EIP2612 Permit**
   - Gasless approval mechanism
   - Improves UX by eliminating the need for two transactions

6. **EIP3009 Authorization**
   - Gasless transfers with signed authorizations
   - Includes receiveWithAuthorization and transferWithAuthorization

7. **Fund Recovery**
   - Rescue accidentally sent ERC20 tokens
   - Recover native blockchain currency

8. **Maximum Supply Cap**
   - Enforces a maximum token supply limit (1 billion tokens)
   - Prevents unlimited minting

## Technical Features

- **Solidity Version**: ^0.8.18
- **OpenZeppelin Libraries**: Uses v5.0.1 for security and standardization
- **EIP-712 Typed Data**: Structured data signing for secure off-chain approvals
- **Role-Based Access Control**: Different roles for minting, blacklisting, etc.

## Installation and Compilation

```bash
# Clone the repository
git clone https://github.com/yourusername/gtbtc.git
cd gtbtc

# Install dependencies
npm install

# Compile contracts
npx hardhat compile
```

## Deployment Guide

```bash
# Deploy to local network
npx hardhat run scripts/deploy.js --network localhost

# Deploy to testnet
npx hardhat run scripts/deploy.js --network goerli

# Deploy to mainnet
npx hardhat run scripts/deploy.js --network mainnet
```

The deployment process uses the UUPS proxy pattern to ensure upgradeability.

## Usage Instructions

### Contract Initialization

The token is initialized with the following parameters:

```solidity
function initialize(
    string memory tokenName,
    string memory tokenSymbol,
    string memory tokenCurrency,
    uint8 tokenDecimals,
    address newBlacklister,
    address _rateController,
    address lostAndFound,
    uint256 _exchangeRate,
    uint256 _maxSupply
) external initializer
```

### Key Operations

- **Minting**: Only authorized minters can create new tokens
- **Burning**: Token holders can burn their own tokens
- **Transfers**: Standard ERC20 transfers with blacklist and pause checks
- **Gasless Operations**: Use permit or authorization functions

## Security Features

- **Role Separation**: Different roles for different administrative functions
- **Pausable**: Emergency pause functionality
- **Blacklisting**: Compliance tool for restricting addresses
- **Exchange Rate Protection**: Limits on rate changes to prevent manipulation
- **Upgradeable with Security**: UUPS pattern with authorization checks

## Development Guide

### Project Structure

```
/contracts
  /base           # Base contracts and core functionality
  /util           # Utility contracts and libraries
  *.sol           # Main contract files
/scripts          # Deployment and interaction scripts
/test             # Test suite
```

### Code Style

The project uses Prettier for code formatting:

```bash
npm run format   # Format Solidity files
```


The test suite includes 61 tests covering various aspects of the token functionality, including:

- Basic ERC20 operations
- Blacklist functionality
- Pause functionality
- Exchange rate management
- EIP2612 permit functionality
- EIP3009 authorization functionality
- UUPS upgrade mechanism
- Fund recovery features

```
FiatToken
  ↳ BaseFiatToken
      ↳ AbstractBaseFiatToken
      ↳ PausableUpgradeable
      ↳ Blacklistable
          ↳ OwnableUpgradeable
  ↳ EIP3009
      ↳ EIP712Domain
      ↳ EIP712
  ↳ EIP2612
      ↳ EIP712Domain
      ↳ EIP712
  ↳ UUPSUpgradeable
```

### Core Contracts

| Contract | Description |
|------|----------|
| `FiatToken` | Main contract implementing ERC20 with EIP3009/EIP2612 and rescue functionality |
| `BaseFiatToken` | Base implementation with core token functionality, pausing, and blacklisting |
| `AbstractBaseFiatToken` | Abstract contract defining the core token interface |
| `Blacklistable` | Blacklist management functionality |
| `EIP3009` | Gasless transfer authorization via signed messages |
| `EIP2612` | Gasless approval operations via permit function |
| `EIP712Domain` | Manages domain separator for typed structured data |

## Main Features

### 🔥 Core Functions

#### ERC20 Standard Functions
- `transfer()` - Transfer tokens
- `approve()` - Approve spending
- `transferFrom()` - Transfer from approved amount
- `balanceOf()` - Query balance
- `allowance()` - Query approved amount

#### Minting and Burning
- `mint()` - Mint tokens (minters only)
- `burn()` - Burn tokens (minters only)
- `configureMinter()` - Configure minter
- `removeMinter()` - Remove minter

#### Exchange Rate Management
- `updateExchangeRate()` - Update exchange rate
- `getExchangeRate()` - Get current exchange rate
- `updateRateController()` - Update rate controller

### 🚀 Advanced Features

#### EIP3009 - Gasless Transfers
- `transferWithAuthorization()` - Transfer with authorization
- `receiveWithAuthorization()` - Receive with authorization
- `cancelAuthorization()` - Cancel authorization

#### EIP2612 - Gasless Approvals
- `permit()` - Approve via signature
- `increaseAllowance()` - Increase approval amount
- `decreaseAllowance()` - Decrease approval amount

#### Administrative Functions
- `pause()` / `unpause()` - Pause/unpause contract
- `blacklist()` / `unBlacklist()` - Blacklist management
- `rescueERC20()` - Rescue ERC20 tokens
- `refundETH()` - Rescue ETH

## Technical Features

### Upgradeability
- Uses UUPS proxy pattern
- Supports contract logic upgrades
- Preserves state data

### Security
- Multiple permission controls
- Blacklist mechanism
- Pause functionality
- Exchange rate change limit (20%)

### Gas Optimization
- EIP3009 gasless transfers
- EIP2612 gasless approvals
- Optimized storage layout

## Installation and Compilation

### Requirements
- Node.js 18+
- npm or yarn
- Hardhat

### Install Dependencies
```bash
npm install
```

### Compile Contracts
```bash
npx hardhat compile
```

## Deployment Guide

### 1. Environment Configuration

Create a `.env` file:
```env
PRIVATE_KEY=your_private_key
MAINNET_RPC_URL=your_mainnet_rpc
SEPOLIA_RPC_URL=your_sepolia_rpc
```

```

### 2. Initialization Parameters

The following parameters are required for deployment:
- `tokenName` - Token name
- `tokenSymbol` - Token symbol
- `tokenCurrency` - Currency type
- `tokenDecimals` - Decimal places
- `newBlacklister` - Blacklist administrator
- `rateController` - Rate controller
- `exchangeRate` - Initial exchange rate
- `maxSupply` - Maximum token supply

## Usage Instructions

### Basic Operations

#### Transfers
```javascript
// Standard transfer
await token.transfer(to, amount);

// Approved transfer
await token.approve(spender, amount);
await token.transferFrom(from, to, amount);
```

#### Gasless Operations
```javascript
// EIP2612 approval
const signature = await signPermit(owner, spender, value, deadline);
await token.permit(owner, spender, value, deadline, v, r, s);

// EIP3009 transfer
const auth = await signTransferAuth(from, to, value, validAfter, validBefore, nonce);
await token.transferWithAuthorization(from, to, value, validAfter, validBefore, nonce, v, r, s);
```

### Administrative Operations

#### Minting Management
```javascript
// Configure minter
await token.configureMinter(minter, allowance);

// Mint tokens
await token.mint(to, amount);
```

#### Exchange Rate Management
```javascript
// Update exchange rate
await token.updateExchangeRate(newRate);

// Get current exchange rate
const rate = await token.getExchangeRate();
```

## Security Features

### Role-Based Access Control
- **Owner**: Contract owner, can update key parameters
- **Blacklister**: Blacklist administrator
- **Rate Controller**: Exchange rate controller
- **Minters**: Authorized to mint tokens

### Security Mechanisms
- **Pause Functionality**: Can pause all operations in emergency situations
- **Blacklisting**: Prevents transactions from specific addresses
- **Exchange Rate Limits**: Single rate change cannot exceed 20%
- **Signature Verification**: EIP712 structured data signing

### Fund Security
- **Rescue Functionality**: Can rescue tokens accidentally sent to the contract
- **ETH Recovery**: Can extract ETH from the contract
- **Role Separation**: Different functions managed by different roles

## Development Guide

### Contract Upgrades

1. Modify contract code
2. Compile new version
3. Deploy using upgrade script


## License

MIT License

## Contributing

Issues and Pull Requests are welcome!

## Support

If you have any questions, please create an Issue or contact the development team.
