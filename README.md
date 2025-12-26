# Solana CLMM (Concentrated Liquidity Market Maker)

## Overview

This project implements a Concentrated Liquidity Market Maker (CLMM) on Solana, inspired by the architecture of Raydium CLMM and Uniswap V3. Unlike traditional Automated Market Makers (AMMs) that distribute liquidity uniformly across the entire price curve ($0$ to $\infty$), CLMMs allow Liquidity Providers (LPs) to allocate capital within specific custom price ranges.

This design significantly improves capital efficiency, allowing LPs to earn higher fees with less capital, while providing traders with deeper liquidity and lower slippage around the active price.

## Core Concepts

### 1. Concentrated Liquidity

In a standard Constant Product Market Maker ($x \cdot y = k$), liquidity is spread thinly across all possible prices. Most of this liquidity is never used.
In a CLMM, you choose a price range (e.g., SOL/USDC between $140 and $160). Your capital is only used when the market price is within this range.

- **In Range**: Your position earns trading fees. You act as a market maker selling the asset that is rising in value and buying the one falling.
- **Out of Range**: Your position becomes 100% of one asset and stops earning fees until the price returns to your range.

### 2. Ticks

The continuous price curve is divided into discrete points called "ticks".

- A tick represents a specific price $P(i) = 1.0001^i$.
- Liquidity positions are defined by a Lower Tick and an Upper Tick.
- As the price moves, it crosses these ticks, and the active liquidity changes based on which positions encompass the current tick.

### 3. SqrtPriceX64

To handle precision efficiently on Solana, prices are stored as the square root of the price, scaled by a large number ($2^{64}$). This allows for high-precision math using integer arithmetic.

## Logic and Mechanics

### Liquidity Math

The core logic relies on the relationship:
$L = \frac{\Delta y}{\Delta \sqrt{P}}$
or
$L = \Delta x \cdot \sqrt{P} \cdot \sqrt{P_{target}}$

When a swap occurs:

1. The contract calculates how much of the "active liquidity" is available at the current tick.
2. It determines how much of the input token can be swapped before reaching the next tick boundary.
3. If the swap is large enough to cross a tick, the "active liquidity" value is updated as positions enter or exit their effective range.
4. Fees are accrued proportionally to the liquidity contributed to the active range.

### Fee Growth

Fees are tracked globally per tick. When a position is active, it earns its share of the global fee growth.

- `feeGrowthGlobal`: Total fees collected per unit of liquidity worldwide.
- `feeGrowthInside`: Calculated for a specific position's range by subtracting growth "outside" the lower and upper ticks from the global growth.

## Trader & LP Examples

### Scenario: The Trader

**Goal**: Buy SOL with USDC.

- **Action**: Trader sends 1000 USDC.
- **Execution**: The CLMM checks the current Price. It finds all LPs offering SOL in the current tick. It consumes their liquidity. If the price impact pushes the price to the next tick, it begins consuming liquidity from LPs in that new tick range.
- **Result**: Trader gets more SOL than in a standard AMM because liquidity is "piled up" around the current price.

### Scenario: The Liquidity Provider (LP)

#### Strategy 1: "The Stable Market" (Sideways Trading)

- **Market View**: SOL is trading at $150. You think it will bounce between $145 and $155 for a week.
- **Action**: You open a position with range [$145, $155].
- **Result**: Since your range is narrow (Concentrated), your capital works 10x-50x harder than a standard pool. You collect massive fees as long as SOL stays in this channel.
- **Risk**: If SOL shoots to $160, you sell all your SOL early (at an average of your range) and hold 100% USDC. You miss the upside beyond $155 (Impermanent Loss).

#### Strategy 2: "The Range Trap" (Buying the Dip)

- **Market View**: SOL is at $150. You want to buy SOL if it drops to $130, but you want to get paid to set that limit order.
- **Action**: You provide liquidity in a range _below_ the current price, e.g., [$130, $140].
- **Result**: Currently, your position is 100% USDC (inactive). If the price drops into your range, the CLMM starts converting your USDC to SOL. If it goes below $130, you are now 100% in SOL.
- **Benefit**: You effectively "bought the dip" slowly while earning trading fees on the way down.

#### Strategy 3: "Full Range" (Passive / Lazy)

- **Market View**: You don't want to manage positions.
- **Action**: Set range from near $0$ to Infinity.
- **Result**: Acts exactly like a standard Raydium/Uniswap v2 pool. Low maintenance, but much lower fee efficiency compared to active managers.
