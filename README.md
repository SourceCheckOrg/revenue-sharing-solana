# Revenue Sharing Solana Program

### Environment Setup
1. Install Rust from https://rustup.rs/
2. Install Solana v1.6.2 or later from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

### Build for program compiled natively
```
$ cargo build
```

### Build the program compiled for BPF
```
$ cargo build-bpf
```

## Program implementation

The program has basically two instructions: one for initilizing a revenue share and the other for withdrawing funds from the shared account, proportional to the shares each recipient has.

Two accounts are also necessary: one for store the revenue share and the other (token account) to hold the funds to be shared. 
