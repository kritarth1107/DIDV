#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod did_verifier {
    use ink_storage::traits::{SpreadAllocate, PackedLayout, SpreadLayout};

    /// Identity struct to store user information
    #[derive(Debug, Clone, PartialEq, Eq, PackedLayout, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Identity {
        name: String,
        age: u32,
        document_id: String,
        proof_hash: [u8; 32],  // 32-byte array to store the hash
        is_verified: bool,
        verifier: Option<AccountId>, // Optional verifier address
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct DIDVerifier {
        identities: ink_storage::collections::HashMap<AccountId, Identity>, // Mapping from account to Identity
        verifiers: ink_storage::collections::HashSet<AccountId>,            // Set of approved verifiers
        owner: AccountId,                                                  // Contract owner
    }

    #[ink(event)]
    pub struct IdentitySubmitted {
        #[ink(topic)]
        account: AccountId,
        name: String,
        age: u32,
        proof_hash: [u8; 32],
    }

    #[ink(event)]
    pub struct IdentityVerified {
        #[ink(topic)]
        account: AccountId,
        #[ink(topic)]
        verifier: AccountId,
    }

    impl DIDVerifier {
        /// Constructor initializes the owner as the contract deployer
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            ink_lang::codegen::initialize_contract(|contract: &mut Self| {
                contract.owner = caller;
                contract.verifiers = ink_storage::collections::HashSet::new();
                contract.identities = ink_storage::collections::HashMap::new();
            })
        }

        /// Submit identity for verification
        #[ink(message)]
        pub fn submit_identity(
            &mut self,
            name: String,
            age: u32,
            document_id: String,
            proof_hash: [u8; 32],
        ) -> Result<(), &'static str> {
            let caller = self.env().caller();
            // Ensure identity does not already exist for this account
            if self.identities.contains_key(&caller) {
                return Err("Identity already submitted");
            }

            // Create and store the identity
            let identity = Identity {
                name: name.clone(),
                age,
                document_id,
                proof_hash,
                is_verified: false,
                verifier: None,
            };
            self.identities.insert(caller, identity);

            // Emit an event for identity submission
            self.env().emit_event(IdentitySubmitted {
                account: caller,
                name,
                age,
                proof_hash,
            });

            Ok(())
        }

        /// Verify an identity with a matching proof hash (only verifiers can call this)
        #[ink(message)]
        pub fn verify_identity(&mut self, account: AccountId, proof_hash: [u8; 32]) -> Result<(), &'static str> {
            let caller = self.env().caller();
            // Ensure the caller is a registered verifier
            if !self.verifiers.contains(&caller) {
                return Err("Only verifiers can verify identities");
            }

            // Ensure the identity exists and is not already verified
            let identity = self.identities.get_mut(&account).ok_or("Identity not found")?;
            if identity.is_verified {
                return Err("Identity already verified");
            }

            // Ensure the proof hash matches the stored one
            if identity.proof_hash != proof_hash {
                return Err("Proof hash does not match");
            }

            // Mark the identity as verified
            identity.is_verified = true;
            identity.verifier = Some(caller);

            // Emit an event for identity verification
            self.env().emit_event(IdentityVerified {
                account,
                verifier: caller,
            });

            Ok(())
        }

        /// Add a new verifier (only contract owner can add verifiers)
        #[ink(message)]
        pub fn add_verifier(&mut self, verifier: AccountId) -> Result<(), &'static str> {
            let caller = self.env().caller();
            // Ensure only the owner can add verifiers
            if caller != self.owner {
                return Err("Only the owner can add verifiers");
            }

            // Add the verifier to the set of verifiers
            self.verifiers.insert(verifier);
            Ok(())
        }

        /// Remove a verifier (only contract owner can remove verifiers)
        #[ink(message)]
        pub fn remove_verifier(&mut self, verifier: AccountId) -> Result<(), &'static str> {
            let caller = self.env().caller();
            // Ensure only the owner can remove verifiers
            if caller != self.owner {
                return Err("Only the owner can remove verifiers");
            }

            // Remove the verifier from the set of verifiers
            self.verifiers.take(&verifier);
            Ok(())
        }

        /// Check if an identity is verified
        #[ink(message)]
        pub fn is_verified(&self, account: AccountId) -> bool {
            if let Some(identity) = self.identities.get(&account) {
                return identity.is_verified;
            }
            false
        }

        /// Get the stored identity for a specific account
        #[ink(message)]
        pub fn get_identity(&self, account: AccountId) -> Option<Identity> {
            self.identities.get(&account).cloned()
        }

        /// Check if an account is a registered verifier
        #[ink(message)]
        pub fn is_verifier(&self, account: AccountId) -> bool {
            self.verifiers.contains(&account)
        }
    }
}
