use crate::{
    build_and_invoke_instruction, create_instruction_struct, write_bytes, ContextStateInfo, Proof,
    UNINIT_BYTE, ZERO_CIPHERTEXT_PROOF_FULL_LEN,
};
use core::slice::from_raw_parts;
use solana_instruction_view::InstructionAccount;
use solana_program_error::ProgramResult;

create_instruction_struct!(
    DOC_MAIN = "Verify a zero-ciphertext proof.",
    DOC_AUX =
        "A zero-ciphertext proof certifies that an ElGamal ciphertext encrypts the value zero.",
    INSTRUCTION_NAME = VerifyZeroCiphertext,
    DISCRIMINATOR = 1,
    PROOF_LEN = ZERO_CIPHERTEXT_PROOF_FULL_LEN
);
