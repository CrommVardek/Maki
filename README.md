# Maki

Maki is a [MACI](https://ethresear.ch/t/minimal-anti-collusion-infrastructure/5413) implementation in [ink!](https://github.com/paritytech/ink)

## Development

### To be installed

 - [Rust & Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
 - (Optional) Unless you know exactly how to build and deploy an ink! smart contract, we recommend to take a look at [this tutorial](https://docs.substrate.io/tutorials/smart-contracts/prepare-your-first-contract/) to set up your environment properly.

### Build

Command to build :

`cargo +nightly-2022-08-15 contract build`

### Test

Run the tests :

`cargo +nightly-2022-08-15 test`

### Deploy

## Credits

Maki is based on these researches at [eth research](https://ethresear.ch/t/minimal-anti-collusion-infrastructure/5413), the implementation mostly follows the [maci specifications](https://github.com/privacy-scaling-explorations/maci/tree/c4fdbcf7373080ba62225ba669a1bf77e057c483/specs) and is therefore largly inspired by [maci code base](https://github.com/privacy-scaling-explorations/maci). Thanks to all the people who worked on MACI.