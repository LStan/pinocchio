use core::slice::from_raw_parts;

use solana_account_view::AccountView;
use solana_instruction_view::{cpi::invoke, InstructionAccount, InstructionView};
use solana_program_error::ProgramResult;

use crate::{build_instruction, write_bytes, PUBKEY_VALIDITY_PROOF_FULL_LEN, UNINIT_BYTE};

// const DISCRIMINATOR: u8 = 4;
// const PROOF_LEN: usize = PUBKEY_VALIDITY_PROOF_FULL_LEN;

// /// Verify a public key validity zero-knowledge proof
// ///
// /// A public key validity proof certifies that an ElGamal public key is well-formed and the
// /// prover knows the corresponding secret key.
// ///
// /// Proof in instruction data, no context state
// pub struct VerifyPubkeyValidityData<'a> {
//     /// Proof data
//     pub proof_data: &'a [u8; PROOF_LEN],
// }

// impl VerifyPubkeyValidityData<'_> {
//     #[inline(always)]
//     pub fn invoke(&self) -> ProgramResult {
//         // 1 byte (discriminator) + proof length bytes
//         let mut instruction_data = [UNINIT_BYTE; 1 + PROOF_LEN];

//         instruction_data[0].write(DISCRIMINATOR);
//         write_bytes(&mut instruction_data[1..], self.proof_data);

//         let instruction = InstructionView {
//             program_id: &crate::ID,
//             accounts: &[],
//             data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 1 + PROOF_LEN) },
//         };

//         invoke(&instruction, &[])
//     }
// }

// /// Verify a public key validity zero-knowledge proof
// ///
// /// A public key validity proof certifies that an ElGamal public key is well-formed and the
// /// prover knows the corresponding secret key.
// ///
// /// Proof in instruction data, with context state
// ///
// /// ### Accounts:
// ///   0. `[writable]` The proof context account to create.
// ///   1. `[]` The proof context account owner.
// pub struct VerifyPubkeyValidityDataWithContext<'a, 'b> {
//     /// Context state account
//     pub context_state_account: &'a AccountView,
//     /// Context state authority account
//     pub context_state_authority: &'a AccountView,
//     /// Proof data
//     pub proof_data: &'b [u8; PROOF_LEN],
// }

// impl VerifyPubkeyValidityDataWithContext<'_, '_> {
//     #[inline(always)]
//     pub fn invoke(&self) -> ProgramResult {
//         // Instruction accounts
//         let instruction_accounts: [InstructionAccount; 2] = [
//             InstructionAccount::writable(self.context_state_account.address()),
//             InstructionAccount::readonly(self.context_state_authority.address()),
//         ];

//         // 1 byte (discriminator) + proof length bytes
//         let mut instruction_data = [UNINIT_BYTE; 1 + PROOF_LEN];

//         instruction_data[0].write(DISCRIMINATOR);
//         write_bytes(&mut instruction_data[1..], self.proof_data);

//         let instruction = InstructionView {
//             program_id: &crate::ID,
//             accounts: &instruction_accounts,
//             data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 1 + PROOF_LEN) },
//         };

//         invoke(
//             &instruction,
//             &[self.context_state_account, self.context_state_authority],
//         )
//     }
// }

// /// Verify a public key validity zero-knowledge proof
// ///
// /// A public key validity proof certifies that an ElGamal public key is well-formed and the
// /// prover knows the corresponding secret key.
// ///
// /// Proof in account, no context state
// ///
// /// ### Accounts:
// ///   0. `[]` Account to read the proof from.
// pub struct VerifyPubkeyValidityAccount<'a> {
//     /// Account with proof
//     pub proof_account: &'a AccountView,
//     /// Offset of proof in the proof account
//     pub offset: u32,
// }

// impl VerifyPubkeyValidityAccount<'_> {
//     #[inline(always)]
//     pub fn invoke(&self) -> ProgramResult {
//         // Instruction accounts
//         let instruction_accounts: [InstructionAccount; 1] =
//             [InstructionAccount::readonly(self.proof_account.address())];

//         // 1 byte (discriminator) + offset (4 bytes, u32)
//         let mut instruction_data = [UNINIT_BYTE; 1 + 4];

//         instruction_data[0].write(DISCRIMINATOR);
//         write_bytes(&mut instruction_data[1..], &self.offset.to_le_bytes());

//         let instruction = InstructionView {
//             program_id: &crate::ID,
//             accounts: &instruction_accounts,
//             data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 1 + 4) },
//         };

//         invoke(&instruction, &[self.proof_account])
//     }
// }

// /// Verify a public key validity zero-knowledge proof
// ///
// /// A public key validity proof certifies that an ElGamal public key is well-formed and the
// /// prover knows the corresponding secret key.
// ///
// /// Proof in account, with context state
// ///
// /// ### Accounts:
// ///   0. `[]` Account to read the proof from.
// ///   1. `[WRITE]` The proof context account to create.
// ///   2. `[]` The proof context account to create.
// pub struct VerifyPubkeyValidityAccountWithContext<'a, 'b> {
//     /// Account with proof
//     pub proof_account: &'a AccountView,
//     /// Context state account
//     pub context_state_account: &'b AccountView,
//     /// Context state authority account
//     pub context_state_authority: &'b AccountView,
//     /// Offset of proof in the proof account
//     pub offset: u32,
// }

// impl VerifyPubkeyValidityAccountWithContext<'_, '_> {
//     #[inline(always)]
//     pub fn invoke(&self) -> ProgramResult {
//         // Instruction accounts
//         let instruction_accounts: [InstructionAccount; 3] = [
//             InstructionAccount::readonly(self.proof_account.address()),
//             InstructionAccount::writable(self.context_state_account.address()),
//             InstructionAccount::readonly(self.context_state_authority.address()),
//         ];

//         // 1 byte (discriminator) + offset (4 bytes, u32)
//         let mut instruction_data = [UNINIT_BYTE; 1 + 4];

//         instruction_data[0].write(DISCRIMINATOR);
//         write_bytes(&mut instruction_data[1..], &self.offset.to_le_bytes());

//         let instruction = InstructionView {
//             program_id: &crate::ID,
//             accounts: &instruction_accounts,
//             data: unsafe { from_raw_parts(instruction_data.as_ptr() as _, 1 + 4) },
//         };

//         invoke(
//             &instruction,
//             &[
//                 self.proof_account,
//                 self.context_state_account,
//                 self.context_state_authority,
//             ],
//         )
//     }
// }

build_instruction!(
    DOC_MAIN = "Verify a public key validity zero-knowledge proof.",
    DOC_AUX = "A public key validity proof certifies that an ElGamal public key is well-formed and the prover knows the corresponding secret key.",
    INSTRUCTION_NAME = VerifyPubkeyValidity,
    DISCRIMINATOR = 4,
    PROOF_LEN = PUBKEY_VALIDITY_PROOF_FULL_LEN
);
