# @scavngr/sdk

TypeScript SDK for interacting with the Scavenger ecosystem — providing typed wrappers for Soroban smart contracts on Stellar and client bindings for the Scavenger backend API.

---

## 📖 API Reference & Documentation

For the complete reference of all backend REST endpoints, WebSocket channels, authentication headers, and rate limits, please refer to the canonical documentation:

👉 **[Scavenger Backend API Reference](../../docs/api-reference.md)**

For smart contract architecture and module map, see:
👉 **[Soroban Contract Architecture](../../stellar-contract/ARCHITECTURE.md)**

---

## 🚀 Installation

```bash
# Using npm
npm install @scavngr/sdk @stellar/stellar-sdk

# Using pnpm
pnpm add @scavngr/sdk @stellar/stellar-sdk

# Using yarn
yarn add @scavngr/sdk @stellar/stellar-sdk
```

---

## 🛠️ Quick Usage

### 1. Initializing the Client

```typescript
import { ScavengerClient } from "@scavngr/sdk";

const client = new ScavengerClient({
  network: "standalone", // or "testnet" / "public"
  rpcUrl: "http://localhost:8000/soroban/rpc",
  contractId: "CA3D...992",
  backendUrl: "http://localhost:8080",
});
```

### 2. Querying Backend Data

```typescript
// Query waste items via backend API
const wastes = await client.api.wastes.list({
  status: "active",
  limit: 20,
});

console.log("Active wastes:", wastes.data);
```

### 3. Submitting Waste to Smart Contract

```typescript
import { WasteType } from "@scavngr/types";

const tx = await client.contract.submitMaterial({
  submitter: "GBZX...3X4",
  wasteType: WasteType.Plastic,
  weightGrams: 50_000,
  latitude: 40712800,
  longitude: -74006000,
});

console.log("Transaction hash:", tx.hash);
```

---

## 📄 License

MIT
