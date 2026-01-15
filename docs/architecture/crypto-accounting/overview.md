# Crypto Accounting Architecture

This document outlines the architectural decisions, data models, and business rules for the Crypto Accounting module in Auro.

## Core Philosophy: The "Smart Tax Lot" System

The system is designed around the concept of **Immutable Acquisitions**. Every time crypto enters the user's possession—whether bought, earned, or gifted—it creates a permanent record (a "Lot").

This allows us to answer two distinct questions from the same data:
1.  **Inventory:** "What do I own right now?" (Sum of `amount_remaining`)
2.  **History:** "How much did I earn from staking last year?" (Sum of `amount_initial` where `kind='staking'`)

## Data Model

### Table: `crypto_acquisition`
The source of truth for all assets entering the system.

| Column             | Type          | Description                                                              |
| :----------------- | :------------ | :----------------------------------------------------------------------- |
| `id`               | `TEXT (UUID)` | Unique identifier for the lot.                                           |
| `asset_id`         | `TEXT`        | Foreign key to `asset` table (e.g., 'BTC', 'ETH').                       |
| `amount_initial`   | `REAL`        | The original quantity acquired. **Immutable.**                           |
| `amount_remaining` | `REAL`        | The quantity currently held. Decreases when sold.                        |
| `cost_basis_fiat`  | `REAL`        | The value per unit in fiat at the moment of acquisition (includes fees). |
| `fee_fiat`         | `REAL`        | The portion of the cost that was a fee (for audit/reporting).            |
| `fiat_id`          | `INTEGER`     | FK to `fiat` table (the currency of the cost basis).                     |
| `acquired_at`      | `TIMESTAMP`   | Date and time of acquisition.                                            |
| `source`           | `TEXT`        | Where the asset is held (e.g., 'Ledger', 'Coinbase').                    |
| `kind`             | `TEXT`        | The nature of the acquisition (Enum).                                    |
| `notes`            | `TEXT`        | Optional user notes.                                                     |

#### `kind` Enum Values
*   `buy`: Purchased with fiat.
*   `trade_in`: Acquired by swapping another crypto (Taxable event).
*   `transfer_in`: Moved from another wallet (Non-taxable, preserves history).
*   `airdrop`: Free distribution.
*   `staking`: Reward for locking assets.
*   `mining`: Reward for securing network.
*   `defi_reward`: Yield farming / LP rewards.
*   `lending`: Interest from lending platforms.
*   `gift_received`: Gift from another person.
*   `transformation`: Non-taxable conversion (Wrap/Unwrap/Bridge).

## Business Rules

### 1. The Immutable Record Rule
We never delete a `crypto_acquisition` record, even if `amount_remaining` reaches 0. This ensures historical reporting (Income Statements) remains accurate forever.

### 2. The Disposal Logic (FIFO/LIFO)
When a user "Sells" or "Spends" crypto, they are strictly modifying the `amount_remaining` of existing lots.
*   **Action:** User logs a "Sell 1 BTC".
*   **System:** Automatically finds the oldest available lots (FIFO) and reduces their `amount_remaining`.
*   **Audit:** A separate `crypto_disposal` table (future feature) tracks these decrement events linking the Sell to the specific Lot ID.

### 3. Multi-Currency Normalization
Cost basis is stored in the original currency (e.g., EUR).
*   **Reporting:** All values are converted to the user's *current* default currency (e.g., USD) on-the-fly using historical rates from `fiat_exchange_rate`.

### 4. LP Tokens & DeFi
Liquidity Provider (LP) tokens are treated as unique Assets.
*   **Entering Pool:** `Disposal` of ETH + `Disposal` of USDC -> `Acquisition` of ETH-USDC-LP.
*   **Exiting Pool:** `Disposal` of ETH-USDC-LP -> `Acquisition` of ETH + `Acquisition` of USDC.

### 5. Gas Fees & Transaction Costs
Gas is treated differently depending on context:
*   **Acquisition Fee:** When buying an asset (e.g. Buying ETH), the gas fee is added to the **Cost Basis** of the new lot. The `fee_fiat` column stores this amount for transparency.
*   **Transaction Costs:** When sending assets or interacting with contracts, the gas paid (e.g. in ETH) is a **Disposal** of that gas token.
    *   *System Action:* Reduces `amount_remaining` of the gas token (ETH/SOL).
    *   *Tax Impact:* This is technically a "Sale" of ETH to pay for the service.

### 6. Withdrawal & Network Fees (Shrinkage)
When transferring assets (e.g., CEX to Wallet) where the fee is taken from the asset itself (Shrinkage):
*   **Scenario:** User withdraws 1.0 BTC, Exchange takes 0.0005 BTC fee, User receives 0.9995 BTC.
*   **Logic:**
    1.  **Source Lot:** Reduced by full amount (1.0 BTC).
    2.  **Destination Lot:** Created with net amount (0.9995 BTC).
    3.  **The Fee:** The 0.0005 BTC difference is recorded as an expense (Disposal).
    4.  **Recording:** The `fee_fiat` on the new destination lot records the value of that 0.0005 BTC at the time of transfer, ensuring the cost of the transfer is preserved for tax deductibility (where applicable).

### 7. Cross-Chain Bridging (XCM Asset Transfers / L2s)
Bridging assets (e.g., DOT via **XCM Asset Transfer** from Relay Chain to Acala, or ETH from Mainnet to Arbitrum) is treated as a **Self-Transfer** with shrinkage.
*   **Logic:** Since the underlying asset remains the same (DOT -> DOT), it is not a Taxable Trade.
*   **Mechanism:** Same as Rule #6 (Withdrawal).
    *   Source Lot (Relay Chain) reduced by full amount sent.
    *   Destination Lot (Parachain) created with net amount received.
    *   Bridge Fee (XCM/Gas) is the difference, recorded as expense.
*   **Result:** Original Cost Basis and Holding Period (Long/Short term) are preserved and carried over to the new chain.

### 8. Asset Transformation (Wrapping & L2 Bridging)
When an asset changes form but represents the same underlying value (e.g., ETH $\to$ WETH, or ETH $\to$ Arbitrum ETH if tracked as a different asset ID).
*   **Scenario:** User bridges ETH (L1) to Polygon, receiving WETH (Polygon).
*   **The Problem:** DB sees `asset_id='ETH'` moving to `asset_id='WETH_POLY'`. A standard trade would reset the Holding Period.
*   **The Fix (`kind='transformation'`):**
    1.  **Source Lot:** Closed (Amount = 0).
    2.  **Destination Lot:** Created with the new `asset_id`.
    3.  **Critical Logic:** The **Cost Basis** and **Original Acquired Date** are **COPIED** from the Source Lot to the Destination Lot.
    4.  **Result:** The user effectively holds the "same" economic position in a new wrapper. No capital gains tax is triggered (depending on jurisdiction settings).
