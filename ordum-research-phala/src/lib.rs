#![cfg_attr(not(feature = "std"), no_std)]

//--------ORDUM FIRST ITERATION IMPLEMENTATION----------//

use ink_env::AccountId;
use ink_lang as ink;
use ink_primitives::{Key, KeyPtr};
use ink_storage::traits::{PackedLayout, SpreadLayout, StorageLayout};
use scale::{Decode, Encode};
use ink_prelude::{string::String};


/// Error type for Create Profile
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    AccountExists,
    AccountDontExists,
    UnexpectedError,
}


///  A grant applicant profile
#[derive(Clone,Encode, Decode, Debug)]
#[cfg_attr(feature = "std",
derive(StorageLayout,scale_info::TypeInfo))]
pub struct ApplicantProfile {
    name: String,
    team_size: u8,
    account_id: AccountId,
    description: String,
}

impl ApplicantProfile {
    pub fn new(
        name: String, team_size:u8,
        description: String,
        account: AccountId
    ) -> CreateResult<Self> {

        Ok(Self {
            name,
            team_size,
            account_id: account,
            description,
        })
    }
}

/// A grant issuer profile
#[derive(Clone,Encode, Decode, Debug)]
#[cfg_attr(feature = "std",
 derive(StorageLayout,scale_info::TypeInfo))]
pub struct IssuerProfile {
    name: String,
    description: String,
}

impl IssuerProfile {
    pub fn new(name: String, description: String) -> CreateResult<Self> {
        Ok(Self{
            name,
            description,
        })
    }
    pub fn update_description(&mut self,description: String) -> CreateResult<()> {
        self.description = description;
        Ok(())
    }

}

// Required traits to work on custom data structures
impl SpreadLayout for IssuerProfile {
    const FOOTPRINT: u64 = 2;

    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        Self {
            name: SpreadLayout::pull_spread(ptr),
            description: SpreadLayout::pull_spread(ptr),
        }
    }

    fn push_spread(&self, ptr: &mut KeyPtr) {
        SpreadLayout::push_spread(&self.name, ptr);
        SpreadLayout::push_spread(&self.description, ptr);
    }

    fn clear_spread(&self, ptr: &mut KeyPtr) {
        SpreadLayout::clear_spread(&self.name, ptr);
        SpreadLayout::clear_spread(&self.description, ptr);
    }
}
impl SpreadLayout for ApplicantProfile {
    const FOOTPRINT: u64 = 4;

    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        Self {
            name: SpreadLayout::pull_spread(ptr),
            description: SpreadLayout::pull_spread(ptr),
            team_size: SpreadLayout::pull_spread(ptr),
            account_id: SpreadLayout::pull_spread(ptr),
        }
    }

    fn push_spread(&self, ptr: &mut KeyPtr) {
        SpreadLayout::push_spread(&self.name, ptr);
        SpreadLayout::push_spread(&self.description, ptr);
        SpreadLayout::push_spread(&self.team_size, ptr);
        SpreadLayout::push_spread(&self.account_id, ptr);
    }

    fn clear_spread(&self, ptr: &mut KeyPtr) {
        SpreadLayout::clear_spread(&self.name, ptr);
        SpreadLayout::clear_spread(&self.description, ptr);
        SpreadLayout::clear_spread(&self.account_id,ptr);
        SpreadLayout::clear_spread(&self.team_size,ptr);
    }

}

impl PackedLayout for ApplicantProfile {
    fn pull_packed(&mut self, at: &Key) {
        PackedLayout::pull_packed(&mut self.name, at);
        PackedLayout::pull_packed(&mut self.description, at);
        PackedLayout::pull_packed(&mut self.team_size, at);
        PackedLayout::pull_packed(&mut self.account_id, at);
    }

    fn push_packed(&self, at: &Key) {
        PackedLayout::push_packed(&self.name, at);
        PackedLayout::push_packed(&self.description, at);
        PackedLayout::push_packed(&self.account_id, at);
        PackedLayout::push_packed(&self.team_size, at);
    }

    fn clear_packed(&self, at: &Key) {
        PackedLayout::clear_packed(&self.name, at);
        PackedLayout::clear_packed(&self.description, at);
        PackedLayout::clear_packed(&self.team_size, at);
        PackedLayout::clear_packed(&self.account_id, at);
    }

}

impl PackedLayout for IssuerProfile {
    fn pull_packed(&mut self, at: &Key) {
        PackedLayout::pull_packed(&mut self.name, at);
        PackedLayout::pull_packed(&mut self.description, at);
    }

    fn push_packed(&self, at: &Key) {
        PackedLayout::push_packed(&self.name, at);
        PackedLayout::push_packed(&self.description, at);
    }

    fn clear_packed(&self, at: &Key) {
        PackedLayout::clear_packed(&self.name, at);
        PackedLayout::clear_packed(&self.description, at);
    }
}

//---------------------------------------------------------------------//

/// Result type for Create Profile
pub type CreateResult<T> = Result<T,Error>;

/// Trait definition for Creating a Profile
/// This trait can be used both at individual, institutional applicants or grants issuer
#[ink::trait_definition]
pub trait CreateProfile {

    #[ink(message)]
    fn create_appl_profile(
        &mut self, name: String,
        account: AccountId,
        size: u8, description: String
    ) -> CreateResult<()>;

    #[ink(message)]
    fn create_issuer_profile(
        &mut self, name: String,
        description: String
    ) -> CreateResult<()>;

    /*/// Experinmental feature for Grant Issuers and Applicants
    /// Signup using email
    #[ink(message)]
    fn signup_applicant() -> CreateResult<()>;

    /// Experinmental
    #[ink(message)]
    fn signup_issuer() -> CreateResult<()>;

    #[ink(message)]
    fn update_profile() -> CreateResult<()>;*/
}



// ----------CONTRACT IMPLEMENTATION--------------------------------------//

#[ink::contract]
mod ordum{

    use ink_lang::utils::initialize_contract;
    use ink_storage::Mapping;
    use ink_storage::traits::SpreadAllocate;
    use crate::CreateResult;
    use super::{CreateProfile,String, IssuerProfile,ApplicantProfile, Error};


    /// Ordum Global State
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct OrdumState {
        issuer_profile: Mapping<AccountId,Option<IssuerProfile>>,
        applicant_profile: Mapping<AccountId,Option<ApplicantProfile>>
    }

    impl OrdumState {

        #[ink(constructor)]
        pub fn initialize() -> Self{
            initialize_contract(|state:&mut Self|{
                let contract_id = Self::env().account_id();
                let initializer_id = Self::env().caller();
                state.issuer_profile.insert(contract_id,&None::<IssuerProfile>);
                state.applicant_profile.insert(initializer_id,&None::<ApplicantProfile>);
            })
        }
        // Default constructor
        #[ink(constructor)]
        pub fn default() -> Self{
            initialize_contract(|_| {})
        }

        // Getters
        #[ink(message)]
        pub fn get_issuer_profile(&self, account: AccountId) -> CreateResult<IssuerProfile>{
            if self.issuer_profile.contains(account){
                Ok(self.issuer_profile.get(account).unwrap().unwrap())
            }else{
                Err(Error::AccountDontExists)
            }
        }

        #[ink(message)]
        pub fn get_app_profile(&self, account: AccountId) -> CreateResult<ApplicantProfile>{
            if self.applicant_profile.contains(account){
                Ok(self.applicant_profile.get(account).unwrap().unwrap())
            }else{
                Err(Error::AccountDontExists)
            }
        }

    }

    impl CreateProfile for OrdumState {
        #[ink(message)]
        fn create_appl_profile(
            &mut self, name: String,
            account: AccountId, size: u8,
            description: String
        ) -> CreateResult<()> {

            if !self.applicant_profile.contains(account){

                let applicant = Self::env().caller();
                let appl_data = ApplicantProfile::new(name,size,description,account)
                    .map_err(|_|Error::UnexpectedError)?;
                self.applicant_profile.insert(&applicant,&Some(appl_data));

                Ok(())
            }else {
                Err(Error::AccountExists)
            }
        }

        #[ink(message)]
        fn create_issuer_profile(
            &mut self, name: String,
            description: String

        ) -> CreateResult<()> {

            let issuer = Self::env().caller();
            if !self.issuer_profile.contains(issuer){

                let issuer_data = IssuerProfile::new(name,description)
                    .map_err(|_|Error::UnexpectedError)?;
                self.issuer_profile.insert(&issuer,&Some(issuer_data));
                Ok(())

            }else {
                return Err(Error::AccountExists)
            }

        }

        /*fn signup_applicant() -> CreateResult<()> {
            todo!()
        }

        fn signup_issuer() -> CreateResult<()> {
            todo!()
        }

        fn update_profile() -> CreateResult<()> {
            todo!()
        }*/
    }
}
