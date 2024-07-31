<p align="center">
  <picture>
    <source srcset="docs/assets/logo-white.svg" media="(prefers-color-scheme: dark)">
    <img src="docs/assets/logo-dark.svg" alt="Mugraph Logo" width="300">
  </picture>

<p align="center"><em>Instant, untraceable payments for Cardano.</em></p>

<p align="center">
    <img src="https://github.com/mugraph-payments/mugraph/actions/workflows/build.yml/badge.svg" alt="Build Status" />
    <a href="https://opensource.org/licenses/Apache-2.0">
      <img src="https://img.shields.io/badge/License-Apache_2.0-blue.svg" alt="Apache 2.0 Licensed" />
    </a>
    <a href="https://opensource.org/licenses/MIT">
      <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="MIT Licensed" />
    </a>
    <a href="https://discord.gg/npSJU6Qk">
      <img src="https://dcbadge.limes.pink/api/server/npSJU6Qk?style=social" alt="Mugraph Discord Server" />
    </a>
  </p>
</p>

Mugraph (pronounced *"mew-graph"*) is a Layer 2 Network for the Cardano blockchain for untraceable payments with instant finality.

By **untraceable**, we mean that, inside a given group $A$ of users:

- All transactions between users inside $A$ are untraceable, meaning senders and receivers are not bound in any way.
- All transactions to users outside $A$ come from a single, shared identity for all group participants.

This shared identity comes from **Delegators**, behaving similarly to Payment Networks in the traditional banking system, like Paypal, Venmo or CashApp, but with some crucial distinctions:

- Delegates hold funds in a fully auditable Smart Contract Vault, held in the Layer 1.
- Delegates can not spend user funds without their authorization.
- Delegates are **blind**, meaning they don't know who is transacting.
- Delegates provide **group concealing** for their users, signing transactions on behalf of them.

An user can, and usually will, hold balance in multiple Delegates at once, and they do not need to have balance in a Delegate to receive payments there.

Essentially, a Delegate has three main roles:

1. Verifying *operation proofs* and signing **Blinded Notes**.
1. Signing external transctions on behalf of the user.
1. Emitting **Notes** in response to user deposits.

## Glossary

| Symbol | Description |
|--------|-------------|
| $G$    | A generator point in the Ristreto25519 curve. |
| $n$    | A Note, blindly signed by the Delegate and ready to be used. |
| $n'$    | A Note with a blinded nullifier to be sent to the Delegate for signing. |

## Types

### Notes

TODO.

### Blinded Notes

TODO.

## Operations

### $F$: Fission

Splits a note into two blinded notes. It is defined as:

$F(n, i) \\mapsto { n'\_o, n'\_c }$

Where:

- $n$ is the input note to be slit in two
- $i$ is the output amount requested by the operation
- $n'\_o$ is a blinded note for the amount $i$
- $n'\_c$ is another blinded note for the amount $n_i - i$, where $n_i$ is the note amount.

### $F'$: Fusion

Joins two notes with the same asset id and server keys into a single one. It is defined as:

$F'(n_0, n_1) \\mapsto n'$

Where:

- $n_0$ and $n_1$ are the input notes to be fused
- $n'$ is a blinded node for the amount $n_0i + n_1i$

Mugraph (pronounced *"mew-graph"*) is a Layer 2 Network for the Cardano blockchain for untraceable payments with instant finality. It is very simplified in both operations and architecture, meant to be easy to integrate anywhere.

# Table of Contents

## The Basics

1. [Motivation](./docs/motivation.md)
1. [Roadmap](./docs/roadmap.md)
1. [Licensing](./docs/licensing.md)

## The Protocol

1. [Delegates](./protocol/delegates.md)
1. [Wallets](./protocol/wallets.md)

## The Future

1. [Cross-node Transfers](./future/cross-node-transfers.md)
