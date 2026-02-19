use crate::{
    build_and_invoke_instruction, create_instruction_struct, write_bytes, ContextStateInfo, Proof,
    CIPHERTEXT_COMMITMENT_EQUALITY_PROOF_FULL_LEN, UNINIT_BYTE,
};
use core::slice::from_raw_parts;
use solana_instruction_view::InstructionAccount;
use solana_program_error::ProgramResult;

create_instruction_struct!(
    DOC_MAIN = "Verify a ciphertext-commitment equality proof.",
    DOC_AUX = "A ciphertext-commitment equality proof certifies that an ElGamal ciphertext and a Pedersen commitment encrypt/encode the same message.",
    INSTRUCTION_NAME = VerifyCiphertextCommitmentEquality,
    DISCRIMINATOR = 3,
    PROOF_LEN = CIPHERTEXT_COMMITMENT_EQUALITY_PROOF_FULL_LEN
);
