# User Interface Design

## 1. Portfolio Dashboard (Inventory View)
**Goal:** Show current holdings and performance.

### Visual Components
*   **Asset List:** Grouped by Coin (e.g., Bitcoin Row).
    *   Columns: Asset, Balance (Sum of `remaining`), Avg Buy Price, Current Price, Unrealized P&L.
    *   *Expand Row:* Clicking a coin expands to show the list of individual **Active Lots**.
*   **Allocation Chart:** Donut chart of current portfolio value.

## 2. Income/History Dashboard (Historical View)
**Goal:** Show earnings over time.

### Visual Components
*   **Income Chart:** Bar chart showing value of `staking`, `mining`, `airdrop` rewards by month.
*   **Filter:** "Show only [Staking] rewards in [2024]".

## 3. "Add Transaction" Dialog
A unified modal for entering data, with tabs for different activities.

### Tab A: Buy
*   **Fields:** Date, Asset, Amount, Price per Unit (Fiat), Fee, Source (Wallet).
*   **Result:** Creates `crypto_acquisition` (kind: 'buy').

### Tab B: Earn (Staking/Mining/Airdrop)
*   **Fields:** Date, Asset, Amount, **Fair Market Value (Auto-fetched or Manual)**, Source, Type (Select: Staking/Airdrop/etc).
*   **Result:** Creates `crypto_acquisition` (kind: 'staking').
*   *UX Note:* The "Cost Basis" field is relabeled "Value at Receipt" to make sense to the user.

### Tab C: Sell
*   **Fields:** Date, Asset, Amount, Sell Price, Source.
*   **Logic:**
    1.  User enters sell amount.
    2.  System displays "Preview": "This will close 2 lots and partially reduce 1 lot (FIFO)."
    3.  User confirms.
    4.  **Result:** Updates `amount_remaining` on affected lots.

### Tab D: Transfer / Bridge / Wrap
*   **Fields:** 
    *   Date
    *   Source (From)
    *   **Source Asset** (e.g. ETH)
    *   Destination (To)
    *   **Destination Asset** (e.g. WETH) - *Defaults to Source Asset. If different, enables "Transformation" mode.*
    *   **Amount Sent** (e.g., 1.0 ETH)
    *   **Fee** (e.g., 0.005 ETH)
    *   **Amount Received** (calculated: 0.995 WETH)
    *   *Checkbox:* "Treat as Taxable Trade?" (If unchecked, acts as Transformation/Rollover).
*   **Result (Transformation):** 
    *   Reduces Source Lot.
    *   Creates New Lot (new Asset ID).
    *   **Copies** original Cost Basis & Date to new lot (No Tax Event).

## 4. Lot Management (Advanced Detail View)
A specific page for power users to manage the tax lots directly.

*   **Table:** List of all `crypto_acquisition` rows.
*   **Actions:**
    *   *Edit:* Fix mistakes in price/date.
    *   *Split:* Break one large lot into two (useful for partial transfers).
    *   *Merge:* (Advanced) Combine dust.
