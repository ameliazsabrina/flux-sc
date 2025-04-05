# Flux Betting App - Integration Guide

This document provides instructions on how to integrate the Flux betting Solana application into a frontend project.

## Project Overview

Flux is a betting application built on the Solana blockchain. It allows users to:

- Create betting pools
- Participate in bets
- Resolve bets and distribute winnings

## Prerequisites

- Node.js (v14.x or later)
- Solana CLI tools
- A Solana wallet (e.g., Phantom)

## Getting Started

1. Clone this repository
2. Install dependencies:
   ```
   yarn install
   ```
3. Build the project:
   ```
   anchor build
   ```

## Deployment

The Solana program can be deployed to different networks:

- Localhost: `anchor deploy --provider.cluster localnet`
- Devnet: `anchor deploy --provider.cluster devnet`
- Mainnet: `anchor deploy --provider.cluster mainnet`

After deployment, note the program ID for integration.

## Frontend Integration

### Key Files for Integration

The following client-side files are essential for integration:

- `app/api/betting.ts`: Main API functions to interact with the Solana program
- `app/contexts/BettingContext.tsx`: React context for managing betting state

### Core Functions

The main functions your frontend needs to use:

#### Initialize Connection

```typescript
import { BettingProvider } from "path/to/BettingContext";

// Wrap your application with BettingProvider
<BettingProvider>
  <YourApp />
</BettingProvider>;
```

#### Create a Betting Pool

```typescript
import { useBetting } from "path/to/BettingContext";

// Inside your component
const { createBettingPool } = useBetting();

const handleCreatePool = async () => {
  try {
    await createBettingPool({
      name: "Pool Name",
      description: "Pool Description",
      options: ["Option 1", "Option 2"],
      endTime: new Date(Date.now() + 86400000), // 24 hours from now
    });
  } catch (error) {
    console.error("Failed to create pool:", error);
  }
};
```

#### Place a Bet

```typescript
import { useBetting } from "path/to/BettingContext";

// Inside your component
const { placeBet } = useBetting();

const handlePlaceBet = async (poolId, optionIndex, amount) => {
  try {
    await placeBet(poolId, optionIndex, amount);
  } catch (error) {
    console.error("Failed to place bet:", error);
  }
};
```

#### Resolve a Bet

```typescript
import { useBetting } from "path/to/BettingContext";

// Inside your component
const { resolveBet } = useBetting();

const handleResolveBet = async (poolId, winningOptionIndex) => {
  try {
    await resolveBet(poolId, winningOptionIndex);
  } catch (error) {
    console.error("Failed to resolve bet:", error);
  }
};
```

#### Fetch Betting Pools

```typescript
import { useBetting } from "path/to/BettingContext";

// Inside your component
const { fetchBettingPools } = useBetting();

const fetchPools = async () => {
  try {
    const pools = await fetchBettingPools();
    console.log("Available pools:", pools);
    return pools;
  } catch (error) {
    console.error("Failed to fetch pools:", error);
    return [];
  }
};
```

### Error Handling

The betting context provides error handling through try/catch blocks. Make sure to wrap function calls in try/catch blocks to handle any potential errors.

## Testing

You can run the test suite to verify the functionality:

```
anchor test
```

## Wallet Integration

This application requires a connected Solana wallet. We recommend using Wallet Adapter for React:

```typescript
import { WalletProvider } from "@solana/wallet-adapter-react";
import { PhantomWalletAdapter } from "@solana/wallet-adapter-wallets";

// Setup wallets
const wallets = [new PhantomWalletAdapter()];

// Wrap your app
<WalletProvider wallets={wallets}>
  <BettingProvider>
    <YourApp />
  </BettingProvider>
</WalletProvider>;
```

## Configuration

Environment variables can be set in a `.env` file:

- `REACT_APP_SOLANA_NETWORK`: The Solana network to connect to (e.g., "mainnet-beta", "devnet", "localnet")
- `REACT_APP_BETTING_PROGRAM_ID`: The deployed program ID

## Support

For any integration issues, please contact the development team.
