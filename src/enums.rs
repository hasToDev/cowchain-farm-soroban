use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum DataKey {
    Admin,
    InitializedLedger,
    NativeToken,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Status {
    Ok,
    Fail,
    AlreadyInitialized,
    NotInitialized,
    TryAgain,
    NotFound,
    Found,
    Saved,
    Bumped,
    Upgraded,
    Duplicate,
    InsufficientFund,
    Underage,
    MissingOwnership,
    FullStomach,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum CowBreed {
    Jersey = 1,
    Limousin = 2,
    Hallikar = 3,
    Hereford = 4,
    Holstein = 5,
    Simmental = 6,
}
