#![cfg_attr(not(feature = "std"), no_std)]

//--------ORDUM FIRST ITERATION IMPLEMENTATION----------//

use ink_env::AccountId;
use ink_lang as ink;
use ink_primitives::{Key, KeyPtr};
use ink_storage::traits::{PackedAllocate, PackedLayout, SpreadLayout, StorageLayout};
use scale::{Decode, Encode};
use ink_prelude::{vec::Vec,vec,string::String};
use ink_storage::Mapping;
use core::hash::Hash;
use ink_storage::traits::SpreadAllocate;
use ink_types::Timestamp;


/// Constants
const MAX_KEYS:u8 = 3;

/// Error type for Create Profile
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    AccountExists,

    AccountDontExists,
    /// Any system related error
    UnexpectedError,
}


///  A grant applicant profile
#[derive(Clone,Encode,Default, Decode, Debug)]
#[cfg_attr(feature = "std",derive(StorageLayout,scale_info::TypeInfo))]
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
#[derive(Encode,Clone,Default, Decode, Debug)]
#[cfg_attr(feature = "std", derive(StorageLayout,scale_info::TypeInfo))]
pub struct IssuerProfile {
    name: String,
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
        chain: Option<String>,
        categories: Option<Vec<String>>,
        time: Timestamp,
        description: String

    ) -> CreateResult<Self> {

        Ok(Self{
            name,
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

    pub fn update_grant_status(&mut self, status: bool) -> CreateResult<()>{
        self.is_active = status;
        Ok(())
    }

}

/// Key management struct
/// This will allow multiple members in certain organization to manage the account
/// The allowed members will be granted by `admin` key
/// The `key_pointer` is the key used in the key to `IssuerProfile` mapping
#[derive(Clone,Encode,Hash, Decode, Debug)]
#[cfg_attr(feature = "std", derive(StorageLayout,scale_info::TypeInfo))]
#[derive(SpreadAllocate)]
pub struct KeyManagement{
    admin: AccountId,
    key_pointer: AccountId, // Account Id for now
    allowed_keys: Vec<AccountId>
}

#[derive(Clone,Encode, Decode, Debug)]
#[cfg_attr(feature = "std",derive(StorageLayout,scale_info::TypeInfo))]
pub enum KeyAction{
    ADD,
    REMOVE,
    ChangeAdmin
}

impl KeyManagement {
    pub fn new(admin: AccountId) ->CreateResult<()>{
        Self{
            admin,
            key_pointer: admin,
            allowed_keys: vec![admin],
        };
        Ok(())
    }
    pub fn update_keys_inner(&mut self, key: AccountId, action:KeyAction) {
       match action {
            KeyAction::ADD => {
                if !self.allowed_keys.contains(&key) {
                    self.allowed_keys.push(key);
                }else{
                    () // For the time being it does nothing, proper error handling will be introduced
                }
            },
            KeyAction::REMOVE => {
               if let Some(index) = self.allowed_keys.iter().position(|k| *k == key){
                   self.allowed_keys.remove(index);
               }else{
                   () // Does nothing, The era of Nothingness
               }
            },
           KeyAction::ChangeAdmin => {
                self.admin = key;
           }
        };

    }
}

// Required traits to work on custom data structures
impl SpreadLayout for KeyManagement {
    const FOOTPRINT: u64 = 3;

    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        Self {
            admin: SpreadLayout::pull_spread(ptr),
            key_pointer: SpreadLayout::pull_spread(ptr),
            allowed_keys: SpreadLayout::pull_spread(ptr),
        }
    }

    fn push_spread(&self, ptr: &mut KeyPtr) {
        SpreadLayout::push_spread(&self.admin,ptr);
        SpreadLayout::push_spread(&self.key_pointer,ptr);
        SpreadLayout::push_spread(&self.allowed_keys,ptr);

    }

    fn clear_spread(&self, ptr: &mut KeyPtr) {
        SpreadLayout::clear_spread(&self.admin, ptr);
        SpreadLayout::clear_spread(&self.key_pointer,ptr);
        SpreadLayout::clear_spread(&self.allowed_keys,ptr);

    }
}

impl SpreadLayout for IssuerProfile {
    const FOOTPRINT: u64 = 8;

    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        Self {
            name: SpreadLayout::pull_spread(ptr),
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
        SpreadLayout::push_spread(&self.chain, ptr);
        SpreadLayout::push_spread(&self.is_active, ptr);
        SpreadLayout::push_spread(&self.registration_date, ptr);
        SpreadLayout::push_spread(&self.categories, ptr);
        SpreadLayout::push_spread(&self.description, ptr);
        SpreadLayout::push_spread(&self.applications, ptr);

    }

    fn clear_spread(&self, ptr: &mut KeyPtr) {
        SpreadLayout::clear_spread(&self.name, ptr);
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
        PackedLayout::pull_packed(&mut self.chain, at);
        PackedLayout::pull_packed(&mut self.is_active, at);
        PackedLayout::pull_packed(&mut self.registration_date, at);
        PackedLayout::pull_packed(&mut self.categories, at);
        PackedLayout::pull_packed(&mut self.description, at);
        PackedLayout::pull_packed(&mut self.applications, at);

    }

    fn push_packed(&self, at: &Key) {
        PackedLayout::push_packed(&self.name, at);
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
        PackedLayout::pull_packed(&mut self.key_pointer, at);
        PackedLayout::pull_packed(&mut self.allowed_keys, at);

    }

    fn push_packed(&self, at: &Key) {
        PackedLayout::push_packed(&self.admin, at);
        PackedLayout::push_packed(&self.key_pointer,at);
        PackedLayout::push_packed(&self.allowed_keys,at);

    }

    fn clear_packed(&self, at: &Key) {
        PackedLayout::clear_packed(&self.admin,at);
        PackedLayout::clear_packed(&self.key_pointer, at);
        PackedLayout::clear_packed(&self.allowed_keys, at);

    }
}
impl PackedAllocate for KeyManagement {
    fn allocate_packed(&mut self, at: &Key) {
        PackedAllocate::allocate_packed(&mut self.admin,at);
        PackedAllocate::allocate_packed(&mut self.key_pointer, at);
        PackedAllocate::allocate_packed(&mut self.allowed_keys, at);
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
    ///
    /// Allowed Accounts act as privileged members that can control the account `Multi-Key system`
    /// In Phala context this function will be dispatched following block production
    /// as it takes in `&mut self`.
    #[ink(message, selector =0xC0DE0002)]
    fn create_issuer_profile(

        &mut self, name: String,
        chain: Option<String>,
        categories: Option<Vec<String>>,
        description: String,
        allowed_accounts: Vec<AccountId>

    ) -> CreateResult<()>;

    /// Adding and removing allowed accounts by the `admin`
    /// This will allow not only one person who is privileged to manage an account but also
    /// multiple allowed accounts
    #[ink(message, selector = 0xC0DE0003)]
    fn update_keys(&mut self,account: AccountId,action: KeyAction);
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
    use crate::{CreateResult, KeyAction, KeyManagement};
    use super::{Vec,vec,CreateProfile,String, IssuerProfile,ApplicantProfile, Error};


    /// Ordum Global State
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct OrdumState {
        //Experinmental Multi-Key management
        issuer_profile: Mapping<AccountId,IssuerProfile>,
        applicant_profile: Mapping<AccountId,ApplicantProfile>,
        // Multi-Key Management
        manage_keys: Vec<KeyManagement>,
    }


    /// Events to be used on Notifications
    /// Event emitted when new Grant Issuer is registered
        #[ink(event)]
        pub struct IssuerAccountCreated {
            #[ink(topic)]
            name: String,
            time: Timestamp
        }
    /// Event emitted when new Applicant is registered
        #[ink(event)]
        pub struct ApplicantAccountCreated {
            #[ink(topic)]
            name: String,
            time:  Timestamp
        }
    /// Event emitted when Grant Issuer updates the profile
    #[ink(event)]
    pub struct IssuerUpdated {
        #[ink(topic)]
        name: String,
        time: Timestamp
    }
    /// Event emitted when Applicant updates the profile
        #[ink(event)]
        pub struct ApplicantUpdated {
            #[ink(topic)]
            name: String,
            time: Timestamp
        }

    impl OrdumState {

        #[ink(constructor)]
        pub fn initialize() -> Self{
            initialize_contract(|state:&mut Self|{
                let contract_id = Self::env().account_id();
                let initializer_id = Self::env().caller();
                let initial_keys = KeyManagement {
                    admin: contract_id,
                    key_pointer: contract_id,
                    allowed_keys: vec![contract_id,initializer_id],
                };

                state.issuer_profile.insert(contract_id,&IssuerProfile::default());
                state.manage_keys = vec![initial_keys];
                state.applicant_profile.insert(initializer_id,&ApplicantProfile::default());
            })
        }
        // Default constructor
        #[ink(constructor)]
        pub fn default() -> Self{
            initialize_contract(|_| {})
        }

        // Getters
        /*#[ink(message,selector=0xC0DE1002)]
        pub fn get_issuer_profile(&self, account: AccountId) -> CreateResult<IssuerProfile>{
            if self.issuer_profile.contains(account){
                Ok(self.issuer_profile.get(account).unwrap().unwrap())
            }else{
                Err(Error::AccountDontExists)
            }
        }*/

        #[ink(message,selector=0xC0DE1001)]
        pub fn get_app_profile(&self, account: AccountId) -> CreateResult<ApplicantProfile>{
            if self.applicant_profile.contains(account){
                Ok(self.applicant_profile.get(account).unwrap())
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
                self.applicant_profile.insert(&applicant,&appl_data);

                Ok(())
            }else {
                Err(Error::AccountExists)
            }
        }

        #[ink(message, selector =0xC0DE0002)]
        fn create_issuer_profile(
            &mut self, name: String,
            chain: Option<String>,
            categories: Option<Vec<String>>,
            description: String,
            allowed_accounts: Vec<AccountId>
        ) -> CreateResult<()> {

            let issuer_admin = Self::env().caller();

            // Checking if the account is already registered

            if let Some(_key) =  self.manage_keys.iter()
                .find(|&k| k.admin == issuer_admin){
                return Err(Error::AccountExists)?;

                // If not then create a new one
            }else {
                let time = Self::env().block_timestamp();
                let profile = IssuerProfile::new(name,chain,categories,time,description)
                    .map_err(|_|Error::UnexpectedError)?;

                let wallet = KeyManagement {
                    admin: issuer_admin,
                    key_pointer: issuer_admin,
                    allowed_keys: allowed_accounts
                };


                // Registering to the storage
                self.issuer_profile.insert(wallet.key_pointer,&profile);
                self.manage_keys.push(wallet);

                // Emit an event
                /*ink_env::emit_event(IssuerAccountCreated {
                    name: profile.clone().name,
                    time,
                });*/
                Ok(())
            }

        }
        #[ink(message, selector = 0xC0DE0003)]
        fn update_keys(&mut self, account: AccountId, action: KeyAction) {
            let caller = Self::env().caller();
            // check if the account is registered as admin
            // Iterating over KeyManagement object and checking admin value
            let result = self.manage_keys.clone().into_iter().find_map(|wallet| {
                if wallet.admin == caller {
                    Some(wallet)
                } else {
                    None
                }
            });
            match result {
                Some(mut acc_manage) => {
                   acc_manage.update_keys_inner(account,action);
                },
                None => ()
            }
        }

    }
}
