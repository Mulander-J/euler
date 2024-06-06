# Euler R
Rust toolbox for blockchain

## Install

```bash
➜  cargo install eulerr
```

## Commands

```bash
# Start
➜  eulerr
Rust toolbox for blockchain.

Usage: eulerr <COMMAND>

Commands:
  aptos  Aptos utils(e.g. faucet)
  sui    Sui utils(e.g. faucet)
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
# Version
➜  eulerr -V
# Aptos Commands
➜  eulerr aptos faucet [ACCOUNT] [AMOUNT]
# Sui Commands
➜  eulerr sui faucet [NETWORK] [ACCOUNT] [AMOUNT]
```
