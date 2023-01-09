#![cfg_attr(not(feature = "std"), no_std)]

//--------ORDUM FIRST ITERATION IMPLEMENTATION----------//

use ink_env::AccountId;
use ink_lang as ink;
use ink_primitives::{Key, KeyPtr};
use ink_storage::traits::{PackedLayout, SpreadLayout, StorageLayout};
use scale::{Decode, Encode};
use ink_prelude::{vec::Vec,vec,string::String};
use ink_storage::Mapping;
use ink_types::Timestamp;


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
/// The order is important in Contract Upgrades
#[derive(Encode, Decode, Debug)]
#[cfg_attr(feature = "std",
 derive(StorageLayout,scale_info::TypeInfo))]
#[derive(SpreadAllocate)]
pub struct IssuerProfile {
    name: String,
    grant_levels: Option<Mapping<u8,u32>>,
    chain: Option<String>,
    is_active: bool,
    registration_date: Timestamp,
    categories: Option<Vec<String>>,
    description: String,
    applications: Option<Vec<u16>>,

}

impl IssuerProfile {
    pub fn new(
        name: String,
        grant_levels: Option<Mapping<u8,u32>>,
        chain: Option<String>,
        categories: Option<Vec<String>>,
        time: Timestamp,
        description: String

    ) -> CreateResult<Self> {

        Ok(Self{
            name,
            grant_levels,
            chain,
            is_active: true,
            registration_date: time,
            categories,
            description,
            applications: None,
        })
    }
    pub fn update_description(&mut self,description: String) -> CreateResult<()> {
        self.description = description;
        Ok(())
    }
    pub fn update_grant_levels(&mut self, grant_levels:Option<Mapping<u8,u32>>) -> CreateResult<()> {
        self.grant_levels = grant_levels;
        Ok(())
    }
    pub fn update_grant_status(&mut self, status: bool) -> CreateResult<()>{
        self.is_active = status;
        Ok(())
    }

}

/// Key management struct
/// This will allow multiple members in certain organization to issue transactions
/// The allowed members will be granted by key `admin`
#[derive(Clone,Encode, Decode, Debug)]
#[cfg_attr(feature = "std",
derive(StorageLayout,scale_info::TypeInfo))]
pub struct KeyManagement{
    admin: AccountId,
    allowed_keys: Vec<AccountId>
}

#[derive(Clone,Encode, Decode, Debug)]
#[cfg_attr(feature = "std",
derive(StorageLayout,scale_info::TypeInfo))]
pub enum KeyAction{
    ADD,
    REMOVE
}

impl KeyManagement {
    pub fn new(admin: AccountId) ->CreateResult<()>{
        Self{
            admin,
            allowed_keys: vec![admin],
        };
        Ok(())
    }
    pub fn update_keys(&mut self,key: AccountId, action:KeyAction) -> CreateResult<()> {
        match action {
            KeyAction::ADD => {self.allowed_keys.push(key); Ok(()) },
            KeyAction::REMOVE => {self.allowed_keys.push(key); Ok(())}
        }
    }
}

// Required traits to work on custom data structures
impl SpreadLayout for KeyManagement {
    const FOOTPRINT: u64 = 2;

    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        Self {
            admin: SpreadLayout::pull_spread(ptr),
            allowed_keys: SpreadLayout::pull_spread(ptr),
        }
    }

    fn push_spread(&self, ptr: &mut KeyPtr) {
        SpreadLayout::push_spread(&self.admin,ptr);
        SpreadLayout::push_spread(&self.allowed_keys,ptr);
    }

    fn clear_spread(&self, ptr: &mut KeyPtr) {
        SpreadLayout::clear_spread(&self.admin, ptr);
        SpreadLayout::clear_spread(&self.allowed_keys,ptr);
    }
}

impl SpreadLayout for IssuerProfile {
    const FOOTPRINT: u64 = 8;

    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        Self {
            name: SpreadLayout::pull_spread(ptr),
            grant_levels: SpreadLayout::pull_spread(ptr),
            chain: SpreadLayout::pull_spread(ptr),
            is_active: SpreadLayout::pull_spread(ptr),
            registration_date: SpreadLayout::pull_spread(ptr),
            categories: SpreadLayout::pull_spread(ptr),
            description: SpreadLayout::pull_spread(ptr),
            applications: SpreadLayout::pull_spread(ptr),
        }
    }

    fn push_spread(&self, ptr: &mut KeyPtr) {
        SpreadLayout::push_spread(&self.name, ptr);
        SpreadLayout::push_spread(&self.grant_levels, ptr);
        SpreadLayout::push_spread(&self.chain, ptr);
        SpreadLayout::push_spread(&self.is_active, ptr);
        SpreadLayout::push_spread(&self.registration_date, ptr);
        SpreadLayout::push_spread(&self.categories, ptr);
        SpreadLayout::push_spread(&self.description, ptr);
        SpreadLayout::push_spread(&self.applications, ptr);

    }

    fn clear_spread(&self, ptr: &mut KeyPtr) {
        SpreadLayout::clear_spread(&self.name, ptr);
        SpreadLayout::clear_spread(&self.grant_levels, ptr);
        SpreadLayout::clear_spread(&self.chain, ptr);
        SpreadLayout::clear_spread(&self.is_active, ptr);
        SpreadLayout::clear_spread(&self.registration_date, ptr);
        SpreadLayout::clear_spread(&self.categories, ptr);
        SpreadLayout::clear_spread(&self.description, ptr);
        SpreadLayout::clear_spread(&self.applications, ptr);

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
        PackedLayout::pull_packed(&mut self.grant_levels, at);
        PackedLayout::pull_packed(&mut self.chain, at);
        PackedLayout::pull_packed(&mut self.is_active, at);
        PackedLayout::pull_packed(&mut self.registration_date, at);
        PackedLayout::pull_packed(&mut self.categories, at);
        PackedLayout::pull_packed(&mut self.description, at);
        PackedLayout::pull_packed(&mut self.applications, at);

    }

    fn push_packed(&self, at: &Key) {
        PackedLayout::push_packed(&self.name, at);
        PackedLayout::push_packed(&self.grant_levels, at);
        PackedLayout::push_packed(&self.chain, at);
        PackedLayout::push_packed(&self.is_active, at);
        PackedLayout::push_packed(&self.registration_date, at);
        PackedLayout::push_packed(&self.categories, at);
        PackedLayout::push_packed(&self.description, at);
        PackedLayout::push_packed(&self.applications, at);

    }

    fn clear_packed(&self, at: &Key) {
        PackedLayout::clear_packed(&self.name, at);
        PackedLayout::clear_packed(&self.description, at);
        PackedLayout::clear_packed(&self.grant_levels, at);
        PackedLayout::clear_packed(&self.chain, at);
        PackedLayout::clear_packed(&self.registration_date, at);
        PackedLayout::clear_packed(&self.categories, at);
        PackedLayout::clear_packed(&self.is_active, at);
        PackedLayout::clear_packed(&self.applications, at);

    }
}

impl PackedLayout for KeyManagement {
    fn pull_packed(&mut self, at: &Key) {
        PackedLayout::pull_packed(&mut self.admin, at);
        PackedLayout::pull_packed(&mut self.allowed_keys, at);
    }

    fn push_packed(&self, at: &Key) {
        PackedLayout::push_packed(&self.admin, at);
        PackedLayout::push_packed(&self.allowed_keys,at);
    }

    fn clear_packed(&self, at: &Key) {
        PackedLayout::clear_packed(&self.admin,at);
        PackedLayout::clear_packed(&self.allowed_keys, at);
    }
}
//---------------------------------------------------------------------//


/// Result type for Create Profile
pub type CreateResult<T> = Result<T,Error>;

/// Trait definition for Creating a Profile
/// This trait can be used both at individual, institutional applicants or grants issuer
#[ink::trait_definition]
pub trait CreateProfile {

    /// Creates Applicant Profile,a function which takes on `name: String`
    ///  `account: AccountId`, `size: u8`, `description: String`
    /// In Phala context this function will be dispatched following block production
    /// as it takes in `&mut self`.
    #[ink(message,selector =0xC0DE0001)]
    fn create_appl_profile(
        &mut self, name: String,
        account: AccountId,
        team_size: u8, description: String
    ) -> CreateResult<()>;



    /// Creates Grant Issuer Profile, a function which takes on `name: String`,
    /// `grant_levels`: This is an optional parameter whereby Issuer can choose different levels
    /// of grants to provide based on amount, `chain type if the grants is an on-chain type and
    /// None if its Off-chain`, `categories`: This specifies which categories this grant is on.
    /// `description`: extra details of the grants.
    /// In Phala context this function will be dispatched following block production
    /// as it takes in `&mut self`.
    #[ink(message, selector =0xC0DE0002)]
    fn create_issuer_profile(
        &mut self, name: String,
        grant_levels: Option<Mapping<u8,u32>>,
        chain: Option<String>,
        categories: Option<Vec<String>>,
        description: String,

    ) -> CreateResult<()>;

    /*
    #[ink(message)]
    fn update_profile() -> CreateResult<()>;*/
}



// ----------CONTRACT IMPLEMENTATION--------------------------------------//

#[ink::contract]
mod ordum{

    use ink_lang::utils::initialize_contract;
    use ink_storage::Mapping;
    use ink_storage::traits::SpreadAllocate;
    use ink_types::Timestamp;
    use pink_extension::AccountId;
    use crate::{CreateResult, KeyManagement};
    use super::{CreateProfile,String, IssuerProfile,ApplicantProfile, Error};


    /// Ordum Global State
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct OrdumState {
        issuer_profile: Mapping<AccountId,Option<IssuerProfile>>,
        applicant_profile: Mapping<AccountId,Option<ApplicantProfile>>,
        // Key management
        //allowed_keys: KeyManagement,

    }


    /// Events to be used on Notifications
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Events{
        IssuerAccountCreated {
            account: AccountId,
            time: Timestamp
        },
        ApplicantAccountCreated {
            account: AccountId,
            time:  Timestamp
        },
        ApplicantUpdated

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
        #[ink(message,selector=0xC0DE1002)]
        pub fn get_issuer_profile(&self, account: AccountId) -> CreateResult<IssuerProfile>{
            if self.issuer_profile.contains(account){
                Ok(self.issuer_profile.get(account).unwrap().unwrap())
            }else{
                Err(Error::AccountDontExists)
            }
        }

        #[ink(message,selector=0xC0DE1001)]
        pub fn get_app_profile(&self, account: AccountId) -> CreateResult<ApplicantProfile>{
            if self.applicant_profile.contains(account){
                Ok(self.applicant_profile.get(account).unwrap().unwrap())
            }else{
                Err(Error::AccountDontExists)
            }
        }

    }

    impl CreateProfile for OrdumState {
        #[ink(message,selector =0xC0DE0001)]
        fn create_appl_profile(
            &mut self, name: String,
            account: AccountId, team_size: u8,
            description: String
        ) -> CreateResult<()> {

            if !self.applicant_profile.contains(account){

                let applicant = Self::env().caller();
                let appl_data = ApplicantProfile::new(name,team_size,description,account)
                    .map_err(|_|Error::UnexpectedError)?;
                self.applicant_profile.insert(&applicant,&Some(appl_data));

                Ok(())
            }else {
                Err(Error::AccountExists)
            }
        }

        #[ink(message, selector =0xC0DE0002)]
        fn create_issuer_profile(
            &mut self, name: String,
            grant_levels: Option<Mapping<u8,u32>>,
            chain: Option<String>,
            categories: Option<Vec<String>>,
            description: String,

        ) -> CreateResult<()> {

            let issuer = Self::env().caller();

            // Checking if the account is already registered
            if !self.issuer_profile.contains(issuer.clone()){
                let time = Self::env().block_timestamp();
                let issuer_data = IssuerProfile
                    ::new(name,grant_levels,chain,categories,time,description)
                    .map_err(|_|Error::UnexpectedError)?;

                self.issuer_profile.insert(&issuer,&Some(issuer_data));

                // Emiting an event
                Self::env().emit_event(Events::IssuerAccountCreated {
                    account: issuer,
                    time,
                });
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
