# Paystream

*Payroll on the rails of Stellar*


Stream salaries to your distributed team in minutes, not days.


## At a glance

- _Live demo: pending_
- Stellar testnet · Soroban smart contracts
- Capability modules: `payments`
- Contract modules: `payments`

## Features

- Onboard team
- Schedule payroll
- Batch payouts
- Audit log

## How it works

1. Open the app and connect your Stellar wallet (Freighter).
2. Use the dashboard to interact with the deployed contracts.
3. Each on-chain action returns a transaction hash and an explorer link.
4. State updates come from chain reads — no off-chain trust required.

## Architecture

- **Frontend:** Vite + React + TypeScript + Tailwind + shadcn/ui + Framer Motion
- **Smart contracts:** Rust + Soroban SDK on Stellar testnet
- **Composed modules:** `payments`
- **Pages:** landing, dashboard, team, payroll

## Tech stack

| Layer | Choice |
|---|---|
| Frontend | React + Vite + Tailwind + shadcn/ui |
| Animation | Framer Motion |
| Wallet | Freighter |
| Smart contracts | Soroban (Rust) |
| Network | Stellar testnet |
| Hosting | Vercel |

## Deployed contracts

| Module | Contract ID | Explorer |
|---|---|---|
| _pending_ | _pending_ | _pending_ |

## Getting started

```bash
cd frontend
npm install
cp .env.example .env
npm run dev
```

## Environment variables

- `VITE_STELLAR_NETWORK` — network name (default `testnet`)
- `VITE_STELLAR_RPC_URL` — Soroban RPC URL
- `VITE_STELLAR_NETWORK_PASSPHRASE` — network passphrase
- `VITE_EXPLORER_BASE` — explorer base URL
- `VITE_CONTRACT_<MODULE>_ID` — one per deployed module

## License

MIT
