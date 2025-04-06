This smart contract contains the web to:

- Create betting groups with friends
- Create betting pools with different options
- Place bets on their favorite options
- Win rewards when they guess correctly

## Getting Started

### 1. Install Required Packages

First, you need to install some packages to make everything work:

```bash
npm install @solana/web3.js @solana/wallet-adapter-react @solana/wallet-adapter-wallets @solana/wallet-adapter-react-ui @project-serum/anchor
```

### 2. Copy Important Files

Copy these files from our repository to your project:

- `app/api/betting.ts` → Put in your API folder
- `app/contexts/BettingContext.tsx` → Put in your contexts folder
- `app/types/betting.ts` → Put in your types folder

These files contain all the code needed to talk to the Flux betting system.

### 3. Setup Wallet Connection

Users need a wallet to bet, so we need to add wallet connection to your app. Add this code to your main file (like `App.tsx` or `_app.tsx`):

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
  // Choose which Solana network to use (devnet for testing, mainnet for real money)
  const network = WalletAdapterNetwork.Devnet; // Change to Mainnet for production
  const endpoint = clusterApiUrl(network);

  // Set up wallets (we're using Phantom wallet here)
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

## User Management Features

### 1. Wallet Connection Button

This button lets users connect their Solana wallet:

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

### 2. User Profile Display

This component shows a user's betting history and groups:

```tsx
import { useEffect, useState } from "react";
import { useBetting } from "../contexts/BettingContext";

function UserProfile() {
  const { fetchUserProfile } = useBetting();
  const [profile, setProfile] = useState(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    async function loadProfile() {
      try {
        const userProfile = await fetchUserProfile();
        setProfile(userProfile);
      } catch (error) {
        console.error("Could not load user profile:", error);
      } finally {
        setIsLoading(false);
      }
    }

    loadProfile();
  }, [fetchUserProfile]);

  if (isLoading) {
    return <div>Loading your profile...</div>;
  }

  if (!profile) {
    return <div>Please connect your wallet to see your profile</div>;
  }

  return (
    <div className="user-profile">
      <h2>Your Profile</h2>

      <div className="stats">
        <div className="stat">
          <span>Total Winnings:</span>
          <span>{profile.totalWinnings} SOL</span>
        </div>
        <div className="stat">
          <span>Total Losses:</span>
          <span>{profile.totalLosses} SOL</span>
        </div>
      </div>

      <h3>Your Groups ({profile.groups.length})</h3>
      {profile.groups.length > 0 ? (
        <ul className="groups-list">
          {profile.groups.map((group) => (
            <li key={group.id}>
              <a href={`/group/${group.id}`}>{group.name}</a>
            </li>
          ))}
        </ul>
      ) : (
        <p>You haven't joined any groups yet</p>
      )}

      <h3>Active Bets ({profile.activeBets.length})</h3>
      {profile.activeBets.length > 0 ? (
        <ul className="bets-list">
          {profile.activeBets.map((bet) => (
            <li key={bet.id}>
              <a href={`/bet/${bet.id}`}>{bet.name}</a>
            </li>
          ))}
        </ul>
      ) : (
        <p>You don't have any active bets</p>
      )}
    </div>
  );
}
```

## Group Management Features

### 1. Create a Group

Users can create their own betting groups:

```tsx
import { useState } from "react";
import { useBetting } from "../contexts/BettingContext";

function CreateGroupForm() {
  const { createGroup } = useBetting();
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  const handleSubmit = async (e) => {
    e.preventDefault();
    setIsLoading(true);

    try {
      await createGroup(name, description);
      alert("Group created successfully!");
      // Clear form
      setName("");
      setDescription("");
    } catch (error) {
      console.error("Error creating group:", error);
      alert("Failed to create group: " + error.message);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <h2>Create a Betting Group</h2>

      <div>
        <label>Group Name:</label>
        <input
          type="text"
          value={name}
          onChange={(e) => setName(e.target.value)}
          placeholder="My Betting Group"
          required
        />
      </div>

      <div>
        <label>Description:</label>
        <textarea
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          placeholder="What is this group about?"
          required
        />
      </div>

      <button type="submit" disabled={isLoading}>
        {isLoading ? "Creating..." : "Create Group"}
      </button>
    </form>
  );
}
```

### 2. Join a Group

Users can join existing groups:

```tsx
import { useState } from "react";
import { useBetting } from "../contexts/BettingContext";

function JoinGroupForm() {
  const { joinGroup } = useBetting();
  const [groupId, setGroupId] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  const handleSubmit = async (e) => {
    e.preventDefault();
    setIsLoading(true);

    try {
      await joinGroup(groupId);
      alert("Successfully joined the group!");
      setGroupId("");
    } catch (error) {
      console.error("Error joining group:", error);
      alert("Failed to join group: " + error.message);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <h2>Join a Betting Group</h2>

      <div>
        <label>Group ID:</label>
        <input
          type="text"
          value={groupId}
          onChange={(e) => setGroupId(e.target.value)}
          placeholder="Enter the group ID"
          required
        />
      </div>

      <button type="submit" disabled={isLoading}>
        {isLoading ? "Joining..." : "Join Group"}
      </button>
    </form>
  );
}
```

### 3. View Group Details

Display information about a specific group:

```tsx
import { useEffect, useState } from "react";
import { useBetting } from "../contexts/BettingContext";

function GroupDetails({ groupId }) {
  const { fetchGroupDetails } = useBetting();
  const [group, setGroup] = useState(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    async function loadGroup() {
      try {
        const groupData = await fetchGroupDetails(groupId);
        setGroup(groupData);
      } catch (error) {
        console.error("Could not load group details:", error);
      } finally {
        setIsLoading(false);
      }
    }

    loadGroup();
  }, [groupId, fetchGroupDetails]);

  if (isLoading) {
    return <div>Loading group details...</div>;
  }

  if (!group) {
    return <div>Group not found</div>;
  }

  return (
    <div className="group-details">
      <h2>{group.name}</h2>
      <p className="description">{group.description}</p>

      <div className="created-by">Created by: {group.admin}</div>

      <h3>Members ({group.members.length})</h3>
      <ul className="members-list">
        {group.members.map((member, index) => (
          <li key={index}>{member}</li>
        ))}
      </ul>

      <h3>Active Bets</h3>
      {group.activeBets.length > 0 ? (
        <ul className="active-bets">
          {group.activeBets.map((bet) => (
            <li key={bet.id}>
              <a href={`/bet/${bet.id}`}>{bet.name}</a>
            </li>
          ))}
        </ul>
      ) : (
        <p>No active bets in this group</p>
      )}
    </div>
  );
}
```

## Betting Features

### 1. Create Betting Pool (Admin Only)

Group admins can create betting pools:

```tsx
import { useState } from "react";
import { useBetting } from "../contexts/BettingContext";

function CreateBettingPoolForm({ groupId }) {
  const { createBettingPool } = useBetting();
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [coin, setCoin] = useState("SOL");
  const [options, setOptions] = useState(["", ""]);
  const [odds, setOdds] = useState([100, 100]); // Default odds (1x)
  const [endTime, setEndTime] = useState("");
  const [minBetAmount, setMinBetAmount] = useState("0.1");
  const [isLoading, setIsLoading] = useState(false);

  const handleAddOption = () => {
    setOptions([...options, ""]);
    setOdds([...odds, 100]); // Default odds for new option
  };

  const handleOptionChange = (index, value) => {
    const newOptions = [...options];
    newOptions[index] = value;
    setOptions(newOptions);
  };

  const handleOddsChange = (index, value) => {
    const newOdds = [...odds];
    newOdds[index] = parseInt(value, 10);
    setOdds(newOdds);
  };

  const handleSubmit = async (e) => {
    e.preventDefault();
    setIsLoading(true);

    try {
      // Convert endTime string to timestamp
      const endTimeDate = new Date(endTime);

      // Filter out empty options
      const validOptions = options.filter((opt, index) => {
        if (opt.trim() !== "") {
          return true;
        }
        // Remove corresponding odds as well
        odds.splice(index, 1);
        return false;
      });

      await createBettingPool({
        groupId,
        betId: name.replace(/\s+/g, "-").toLowerCase(),
        coin,
        description,
        options: validOptions,
        odds,
        endTime: Math.floor(endTimeDate.getTime() / 1000),
        minBetAmount: parseFloat(minBetAmount) * 1000000000, // Convert to lamports
      });

      // Reset form
      setName("");
      setDescription("");
      setCoin("SOL");
      setOptions(["", ""]);
      setOdds([100, 100]);
      setEndTime("");
      setMinBetAmount("0.1");

      alert("Betting pool created successfully!");
    } catch (error) {
      console.error("Failed to create betting pool:", error);
      alert("Error: " + error.message);
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
          placeholder="Who will win the match?"
          required
        />
      </div>

      <div>
        <label>Description:</label>
        <textarea
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          placeholder="Describe your betting pool"
          required
        />
      </div>

      <div>
        <label>Coin:</label>
        <select value={coin} onChange={(e) => setCoin(e.target.value)}>
          <option value="SOL">SOL</option>
          <option value="USDC">USDC</option>
        </select>
      </div>

      <div>
        <label>Options and Odds:</label>
        {options.map((option, index) => (
          <div key={index} className="option-row">
            <input
              type="text"
              value={option}
              onChange={(e) => handleOptionChange(index, e.target.value)}
              placeholder={`Option ${index + 1}`}
              required
            />
            <input
              type="number"
              value={odds[index]}
              onChange={(e) => handleOddsChange(index, e.target.value)}
              min="100"
              step="1"
              title="Odds in basis points (100 = 1x, 200 = 2x)"
              required
            />
            <span>({odds[index] / 100}x)</span>
          </div>
        ))}
        <button type="button" onClick={handleAddOption}>
          Add Option
        </button>
      </div>

      <div>
        <label>Minimum Bet Amount (SOL):</label>
        <input
          type="number"
          value={minBetAmount}
          onChange={(e) => setMinBetAmount(e.target.value)}
          min="0.001"
          step="0.001"
          required
        />
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

### 2. List Available Betting Pools

Show users all the betting pools they can join:

```tsx
import { useEffect, useState } from "react";
import { useBetting } from "../contexts/BettingContext";
import { BettingPool } from "../types/betting";

function BettingPoolsList({ groupId }) {
  const { fetchBettingPools } = useBetting();
  const [pools, setPools] = useState([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    async function loadPools() {
      try {
        const fetchedPools = await fetchBettingPools(groupId);
        setPools(fetchedPools);
      } catch (error) {
        console.error("Failed to fetch betting pools:", error);
      } finally {
        setIsLoading(false);
      }
    }

    loadPools();
  }, [fetchBettingPools, groupId]);

  if (isLoading) {
    return <div>Loading betting pools...</div>;
  }

  if (pools.length === 0) {
    return <div>No betting pools available in this group</div>;
  }

  return (
    <div>
      <h2>Available Betting Pools</h2>
      <div className="pools-grid">
        {pools.map((pool) => (
          <div key={pool.id} className="pool-card">
            <h3>{pool.name}</h3>
            <p>{pool.description}</p>
            <p>Coin: {pool.coin}</p>
            <p>Ends: {new Date(pool.endTime * 1000).toLocaleString()}</p>
            <h4>Options:</h4>
            <ul>
              {pool.options.map((option, index) => (
                <li key={index}>
                  {option} - {pool.odds[index] / 100}x
                </li>
              ))}
            </ul>
            <p>Total Pool: {pool.totalPool / 1000000000} SOL</p>
            <a href={`/pool/${pool.id}`}>View Details</a>
          </div>
        ))}
      </div>
    </div>
  );
}
```

### 3. Place a Bet

Users can place bets on a pool:

```tsx
import { useState } from "react";
import { useBetting } from "../contexts/BettingContext";
import { BettingPool } from "../types/betting";

function PlaceBet({ pool }) {
  const { placeBet } = useBetting();
  const [selectedOption, setSelectedOption] = useState(null);
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

    // Check if bet is above minimum
    if (betAmount * 1000000000 < pool.minBetAmount) {
      alert(`Minimum bet amount is ${pool.minBetAmount / 1000000000} SOL`);
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

  // If pool is already resolved, show a message
  if (pool.resolved) {
    return <div>This betting pool has ended</div>;
  }

  // If pool end time has passed, show a message
  if (Date.now() / 1000 > pool.endTime) {
    return <div>Betting period has ended</div>;
  }

  return (
    <form onSubmit={handleSubmit}>
      <h3>Place Your Bet</h3>

      <div>
        <label>Select Option:</label>
        {pool.options.map((option, index) => (
          <div key={index} className="bet-option">
            <input
              type="radio"
              id={`option-${index}`}
              name="betting-option"
              checked={selectedOption === index}
              onChange={() => setSelectedOption(index)}
            />
            <label htmlFor={`option-${index}`}>
              {option} - {pool.odds[index] / 100}x
            </label>
          </div>
        ))}
      </div>

      <div>
        <label>Amount ({pool.coin}):</label>
        <input
          type="number"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
          min={pool.minBetAmount / 1000000000}
          step="0.001"
          placeholder={`Min: ${pool.minBetAmount / 1000000000} ${pool.coin}`}
          required
        />
      </div>

      <div className="potential-winnings">
        Potential winnings:
        {selectedOption !== null && amount
          ? ` ${(amount * (pool.odds[selectedOption] / 100)).toFixed(3)} ${
              pool.coin
            }`
          : " (select option and enter amount)"}
      </div>

      <button type="submit" disabled={isLoading || selectedOption === null}>
        {isLoading ? "Placing Bet..." : "Place Bet"}
      </button>
    </form>
  );
}
```

### 4. Resolve a Betting Pool (Admin Only)

Group admins can resolve a betting pool by selecting the winning option:

```tsx
import { useState } from "react";
import { useBetting } from "../contexts/BettingContext";

function ResolveBet({ pool, isAdmin }) {
  const { resolveBet } = useBetting();
  const [winningOption, setWinningOption] = useState(null);
  const [actualPrice, setActualPrice] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  if (!isAdmin) {
    return null; // Only show for admins
  }

  if (pool.resolved) {
    return <div>This betting pool has already been resolved</div>;
  }

  const handleSubmit = async (e) => {
    e.preventDefault();

    if (winningOption === null) {
      alert("Please select a winning option");
      return;
    }

    let price = 0;
    if (actualPrice.trim() !== "") {
      price = parseFloat(actualPrice) * 1000000000; // Convert to lamports
    }

    setIsLoading(true);

    try {
      await resolveBet(pool.id, winningOption, price);
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
          <div key={index} className="winning-option">
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

      <div>
        <label>Actual Price (optional):</label>
        <input
          type="number"
          value={actualPrice}
          onChange={(e) => setActualPrice(e.target.value)}
          step="0.000001"
          placeholder="Enter the actual price if applicable"
        />
      </div>

      <button type="submit" disabled={isLoading || winningOption === null}>
        {isLoading ? "Resolving..." : "Resolve Betting Pool"}
      </button>
    </form>
  );
}
```

### 5. Claim Winnings

Users can claim their winnings from resolved bets:

```tsx
import { useEffect, useState } from "react";
import { useBetting } from "../contexts/BettingContext";

function ClaimWinnings({ betId }) {
  const { fetchUserBet, claimWinnings } = useBetting();
  const [userBet, setUserBet] = useState(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isClaiming, setIsClaiming] = useState(false);

  useEffect(() => {
    async function loadUserBet() {
      try {
        const bet = await fetchUserBet(betId);
        setUserBet(bet);
      } catch (error) {
        console.error("Failed to fetch user bet:", error);
      } finally {
        setIsLoading(false);
      }
    }

    loadUserBet();
  }, [betId, fetchUserBet]);

  const handleClaim = async () => {
    setIsClaiming(true);

    try {
      await claimWinnings(betId);
      alert("Winnings claimed successfully!");
      // Update the UI to show claimed status
      setUserBet({ ...userBet, claimed: true });
    } catch (error) {
      console.error("Failed to claim winnings:", error);
      alert(`Error: ${error.message}`);
    } finally {
      setIsClaiming(false);
    }
  };

  if (isLoading) {
    return <div>Loading your bet...</div>;
  }

  if (!userBet) {
    return <div>You don't have a bet for this pool</div>;
  }

  if (!userBet.bet.resolved) {
    return <div>This betting pool hasn't been resolved yet</div>;
  }

  if (userBet.claimed) {
    return <div>You have already claimed your winnings</div>;
  }

  if (userBet.optionIndex !== userBet.bet.winningOption) {
    return <div>Sorry, your bet didn't win</div>;
  }

  return (
    <div className="claim-winnings">
      <h3>You Won!</h3>
      <p>
        Your bet amount: {userBet.amount / 1000000000} {userBet.bet.coin}
      </p>
      <p>Your winning option: {userBet.bet.options[userBet.optionIndex]}</p>
      <p>Odds: {userBet.bet.odds[userBet.optionIndex] / 100}x</p>
      <p>
        Estimated winnings:{" "}
        {(
          (userBet.amount * userBet.bet.odds[userBet.optionIndex]) /
          100000000000
        ).toFixed(4)}{" "}
        {userBet.bet.coin}
      </p>

      <button onClick={handleClaim} disabled={isClaiming}>
        {isClaiming ? "Claiming..." : "Claim Winnings"}
      </button>
    </div>
  );
}
```

## Setting Up Your Project

### 1. Create Environment File

Make a `.env` file in your project with these settings:

```
REACT_APP_SOLANA_NETWORK=devnet
REACT_APP_BETTING_PROGRAM_ID=your_program_id_here
```

Replace `your_program_id_here` with the actual program ID.

### 2. Create Basic Pages

1. **Home Page**: Show active betting pools and a wallet connect button
2. **Profile Page**: Show user's groups, active bets, and betting history
3. **Group Page**: Show group details and betting pools
4. **Bet Details Page**: Show details about a specific bet and allow placing bets

### 3. Basic Navigation

Create a simple navigation menu:

```tsx
function Navigation() {
  const { connected } = useWallet();

  return (
    <nav className="navbar">
      <div className="logo">
        <a href="/">Flux Betting</a>
      </div>

      <div className="nav-links">
        <a href="/">Home</a>
        {connected && <a href="/profile">My Profile</a>}
        {connected && <a href="/groups">My Groups</a>}
        <WalletMultiButton />
      </div>
    </nav>
  );
}
```

## Testing Your Integration

1. Make sure you have a Solana wallet (like Phantom) installed in your browser
2. Set your wallet to the "Devnet" network for testing
3. Get some test SOL from a faucet (search "Solana devnet faucet")
4. Try these actions to test your integration:
   - Connect your wallet
   - Create a group
   - Create a betting pool
   - Place a bet
   - Resolve a bet (if you're the admin)
   - Claim winnings

## Common Problems and Solutions

### "Wallet Not Connecting"

- Make sure your wallet extension is installed and unlocked
- Check that you're on the same network in your wallet as in your app (devnet/mainnet)

### "Transaction Error"

- Check if you have enough SOL for fees
- Look at browser console for detailed error messages

### "Can't See My Bets"

- Verify the program ID is correct in your settings
- Make sure your wallet is connected

### "Permission Denied"

- Only the group admin can create or resolve bets
- Check if you're using the correct wallet

## Next Steps

After basic integration, you might want to add:

1. Real-time updates using websockets
2. Better error handling and user feedback
3. Mobile-friendly design
4. Social features like inviting friends
