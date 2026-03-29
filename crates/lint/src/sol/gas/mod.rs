use crate::sol::{EarlyLintPass, LateLintPass, SolLint};

mod custom_errors;
mod increment;
mod keccak;

use custom_errors::CUSTOM_ERRORS;
use increment::POST_INCREMENT_DECREMENT;
use keccak::ASM_KECCAK256;

register_lints!(
    (CustomErrors, early, (CUSTOM_ERRORS)),
    (AsmKeccak256, late, (ASM_KECCAK256)),
    (PostIncrementDecrement, early, (POST_INCREMENT_DECREMENT))
);