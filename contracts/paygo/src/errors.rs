use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    CompanyNotFound = 1,
    InsufficientFunds = 2,
    Unauthorized = 3,
    CompanyAlreadyExists = 4,
}
