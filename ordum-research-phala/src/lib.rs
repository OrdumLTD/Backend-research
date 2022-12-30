#![cfg_attr(not(feature = "std"), no_std)]

//--------ORDUM FIRST ITERATION IMPLEMENTATION----------//

use ink_lang as ink;

/// Error type for Create Profile
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CreateProfileError {
    AccountExists,
}

/// Result type for Create Profile
pub type CreateResult<T> = Result<T,CreateProfileError>;

/*/// Trait definition for Creating a Profile
/// This trait can be used both at individual, institutional applicants or grants issuer
#[ink::trait_definition]
pub trait CreateProfile {

    /*#[ink(message)]
    fn create_profile() -> CreateResult<()>{
        todo!()
    }

    #[ink(message)]
    fn update_profile() -> CreateResult<()>{
        todo!()
    }*/
}*/



#[ink::contract]
mod ordum{
    use super::*;


    /// Ordum Global State
    #[ink(storage)]
    pub struct OrdumState {

    }

    impl OrdumState {

        #[ink(constructor)]
        pub fn initialize() -> Self{
            todo!()
        }
        #[ink(message)]
        pub fn test(&self){
            todo!()
        }
    }
    //impl CreateProfile for OrdumState{}
}
