# Decentralized Identity Verifier (DIDV) Smart Contract

## 1. Structs and Storage:
- **Identity**: Stores the user’s personal details (`name`, `age`, `document_id`), proof hash, and verification status.
- **identities**: A `HashMap` where the key is the user’s account and the value is the `Identity` struct.
- **verifiers**: A `HashSet` that holds the list of approved verifier accounts.
- **owner**: The account that deployed the contract and has permissions to add/remove verifiers.

## 2. Events:
- **IdentitySubmitted**: Emitted when a user submits their identity.
- **IdentityVerified**: Emitted when a verifier successfully verifies an identity.

## 3. Functions:
- **submit_identity()**: Allows users to submit their identity details for verification.
- **verify_identity()**: Allows verifiers to verify identities by providing the correct proof hash.
- **add_verifier()** and **remove_verifier()**: Used by the contract owner to manage the list of approved verifiers.
- **is_verified()**: Checks if an identity has been verified.
- **get_identity()**: Retrieves the identity information for a specific user.
- **is_verifier()**: Checks if an account is a registered verifier.
