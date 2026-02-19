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

#[derive(Clone, Debug, PartialEq)]
pub enum Proof<'a, const PROOF_LEN: usize> {
    Account {
        account: &'a AccountView,
        offset: u32,
    },
    Data(&'a [u8; PROOF_LEN]),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContextStateInfo<'a> {
    pub context_state_account: &'a AccountView,
    pub context_state_authority: &'a AccountView,
}

pub struct VerifyPubkeyValidity<'a, 'b> {
    pub context_state_info: Option<ContextStateInfo<'a>>,
    pub proof: Proof<'b, PUBKEY_VALIDITY_PROOF_FULL_LEN>,
}
impl VerifyPubkeyValidity<'_, '_> {
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        match self.proof {
            Proof::Account {
                account: proof_account,
                offset,
            } => {
                // 1 byte (discriminator) + offset (4 bytes, u32)
                let mut instruction_data = [UNINIT_BYTE; 1 + 4];
                instruction_data[0].write(4);
                write_bytes(&mut instruction_data[1..], &offset.to_le_bytes());
                let instruction_data =
                    unsafe { from_raw_parts(instruction_data.as_ptr() as _, 1 + 4) };

                if let Some(ref context_state_info) = self.context_state_info {
                    let instruction_accounts: [InstructionAccount; 3] = [
                        InstructionAccount::readonly(proof_account.address()),
                        InstructionAccount::writable(
                            context_state_info.context_state_account.address(),
                        ),
                        InstructionAccount::readonly(
                            context_state_info.context_state_authority.address(),
                        ),
                    ];

                    build_and_invoke_instruction(
                        &instruction_accounts,
                        instruction_data,
                        &[
                            proof_account,
                            context_state_info.context_state_account,
                            context_state_info.context_state_authority,
                        ],
                    )
                } else {
                    let instruction_accounts: [InstructionAccount; 1] =
                        [InstructionAccount::readonly(proof_account.address())];

                    build_and_invoke_instruction(
                        &instruction_accounts,
                        instruction_data,
                        &[proof_account],
                    )
                }
            }
            Proof::Data(proof_data) => {
                // 1 byte (discriminator) + proof length bytes
                let mut instruction_data = [UNINIT_BYTE; 1 + PUBKEY_VALIDITY_PROOF_FULL_LEN];
                instruction_data[0].write(4);
                write_bytes(&mut instruction_data[1..], proof_data);
                let instruction_data = unsafe {
                    from_raw_parts(
                        instruction_data.as_ptr() as _,
                        1 + PUBKEY_VALIDITY_PROOF_FULL_LEN,
                    )
                };

                if let Some(ref context_state_info) = self.context_state_info {
                    let instruction_accounts: [InstructionAccount; 2] = [
                        InstructionAccount::writable(
                            context_state_info.context_state_account.address(),
                        ),
                        InstructionAccount::readonly(
                            context_state_info.context_state_authority.address(),
                        ),
                    ];

                    build_and_invoke_instruction(
                        &instruction_accounts,
                        instruction_data,
                        &[
                            context_state_info.context_state_account,
                            context_state_info.context_state_authority,
                        ],
                    )
                } else {
                    build_and_invoke_instruction(&[], instruction_data, &[])
                }
            }
        }
    }
}

#[inline(always)]
fn build_and_invoke_instruction<const ACCOUNTS: usize>(
    accounts: &[InstructionAccount],
    data: &[u8],
    account_views: &[&AccountView; ACCOUNTS],
) -> ProgramResult {
    let instruction = InstructionView {
        program_id: &crate::ID,
        accounts,
        data,
    };
    invoke(&instruction, account_views)
}
