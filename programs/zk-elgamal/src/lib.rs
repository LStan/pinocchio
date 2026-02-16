#![no_std]

pub mod instructions;

use core::mem::MaybeUninit;

solana_address::declare_id!("ZkE1Gama1Proof11111111111111111111111111111");

// /// Byte length of a ciphertext-commitment equality proof
// pub const CIPHERTEXT_COMMITMENT_EQUALITY_PROOF_LEN: usize = 192;

// /// Byte length of a ciphertext-ciphertext equality proof
// pub const CIPHERTEXT_CIPHERTEXT_EQUALITY_PROOF_LEN: usize = 224;

// /// Byte length of a grouped ciphertext for 2 handles validity proof
// pub const GROUPED_CIPHERTEXT_2_HANDLES_VALIDITY_PROOF_LEN: usize = 160;

// /// Byte length of a grouped ciphertext for 3 handles validity proof
// pub const GROUPED_CIPHERTEXT_3_HANDLES_VALIDITY_PROOF_LEN: usize = 192;

// /// Byte length of a batched grouped ciphertext for 2 handles validity proof
// pub const BATCHED_GROUPED_CIPHERTEXT_2_HANDLES_VALIDITY_PROOF_LEN: usize = 160;

// /// Byte length of a batched grouped ciphertext for 3 handles validity proof
// pub const BATCHED_GROUPED_CIPHERTEXT_3_HANDLES_VALIDITY_PROOF_LEN: usize = 192;

// /// Byte length of a zero-ciphertext proof
// pub const ZERO_CIPHERTEXT_PROOF_LEN: usize = 96;

// /// Byte length of a percentage with cap proof
// pub const PERCENTAGE_WITH_CAP_PROOF_LEN: usize = 256;

/// Byte length of a public key validity proof with context
pub const PUBKEY_VALIDITY_PROOF_FULL_LEN: usize = 32 + 64; // 32 for context + 64 for proof

const UNINIT_BYTE: MaybeUninit<u8> = MaybeUninit::<u8>::uninit();

#[inline(always)]
fn write_bytes(destination: &mut [MaybeUninit<u8>], source: &[u8]) {
    let len = destination.len().min(source.len());
    // SAFETY:
    // - Both pointers have alignment 1.
    // - For valid (non-UB) references, the borrow checker guarantees no overlap.
    // - `len` is bounded by both slice lengths.
    unsafe {
        core::ptr::copy_nonoverlapping(source.as_ptr(), destination.as_mut_ptr() as *mut u8, len);
    }
}

macro_rules! build_instruction {
    (
        DOC_MAIN = $doc_main:literal,
        DOC_AUX = $doc_aux:literal,
        INSTRUCTION_NAME = $name:ident,
        DISCRIMINATOR = $discriminator:expr,
        PROOF_LEN = $proof_len:expr
    ) => {
        paste::paste! {
            #[doc = $doc_main]
            /// Proof in instruction data, no context state.
            ///
            #[doc = $doc_aux]
            pub struct [<$name Data>]<'a> {
                /// Proof data
                pub proof_data: &'a [u8; $proof_len],
            }

            impl [<$name Data>]<'_> {
                #[inline(always)]
                pub fn invoke(&self) -> ProgramResult {
                    // 1 byte (discriminator) + proof length bytes
                    let mut instruction_data = [UNINIT_BYTE; 1 + $proof_len];

                    instruction_data[0].write($discriminator);
                    write_bytes(&mut instruction_data[1..], self.proof_data);

                    let instruction = InstructionView {
                        program_id: &$crate::ID,
                        accounts: &[],
                        data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 1 + $proof_len) },
                    };

                    invoke(&instruction, &[])
                }
            }

            #[doc = $doc_main]
            /// Proof in instruction data, with context state.
            ///
            #[doc = $doc_aux]
            ///
            /// ### Accounts:
            ///   0. `[writable]` The proof context account to create.
            ///   1. `[]` The proof context account owner.
            pub struct [<$name DataWithContext>]<'a, 'b> {
                /// Context state account
                pub context_state_account: &'a AccountView,
                /// Context state authority account
                pub context_state_authority: &'a AccountView,
                /// Proof data
                pub proof_data: &'b [u8; $proof_len],
            }

            impl [<$name DataWithContext>]<'_,'_> {
                #[inline(always)]
                pub fn invoke(&self) -> ProgramResult {
                    // Instruction accounts
                    let instruction_accounts: [InstructionAccount; 2] = [
                        InstructionAccount::writable(self.context_state_account.address()),
                        InstructionAccount::readonly(self.context_state_authority.address()),
                    ];

                    // 1 byte (discriminator) + proof length bytes
                    let mut instruction_data = [UNINIT_BYTE; 1 + $proof_len];

                    instruction_data[0].write($discriminator);
                    write_bytes(&mut instruction_data[1..], self.proof_data);

                    let instruction = InstructionView {
                        program_id: &$crate::ID,
                        accounts: &instruction_accounts,
                        data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 1 + $proof_len) },
                    };

                    invoke(
                        &instruction,
                        &[self.context_state_account, self.context_state_authority],
                    )
                }
            }

            #[doc = $doc_main]
            /// Proof in account, no context state.
            ///
            #[doc = $doc_aux]
            ///
            /// ### Accounts:
            ///   0. `[]` Account to read the proof from.
            pub struct [<$name Account>]<'a> {
                /// Account with proof
                pub proof_account: &'a AccountView,
                /// Offset of proof in the proof account
                pub offset: u32,
            }

            impl [<$name Account>]<'_> {
                #[inline(always)]
                pub fn invoke(&self) -> ProgramResult {
                    // Instruction accounts
                    let instruction_accounts: [InstructionAccount; 1] =
                        [InstructionAccount::readonly(self.proof_account.address())];

                    // 1 byte (discriminator) + offset (4 bytes, u32)
                    let mut instruction_data = [UNINIT_BYTE; 1 + 4];

                    instruction_data[0].write($discriminator);
                    write_bytes(&mut instruction_data[1..], &self.offset.to_le_bytes());

                    let instruction = InstructionView {
                        program_id: &$crate::ID,
                        accounts: &instruction_accounts,
                        data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 1 + 4) },
                    };

                    invoke(&instruction, &[self.proof_account])
                }
            }

            #[doc = $doc_main]
            /// Proof in account, with context state.
            ///
            #[doc = $doc_aux]
            ///
            /// ### Accounts:
            ///   0. `[]` Account to read the proof from.
            ///   1. `[WRITE]` The proof context account to create.
            ///   2. `[]` The proof context account to create.
            pub struct [<$name AccountWithContext>]<'a, 'b> {
                /// Account with proof
                pub proof_account: &'a AccountView,
                /// Context state account
                pub context_state_account: &'b AccountView,
                /// Context state authority account
                pub context_state_authority: &'b AccountView,
                /// Offset of proof in the proof account
                pub offset: u32,
            }

            impl [<$name AccountWithContext>]<'_,'_> {
                #[inline(always)]
                pub fn invoke(&self) -> ProgramResult {
                    // Instruction accounts
                    let instruction_accounts: [InstructionAccount; 3] = [
                        InstructionAccount::readonly(self.proof_account.address()),
                        InstructionAccount::writable(self.context_state_account.address()),
                        InstructionAccount::readonly(self.context_state_authority.address()),
                    ];

                    // 1 byte (discriminator) + offset (4 bytes, u32)
                    let mut instruction_data = [UNINIT_BYTE; 1 + 4];

                    instruction_data[0].write($discriminator);
                    write_bytes(&mut instruction_data[1..], &self.offset.to_le_bytes());

                    let instruction = InstructionView {
                        program_id: &$crate::ID,
                        accounts: &instruction_accounts,
                        data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 1 + 4) },
                    };

                    invoke(
                        &instruction,
                        &[
                            self.proof_account,
                            self.context_state_account,
                            self.context_state_authority,
                        ],
                    )
                }
            }
        }
    };
}

use build_instruction;
