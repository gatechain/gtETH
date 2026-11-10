import { HardhatUserConfig } from "hardhat/config";
import '@openzeppelin/hardhat-upgrades';
import * as dotenv from "dotenv";
dotenv.config();
// Provide a fallback private key for development/testing when PRIVATE_KEY is not set
const account = process.env.PRIVATE_KEY || "0x0000000000000000000000000000000000000000000000000000000000000001";

import "@nomicfoundation/hardhat-toolbox";
import "@nomicfoundation/hardhat-verify";
import "@nomicfoundation/hardhat-web3-v4";
import "hardhat-contract-sizer";
const config: HardhatUserConfig = {
  zksolc: {
    version: "latest",
    settings: {
      // Note: This must be true to call NonceHolder & ContractDeployer system contracts
      enableEraVMExtensions: true,
    },
  },
  contractSizer: {
    alphaSort: true,
    disambiguatePaths: false,
    runOnCompile: true,
    strict: true,
    only: [],
  },
  defaultNetwork: "hardhat",
  hardhat: {
    allowUnlimitedContractSize: false,
    chainId: 1 // We set 1337 to make interacting with MetaMask simpler
  },
  etherscan: {
    // Your API key for Etherscan
    // To see the full list of supported networks, run `npx hardhat verify --list-networks`
    apiKey: {
        sepolia: "5GW3HPCM3874B1Q6WJGYFK2V3XBEV94IA5",
    },
},
  networks: {
    base: {
      // url: 'https://rpc.tenderly.co/fork/fd5176f7-2041-443d-93bd-f2f96ae2e5df',
      url:"https://mainnet.base.org",
      accounts:[account]
    },
    bsc: {
      url: "https://bsc-dataseed3.ninicoin.io",
      chainId: 56,
      accounts:[account]
    },
    eth: {
      url: "https://eth.llamarpc.com",
      chainId: 1,
      accounts:[account],
      
    },
    bsctestnet: {
      // url: 'https://rpc.tenderly.co/fork/fd5176f7-2041-443d-93bd-f2f96ae2e5df',
      url:"https://data-seed-prebsc-1-s1.binance.org:8545",
      chainId: 97,
      accounts:[account]
    },
    ethtestnet: {
      url: "https://rpc.sepolia.ethpandaops.io",
      chainId: 11155111,
      accounts:[account],
      
    },
    basetestnet: {
      url: "https://sepolia.base.org",
      chainId: 84532,
      accounts:[account],
      
    },
    abstractTestnet: {
      url: "https://api.testnet.abs.xyz",
      ethNetwork: "sepolia",
      zksync: true,
      verifyURL:
        "https://api-explorer-verify.testnet.abs.xyz/contract_verification",
    },
  },
  solidity: {
    compilers: [
      {
        version: '0.8.28',
        settings: {
          optimizer: {
            enabled: true,
            runs: 200,
          },
          evmVersion:"cancun",
          viaIR: false, // for the reason of :CompilerError: Stack too deep. Try compiling with `--via-ir` (cli) or the equivalent `viaIR: true`
        },
      },{
        version: '0.8.20',
        settings: {
          optimizer: {
            enabled: true,
            runs: 200,
          },
          viaIR: false, // for the reason of :CompilerError: Stack too deep. Try compiling with `--via-ir` (cli) or the equivalent `viaIR: true`
        },
      },
      {
        version: '0.8.18',
        settings: {
          optimizer: {
            enabled: true,
            runs: 200,
          },
          viaIR: false, // for the reason of :CompilerError: Stack too deep. Try compiling with `--via-ir` (cli) or the equivalent `viaIR: true`
        },
      },
      {
        version: '0.8.12',
        settings: {
          optimizer: {
            enabled: true,
            runs: 200,
          },
          viaIR: false, // for the reason of :CompilerError: Stack too deep. Try compiling with `--via-ir` (cli) or the equivalent `viaIR: true`
        },
      },
      {
        version: '0.6.12',
        settings: {
          optimizer: {
            enabled: true,
            runs: 200,
          },
          //or the reason of :CompilerError: Stack too deep. Try compiling with `--via-ir` (cli) or the equivalent `viaIR: true`
        },
      },
      {
        version: '0.6.6',
        settings: {
          optimizer: {
            enabled: true,
            runs: 200,
          },
        },
      },
      {
        version: '0.7.6',
        settings: {
          optimizer: {
            enabled: true,
            runs: 200,
          },
        },
      },
      {
        version: '0.5.16',
        settings: {
          optimizer: {
            enabled: true,
            runs: 200,
          },
        },
      }
    ],
  },
  paths: {
    sources: "./contracts",
    tests: "./test",
    cache: "./cache",
    artifacts: "./artifacts"
  },
  // 添加 remappings 配置
  remappings: [
    "@openzeppelin/contracts/=node_modules/@openzeppelin/contracts/",
    "@openzeppelin/contracts-upgradeable/=node_modules/@openzeppelin/contracts-upgradeable/"
  ]
};

export default config;
