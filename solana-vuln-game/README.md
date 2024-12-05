# Solana Vulnerability Game 🎮

Devnet Address: `CyE4QLqUJ2PzMEk8gd7eYyyj8c4VCdXSMXATdGJkoMBT`

A hands-on educational game designed to help developers understand and identify common vulnerabilities in Solana smart contracts.

## Overview

This project is an interactive game that simulates various security vulnerabilities commonly found in Solana programs. Players can practice identifying and exploiting these vulnerabilities in a safe, controlled environment.

## Challenges

The game includes the following vulnerability challenges:

- Space Validation: Learn about space-related validation vulnerabilities
- Input Validation: Explore input sanitization and validation bypasses
- Arithmetic Underflow: Understand numerical underflow vulnerabilities
- Arithmetic Overflow: Practice identifying overflow conditions
- Program ID Verification: Learn about program identity verification issues

## Prerequisites

- Rust (latest stable version)
- Solana CLI Tools
- Node.js (v14 or higher)
- Yarn or npm
- [Anchor Framework](https://www.anchor-lang.com/)

## Installation

1. Clone the repository:
   

   git clone [your-repository-url]
   cd solana-vuln-game
   

2. Install dependencies:
   

   yarn install
   

3. Build the program:
   

   anchor build
   

## Configuration

1. Create a .env file in the root directory:
   

   SOLANA_CLUSTER_URL=https://api.devnet.solana.com
   PROGRAM_ID=DEFuzL6ArEcszLSgy1pQBLSdyBd7BKR5CUdckq2RXn2A
   

2. Create a wallet for testing:
   

   solana-keygen new -o walletSec.json
   

## Usage

1. Start the game:
   

   cd game-script
   yarn start
   

2. Follow the interactive prompts to select and attempt different challenges.

## Project Structure

- /programs/solana-vuln-game/ - Solana program source code
- /game-script/ - Game client interface
- /tests/ - Program test files
- /migrations/ - Deployment scripts

## Testing

Run the test suite:


anchor test


## Acknowledgments

Thanks to Jeff & the Turbin team!
