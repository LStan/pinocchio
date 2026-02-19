use {
    crate::{
        build_and_invoke_instruction, create_instruction_struct, write_bytes, ContextStateInfo,
        Proof, CIPHERTEXT_CIPHERTEXT_EQUALITY_PROOF_FULL_LEN, UNINIT_BYTE,
    },
    core::slice::from_raw_parts,
    solana_instruction_view::InstructionAccount,
    solana_program_error::ProgramResult,
};

create_instruction_struct!(
    DOC_MAIN = "Verify a ciphertext-ciphertext equality proof.",
    DOC_AUX = "A ciphertext-ciphertext equality proof certifies that two ElGamal ciphertexts \
               encrypt the same message.",
    INSTRUCTION_NAME = VerifyCiphertextCiphertextEquality,
    DISCRIMINATOR = 2,
    PROOF_LEN = CIPHERTEXT_CIPHERTEXT_EQUALITY_PROOF_FULL_LEN
);
