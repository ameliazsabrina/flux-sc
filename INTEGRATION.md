# Flux Betting - Frontend Integration Guide

This document provides specific instructions for integrating the Flux betting application into your frontend project.

## Quick Start

### 1. Install Dependencies

Add the required dependencies to your frontend project:

```bash
npm install @solana/web3.js @solana/wallet-adapter-react @solana/wallet-adapter-wallets @solana/wallet-adapter-react-ui @project-serum/anchor
```

### 2. Import Required Files

Copy the following files from this repository to your project:

- `app/api/betting.ts` → Your project's API directory
- `app/contexts/BettingContext.tsx` → Your project's context directory
- `app/types/betting.ts` → Your project's types directory

### 3. Setup Wallet Provider

In your main application file (e.g., `App.tsx` or `_app.tsx`):

```typescript
import { WalletAdapterNetwork } from "@solana/wallet-adapter-base";
import {
  ConnectionProvider,
  WalletProvider,
} from "@solana/wallet-adapter-react";
import { PhantomWalletAdapter } from "@solana/wallet-adapter-wallets";
import { WalletModalProvider } from "@solana/wallet-adapter-react-ui";
import { clusterApiUrl } from "@solana/web3.js";
import { BettingProvider } from "./contexts/BettingContext";

// Import wallet adapter styles
import "@solana/wallet-adapter-react-ui/styles.css";

function MyApp({ Component, pageProps }) {
  // Set up network and endpoint
  const network = WalletAdapterNetwork.Devnet; // Change to Mainnet for production
  const endpoint = clusterApiUrl(network);

  // Set up wallets
  const wallets = [new PhantomWalletAdapter()];

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <BettingProvider>
            <Component {...pageProps} />
          </BettingProvider>
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}

export default MyApp;
```

## Integration Examples

### 1. Connect Wallet Button

```tsx
import { useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";

function ConnectWalletButton() {
  const { connected } = useWallet();

  return (
    <div>
      <WalletMultiButton />
      {connected ? "Wallet connected!" : "Please connect your wallet"}
    </div>
  );
}
```

### 2. Create Betting Pool Form

```tsx
import { useState } from "react";
import { useBetting } from "../contexts/BettingContext";

function CreateBettingPoolForm() {
  const { createBettingPool } = useBetting();
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [options, setOptions] = useState(["", ""]);
  const [endTime, setEndTime] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  const handleAddOption = () => {
    setOptions([...options, ""]);
  };

  const handleOptionChange = (index, value) => {
    const newOptions = [...options];
    newOptions[index] = value;
    setOptions(newOptions);
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    setIsLoading(true);

    try {
      // Convert endTime string to Date object
      const endTimeDate = new Date(endTime);

      // Filter out empty options
      const validOptions = options.filter((opt) => opt.trim() !== "");

      await createBettingPool({
        name,
        description,
        options: validOptions,
        endTime: endTimeDate,
      });

      // Reset form after successful creation
      setName("");
      setDescription("");
      setOptions(["", ""]);
      setEndTime("");

      alert("Betting pool created successfully!");
    } catch (error) {
      console.error("Failed to create betting pool:", error);
      alert(`Error: ${error.message}`);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <h2>Create Betting Pool</h2>

      <div>
        <label>Name:</label>
        <input
          type="text"
          value={name}
          onChange={(e) => setName(e.target.value)}
          required
        />
      </div>

      <div>
        <label>Description:</label>
        <textarea
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          required
        />
      </div>

      <div>
        <label>Options:</label>
        {options.map((option, index) => (
          <input
            key={index}
            type="text"
            value={option}
            onChange={(e) => handleOptionChange(index, e.target.value)}
            placeholder={`Option ${index + 1}`}
            required
          />
        ))}
        <button type="button" onClick={handleAddOption}>
          Add Option
        </button>
      </div>

      <div>
        <label>End Time:</label>
        <input
          type="datetime-local"
          value={endTime}
          onChange={(e) => setEndTime(e.target.value)}
          required
        />
      </div>

      <button type="submit" disabled={isLoading}>
        {isLoading ? "Creating..." : "Create Pool"}
      </button>
    </form>
  );
}
```

### 3. Display Betting Pools

```tsx
import { useEffect, useState } from "react";
import { useBetting } from "../contexts/BettingContext";
import { BettingPool } from "../types/betting";

function BettingPoolsList() {
  const { fetchBettingPools } = useBetting();
  const [pools, setPools] = useState<BettingPool[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const loadPools = async () => {
      try {
        const fetchedPools = await fetchBettingPools();
        setPools(fetchedPools);
      } catch (error) {
        console.error("Failed to fetch betting pools:", error);
      } finally {
        setIsLoading(false);
      }
    };

    loadPools();
  }, [fetchBettingPools]);

  if (isLoading) {
    return <div>Loading betting pools...</div>;
  }

  if (pools.length === 0) {
    return <div>No betting pools available</div>;
  }

  return (
    <div>
      <h2>Available Betting Pools</h2>
      <div className="pools-grid">
        {pools.map((pool) => (
          <div key={pool.id} className="pool-card">
            <h3>{pool.name}</h3>
            <p>{pool.description}</p>
            <p>Ends: {new Date(pool.endTime).toLocaleString()}</p>
            <h4>Options:</h4>
            <ul>
              {pool.options.map((option, index) => (
                <li key={index}>{option}</li>
              ))}
            </ul>
            <a href={`/pool/${pool.id}`}>View Details</a>
          </div>
        ))}
      </div>
    </div>
  );
}
```

### 4. Place a Bet

```tsx
import { useState } from "react";
import { useBetting } from "../contexts/BettingContext";
import { BettingPool } from "../types/betting";

interface PlaceBetProps {
  pool: BettingPool;
}

function PlaceBet({ pool }: PlaceBetProps) {
  const { placeBet } = useBetting();
  const [selectedOption, setSelectedOption] = useState<number | null>(null);
  const [amount, setAmount] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  const handleSubmit = async (e) => {
    e.preventDefault();

    if (selectedOption === null) {
      alert("Please select an option");
      return;
    }

    const betAmount = parseFloat(amount);
    if (isNaN(betAmount) || betAmount <= 0) {
      alert("Please enter a valid amount");
      return;
    }

    setIsLoading(true);

    try {
      await placeBet(pool.id, selectedOption, betAmount);
      alert("Bet placed successfully!");
      // Reset form
      setSelectedOption(null);
      setAmount("");
    } catch (error) {
      console.error("Failed to place bet:", error);
      alert(`Error: ${error.message}`);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <h3>Place Your Bet</h3>

      <div>
        <label>Select Option:</label>
        {pool.options.map((option, index) => (
          <div key={index}>
            <input
              type="radio"
              id={`option-${index}`}
              name="betting-option"
              checked={selectedOption === index}
              onChange={() => setSelectedOption(index)}
            />
            <label htmlFor={`option-${index}`}>{option}</label>
          </div>
        ))}
      </div>

      <div>
        <label>Amount (SOL):</label>
        <input
          type="number"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
          min="0.001"
          step="0.001"
          required
        />
      </div>

      <button type="submit" disabled={isLoading || selectedOption === null}>
        {isLoading ? "Placing Bet..." : "Place Bet"}
      </button>
    </form>
  );
}
```

### 5. Resolve a Betting Pool (Admin Only)

```tsx
import { useState } from "react";
import { useBetting } from "../contexts/BettingContext";
import { BettingPool } from "../types/betting";

interface ResolveBetProps {
  pool: BettingPool;
  isAdmin: boolean;
}

function ResolveBet({ pool, isAdmin }: ResolveBetProps) {
  const { resolveBet } = useBetting();
  const [winningOption, setWinningOption] = useState<number | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  if (!isAdmin) {
    return null; // Only show for admins
  }

  const handleSubmit = async (e) => {
    e.preventDefault();

    if (winningOption === null) {
      alert("Please select a winning option");
      return;
    }

    setIsLoading(true);

    try {
      await resolveBet(pool.id, winningOption);
      alert("Betting pool resolved successfully!");
    } catch (error) {
      console.error("Failed to resolve betting pool:", error);
      alert(`Error: ${error.message}`);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <h3>Resolve Betting Pool</h3>

      <div>
        <label>Select Winning Option:</label>
        {pool.options.map((option, index) => (
          <div key={index}>
            <input
              type="radio"
              id={`winning-option-${index}`}
              name="winning-option"
              checked={winningOption === index}
              onChange={() => setWinningOption(index)}
            />
            <label htmlFor={`winning-option-${index}`}>{option}</label>
          </div>
        ))}
      </div>

      <button type="submit" disabled={isLoading || winningOption === null}>
        {isLoading ? "Resolving..." : "Resolve Betting Pool"}
      </button>
    </form>
  );
}
```

## Configuration

Create a `.env` file in your frontend project with the following variables:

```
REACT_APP_SOLANA_NETWORK=devnet
REACT_APP_BETTING_PROGRAM_ID=your_program_id_here
```

Replace `your_program_id_here` with the actual program ID after deployment.

## Testing Integration

1. Make sure you have a Solana wallet extension installed (e.g., Phantom)
2. Configure your wallet to use the same network specified in your environment variables
3. Ensure you have some SOL for transactions (use a faucet for devnet)
4. Try connecting your wallet and creating a betting pool
5. Test placing a bet on an existing pool
6. Test resolving a pool if you have admin access

## Troubleshooting

### Common Issues

1. **Wallet not connecting**

   - Ensure wallet extension is installed and unlocked
   - Check that your wallet is on the correct network

2. **Transaction errors**

   - Ensure you have enough SOL for transaction fees
   - Check console logs for detailed error information

3. **Pool data not showing**

   - Verify program ID is correct in your environment variables
   - Check network configuration matches deployed program

4. **Permissions errors**
   - Only the pool creator can resolve bets
   - Ensure correct wallet is connected for admin functions

## Support

For technical issues or questions about integration, contact the Flux development team at:

- GitHub Issues: [repo-url]/issues
- Email: [support-email]
