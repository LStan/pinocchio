use crate::{
    build_and_invoke_instruction, create_instruction_struct, write_bytes, ContextStateInfo, Proof,
    PERCENTAGE_WITH_CAP_PROOF_FULL_LEN, UNINIT_BYTE,
};
use core::slice::from_raw_parts;
use solana_instruction_view::InstructionAccount;
use solana_program_error::ProgramResult;

create_instruction_struct!(
    DOC_MAIN = "Verify a percentage-with-cap proof.",
    DOC_AUX = "A percentage-with-cap proof certifies that a tuple of Pedersen commitments satisfy a percentage relation.",
    INSTRUCTION_NAME = VerifyPercentageWithCap,
    DISCRIMINATOR = 5,
    PROOF_LEN = PERCENTAGE_WITH_CAP_PROOF_FULL_LEN
);
