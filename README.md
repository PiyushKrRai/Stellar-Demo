# 🌟 Micro Savings Club (Stellar DApp)

A fully **permissionless rotating savings protocol (ROSCA)** built on the Stellar network using **Soroban smart contracts**.

Pool funds with friends, contribute small amounts of XLM weekly, and automatically receive the full pot in your turn — **no banks, no intermediaries, no trust required.**

---

## 🚀 Live Concept

> **Save Together. Win Together. On Stellar.**

Micro Savings Club enables anyone to:

* Create a savings pool
* Join open pools
* Contribute periodically
* Win the pooled funds in rotating rounds

---

## 🧠 Problem Statement

Traditional savings groups (chit funds / committees):

* Require **trust in an organizer**
* Are prone to **fraud or mismanagement**
* Lack **transparency**
* Have no **enforcement mechanism**

---

## 💡 Solution

A **trustless, on-chain ROSCA system** where:

* Rules are enforced by smart contracts
* No admin or central authority exists
* Funds are handled transparently
* Payouts are deterministic and verifiable

---

## 🔑 Core Principles

* ✅ Fully Permissionless (no admin control)
* ✅ Trustless Execution
* ✅ Transparent Fund Flow
* ✅ Open Participation
* ✅ Deterministic Winner Selection

---

## ⚙️ How It Works

### 1. Create a Pool

Any user can create a pool by specifying:

* Contribution amount (XLM)
* Number of members
* Round duration

Creator automatically joins as the first member.

---

### 2. Join a Pool

* Anyone can join until the pool is full
* No approvals required

---

### 3. Contribute Funds

* Each member contributes XLM per round
* Contributions are tracked on-chain

---

### 4. Select Winner

* Triggered permissionlessly by anyone
* Happens when:

  * All members contribute OR
  * Round duration expires

Winner is selected using deterministic pseudo-random logic.

---

### 5. Claim Payout

* Winner claims the total pooled amount
* Smart contract transfers funds

---

## 🏗️ Smart Contract Architecture

### 📦 Data Structures

* **Pool**

  * Pool ID
  * Members list
  * Contribution amount
  * Round tracking
  * Payout history
  * Contribution records

---

### 🔓 Key Functions

| Function        | Description               |
| --------------- | ------------------------- |
| `create_pool`   | Create a new savings pool |
| `join_pool`     | Join an existing pool     |
| `contribute`    | Deposit XLM for a round   |
| `select_winner` | Determine round winner    |
| `claim_payout`  | Withdraw winnings         |
| `get_pool`      | Fetch pool data           |

---

## 🔐 Permissionless Design

Unlike traditional systems:

❌ No `admin`
❌ No `add_member_by_owner`
❌ No `manual winner selection`

Everything is:

* User-driven
* Automatically enforced by the contract

---

## 🎯 Deterministic Winner Selection

Winner is selected using:

```
seed = ledger_timestamp + pool_id + current_round
index = seed % total_members
```

This ensures:

* No central control
* Transparent outcome
* Verifiable logic

⚠️ Note: This is pseudo-random and can be improved in future versions.

---

## 🧩 Tech Stack

### Smart Contract

* Soroban (Rust)
* Stellar SDK

### Frontend

* Next.js
* TailwindCSS

### Wallet Integration

* Freighter Wallet
* Albedo

---

## 💻 Frontend Features

* 🌐 Create and explore pools
* 👥 Join savings clubs
* 💰 Contribute with one click
* ⏱️ Track round timers
* 🏆 View winner history
* 🎉 Animated winner selection

---

## 📦 Project Structure

```
micro-savings-club/
│
├── contracts/
│   └── savings-club/
│       └── src/
│           └── lib.rs
│
├── frontend/
│   ├── components/
│   ├── pages/
│   └── utils/
│
├── scripts/
│   └── deploy.sh
│
└── README.md
```

---

## ⚠️ Known Limitations

### 1. Randomness

* Current implementation is predictable
* Can be exploited in adversarial conditions

### 2. No Penalty for Non-Contributors

* Users can skip contributions
* Only time-based fallback exists

### 3. Manual Claim

* Winners must claim funds manually

---

## 🔮 Future Improvements

* 🔐 Secure randomness (commit-reveal / oracle)
* 💸 Auto-payout system
* 📉 Penalty for missed contributions
* 🔒 Private pools with access control
* 🪙 Multi-token support (USDC, etc.)
* 📊 Reputation system for users
* 📱 Mobile-first UX

---

## 🧪 Local Development

### Prerequisites

* Rust
* Soroban CLI
* Node.js (for frontend)

---

### Build Contract

```bash
soroban contract build
```

---

### Deploy Contract (Testnet)

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/savings_club.wasm \
  --network testnet
```

---

### Run Frontend

```bash
cd frontend
npm install
npm run dev
```

---

## 🔗 Use Cases

* Friends savings groups
* College communities
* DAO micro-finance systems
* Web3-native chit funds

---

## 🤝 Contributing

Contributions are welcome!

* Fork the repo
* Create a feature branch
* Submit a PR

---

## 📜 License

MIT License

---

## 🙌 Acknowledgements

Built on:

* Stellar Network
* Soroban Smart Contracts

---

## 💬 Final Thought

> This is more than a savings tool — it's a step toward **trustless community finance**.

---

Wallet Address: GA5REBQ526MOTZKEIRCYQQISBEBTQ4HWDJ4LMOCAWKO5CS32JMPCQTM5


View on Stellar Expert: https://stellar.expert/explorer/testnet/tx/6c186b9389093e62529184eaaaad88320361d5403b4ddd36ff2380623690a8ed


<img width="1919" height="952" alt="image" src="https://github.com/user-attachments/assets/27b56fb1-02d1-4238-8f71-ca968b3eebed" />

