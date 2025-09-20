# Deriverse Common

Shared packages for the Deriverse project. 
These packages serve as a bridge between the on-chain smart contract and off-chain SDK/backend services.

## Packages

### drv-errors
Common error definitions used across the entire Deriverse ecosystem. 
Provides consistent error codes and messages for both the smart contract and SDK.

#### Features

- Unified error codes: Each error has a unique numeric code for Solana program returns
- JSON serialization: Errors can be serialized to JSON with human-readable messages and structured data
- Dual environment support: Works in both smart contract and backend services via feature flags
- Rich error context: Errors can include additional fields for debugging

### drv-errors-derive
Internal macro support for the errors package.
