# Dank Coin - SPL Token (Solana)

## 📦 Build and Deploy from GitHub Codespaces

### Setup (Inside GitHub Codespaces Terminal):
```bash
solana config set --url devnet
solana airdrop 2
anchor build
anchor deploy
spl-token create-token --decimals 9 --mint-authority YOUR_WALLET
spl-token create-account TOKEN_ADDRESS
spl-token mint TOKEN_ADDRESS 300000000000000000
```

Replace `YOUR_WALLET` and `TOKEN_ADDRESS` with your real values after deploy.
