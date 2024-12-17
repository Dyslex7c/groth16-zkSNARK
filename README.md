# Groth16 zkSNARK Implementation in Rust

This project is a custom implementation of the Groth16 Zero-Knowledge Succinct Non-Interactive Argument of Knowledge (zkSNARK) protocol, built from the ground up using fundamental cryptographic principles in Rust.

Zero-Knowledge Proofs (ZKPs) are cryptographic methods that allow one party (the prover) to prove to another party (the verifier) that a statement is true without revealing any additional information beyond the validity of the statement itself.

## Understanding Groth16

Groth16 represents a pinnacle of zkSNARK protocol design, offering an elegant solution to complex cryptographic verification challenges. This protocol is renowned for generating extremely compact proofs that can be verified with remarkable efficiency. Unlike traditional verification methods, Groth16 allows for the validation of complex computational statements with minimal computational overhead.

The protocol operates through three critical phases: a sophisticated setup process that generates proving and verification keys, a proof generation mechanism that creates cryptographic evidence for a given statement, and a verification process that validates the proof without exposing sensitive underlying information. This approach provides an unprecedented balance between computational privacy and verification integrity.

## Cryptographic Approach

This implementation distinguishes itself by moving beyond simple library utilization, instead diving deep into the fundamental cryptographic principles underlying zero-knowledge proofs. Rather than relying on pre-built abstractions, we manually implement cryptographic primitives, giving us granular control over the entire proof generation and verification process.
By leveraging the Ark cryptography library for base operations while constructing our own protocol layers, we create a unique approach that demonstrates the intricate mechanics of zkSNARKs. This method involves manually handling low-level elliptic curve operations, constructing custom constraint systems, and implementing pairing-based verification techniques that showcase the deep mathematical foundations of modern cryptographic proofs.

## Features

- BLS12-381 elliptic curve cryptography
- Quadratic Arithmetic Program (QAP) transformation
- Custom constraint system generation
- Manual pairing-based verification

## Prerequisites

- Install Rust and Cargo package manager
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Usage

```run
$ cargo build && cargo run

Proof Generation and Verification
Public Inputs:
  Input 1: 25692268414491879152264861613477434195400093808654611602381098037180526295161
Proof Elements:
  a: 4317487948431873228554015396799640778702344920757770337680300574902307581780
  b: 49473948194802221933301408910213682934296660571129168806963496732731174706912
  c: 25692268414491879152264861613477434195400093808654611602381098037180526295161
Verification:
  Pairing Check 1: SUCCESS
  Pairing Check 2: SUCCESS
  Public Input Commitment: VALID
Final Result:
  Proof is valid: true
```

# References

[Original Groth16 Paper](https://eprint.iacr.org/2016/260.pdf)

[Ark Cryptography Library](https://github.com/arkworks-rs)
