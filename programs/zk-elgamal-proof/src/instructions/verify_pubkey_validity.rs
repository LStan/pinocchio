use core::slice::from_raw_parts;

use solana_account_view::AccountView;
use solana_instruction_view::{cpi::invoke, InstructionAccount, InstructionView};
use solana_program_error::ProgramResult;

use crate::{build_instruction, write_bytes, PUBKEY_VALIDITY_PROOF_FULL_LEN, UNINIT_BYTE};

build_instruction!(
    DOC_MAIN = "Verify a public key validity zero-knowledge proof.",
    DOC_AUX = "A public key validity proof certifies that an ElGamal public key is well-formed and the prover knows the corresponding secret key.",
    INSTRUCTION_NAME = VerifyPubkeyValidity,
    DISCRIMINATOR = 4,
    PROOF_LEN = PUBKEY_VALIDITY_PROOF_FULL_LEN
);
