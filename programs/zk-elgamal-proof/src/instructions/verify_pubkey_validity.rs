use core::slice::from_raw_parts;

use solana_instruction_view::InstructionAccount;
use solana_program_error::ProgramResult;

use crate::{
    build_and_invoke_instruction, create_instruction_struct, write_bytes, ContextStateInfo, Proof,
    PUBKEY_VALIDITY_PROOF_FULL_LEN, UNINIT_BYTE,
};

create_instruction_struct!(
    DOC_MAIN = "Verify a public key validity zero-knowledge proof.",
    DOC_AUX = "A public key validity proof certifies that an ElGamal public key is well-formed and the prover knows the corresponding secret key.",
    INSTRUCTION_NAME = VerifyPubkeyValidity,
    DISCRIMINATOR = 4,
    PROOF_LEN = PUBKEY_VALIDITY_PROOF_FULL_LEN
);
