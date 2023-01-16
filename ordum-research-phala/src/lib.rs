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
    NotAuthorized,
    AccountDontExists,
    ProfileDontExists,
    MaxKeysExceeded,
    AccountExistsOrMaxExceeded,
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
    registered_time: Timestamp,
    applications:Option<u8>
}

impl ApplicantProfile {
    pub fn new(
        name: String, team_size:u8,
        description: String,
        account: AccountId,
        time: Timestamp

    ) -> CreateResult<Self> {

        Ok(Self {
            name,
            team_size,
            account_id: account,
            description,
            registered_time: time,
            applications: None
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
    pub fn new(admin: AccountId, accounts: Vec<AccountId>) ->CreateResult<()>{
        Self{
            admin,
            key_pointer: admin,
            allowed_keys: vec![admin],
        };
        Ok(())
    }
    pub fn update_keys_inner(&mut self, key: AccountId, action:KeyAction) -> CreateResult<()> {
       match action {
            KeyAction::ADD => {
                if !self.allowed_keys.contains(&key) && self.allowed_keys.len() as u8 <= MAX_KEYS {
                    self.allowed_keys.push(key);
                    Ok(())
                }else{
                    Err(Error::AccountExistsOrMaxExceeded)// For the time being it does nothing, proper error handling will be introduced
                }
            },
            KeyAction::REMOVE => {
               if let Some(index) = self.allowed_keys.iter().position(|k| *k == key){
                   self.allowed_keys.remove(index);
                   Ok(())
               }else{
                   Err(Error::AccountDontExists) // Does nothing, The era of Nothingness
               }
            },
           KeyAction::ChangeAdmin => {
                self.admin = key;
                Ok(())
           }
        }
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
    const FOOTPRINT: u64 = 6;

    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        Self {
            name: SpreadLayout::pull_spread(ptr),
            description: SpreadLayout::pull_spread(ptr),
            team_size: SpreadLayout::pull_spread(ptr),
            account_id: SpreadLayout::pull_spread(ptr),
            applications: SpreadLayout::pull_spread(ptr),
            registered_time: SpreadLayout::pull_spread(ptr),
        }
    }

    fn push_spread(&self, ptr: &mut KeyPtr) {
        SpreadLayout::push_spread(&self.name, ptr);
        SpreadLayout::push_spread(&self.description, ptr);
        SpreadLayout::push_spread(&self.team_size, ptr);
        SpreadLayout::push_spread(&self.account_id, ptr);
        SpreadLayout::push_spread(&self.applications, ptr);
        SpreadLayout::push_spread(&self.registered_time, ptr);

    }

    fn clear_spread(&self, ptr: &mut KeyPtr) {
        SpreadLayout::clear_spread(&self.name, ptr);
        SpreadLayout::clear_spread(&self.description, ptr);
        SpreadLayout::clear_spread(&self.account_id,ptr);
        SpreadLayout::clear_spread(&self.team_size,ptr);
        SpreadLayout::clear_spread(&self.applications,ptr);
        SpreadLayout::clear_spread(&self.registered_time,ptr);

    }

}

impl PackedLayout for ApplicantProfile {
    fn pull_packed(&mut self, at: &Key) {
        PackedLayout::pull_packed(&mut self.name, at);
        PackedLayout::pull_packed(&mut self.description, at);
        PackedLayout::pull_packed(&mut self.team_size, at);
        PackedLayout::pull_packed(&mut self.account_id, at);
        PackedLayout::pull_packed(&mut self.applications, at);
        PackedLayout::pull_packed(&mut self.registered_time, at);

    }

    fn push_packed(&self, at: &Key) {
        PackedLayout::push_packed(&self.name, at);
        PackedLayout::push_packed(&self.description, at);
        PackedLayout::push_packed(&self.account_id, at);
        PackedLayout::push_packed(&self.team_size, at);
        PackedLayout::push_packed(&self.applications, at);
        PackedLayout::push_packed(&self.registered_time, at);

    }

    fn clear_packed(&self, at: &Key) {
        PackedLayout::clear_packed(&self.name, at);
        PackedLayout::clear_packed(&self.description, at);
        PackedLayout::clear_packed(&self.team_size, at);
        PackedLayout::clear_packed(&self.account_id, at);
        PackedLayout::clear_packed(&self.applications, at);
        PackedLayout::clear_packed(&self.registered_time, at);

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
/// The function selector are in order `(eg, C0DE0001,C0DE0002,.. ) for all transactional functions
#[ink::trait_definition]
pub trait CreateProfile {

    /// Creates Applicant Profile,a function which takes on `name`
    ///  `optional applicant profile`, `team-size`, `description`
    /// The optional account act as Team's profile account. If not provided caller's
    /// account will be used as Team's profile account.
    ///
    /// In Phala context this function will be dispatched following block production
    /// as it takes in `&mut self`.
    #[ink(message,selector =0xC0DE0001)]
    fn create_applicant_profile(

        &mut self, name: String,
        account: Option<AccountId>,
        team_size: u8, description: String,
        allowed_accounts: Option<Vec<AccountId>>

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
    /// multiple allowed accounts. Max allowed accounts is a fixed constant [MAX_KEYS].
    ///
    /// Worst case scenario, time complexity will be `O(n)` with a best case of `O(1)`
    /// In Phala context this function will be dispatched following block production
    /// as it takes in `&mut self`.
    #[ink(message, selector = 0xC0DE0003)]
    fn update_keys(&mut self,account: AccountId,action: KeyAction) -> CreateResult<()>;


    /// Updating Grant Issuer with limitation of only `description`, `categories`,
    /// `chain`, `status` which can be updated to the profile.
    /// Any account member in `allowed accounts` have the privileges for updating.
    /// Worst case scenario, time complexity will be `O(n)` with a best case of `O(1)`
    ///
    /// In Phala context this function will be dispatched following block production
    /// as it takes in `&mut self`.
    #[ink(message,payable,selector = 0xC0DE0004)]
    fn update_issuer_profile(

        &mut self,description:Option<String>,
        categories: Option<Vec<String>>, // This replaces the existing categories
        chain: Option<Option<String>>,
        status: Option<bool>

    ) -> CreateResult<()>;

}



// ----------CONTRACT IMPLEMENTATION--------------------------------------//

#[ink::contract]
mod ordum {
    use ink_lang::utils::initialize_contract;
    use ink_storage::Mapping;
    use ink_storage::traits::SpreadAllocate;
    use crate::{CreateResult, KeyAction, KeyManagement, MAX_KEYS};
    use super::{Vec,vec,CreateProfile,String, IssuerProfile,ApplicantProfile, Error};


    /// Ordum Global State
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct OrdumState {
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
        #[ink(message,selector=0xC0DE1002)]
        pub fn get_issuer_profile(&self) -> CreateResult<IssuerProfile>{

            let caller = Self::env().caller();
            // Check if the caller is authorized to retrieve Applicant profile
            if let Some(wallet) = self.manage_keys.iter().find(|&key|{
                key.allowed_keys.contains(&caller)
            }){
                let profile = self.issuer_profile.get(wallet.key_pointer)
                    .ok_or(Error::UnexpectedError)?;
                Ok(profile)
            }else{
                Err(Error::AccountDontExists)
            }
        }

        #[ink(message,selector=0xC0DE1001)]
        pub fn get_applicant_profile(&self) -> CreateResult<ApplicantProfile>{

            let caller = Self::env().caller();
            // Check if the caller is authorized to retrieve Applicant profile
            if let Some(wallet) = self.manage_keys.iter().find(|&key|{
                key.allowed_keys.contains(&caller)
            }){
                let profile = self.applicant_profile.get(wallet.key_pointer)
                    .ok_or(Error::UnexpectedError)?;
                Ok(profile)
            }else{
                Err(Error::AccountDontExists)
            }

        }

    }

    impl CreateProfile for OrdumState {
        #[ink(message,selector =0xC0DE0001)]
        fn create_applicant_profile(
            &mut self, name: String,
            account: Option<AccountId>, team_size: u8,
            description: String,
            allowed_accounts: Option<Vec<AccountId>>
        ) -> CreateResult<()> {

            let applicant = Self::env().caller();
            let time = Self::env().block_timestamp();

            // Check if account is provided or else use applicant account
            if let Some(account_inner) = account {

                if let Some(mut allowed_acc) = allowed_accounts {
                    // Create KeyManagement
                    let mut wallet = KeyManagement {
                        admin: applicant,
                        key_pointer: account_inner,
                        allowed_keys: vec![applicant],
                    };
                    wallet.allowed_keys.append(&mut allowed_acc);

                    let applicant_data = ApplicantProfile::new(name,team_size,description,account_inner,time)
                        .map_err(|_|Error::UnexpectedError)?;
                    self.applicant_profile.insert(&wallet.key_pointer,&applicant_data);

                    // Register Keys
                    self.manage_keys.push(wallet);

                    Ok(())

                } else {
                    // If no allowed-accounts provided
                    // Create KeyManagement
                    let wallet = KeyManagement {
                        admin: applicant,
                        key_pointer: account_inner,
                        allowed_keys: vec![applicant],
                    };

                    let applicant_data = ApplicantProfile::new(name,team_size,description,account_inner,time)
                        .map_err(|_|Error::UnexpectedError)?;
                    self.applicant_profile.insert(&wallet.key_pointer,&applicant_data);

                    // Register Keys
                    self.manage_keys.push(wallet);

                    Ok(())
                }
            } else {
                // If no account provided, applicant will be used.
                if let Some(mut allowed_acc) = allowed_accounts {
                    let mut wallet = KeyManagement { admin: applicant,
                        key_pointer: applicant,
                        allowed_keys: vec![applicant],
                    };
                    wallet.allowed_keys.append(&mut allowed_acc);

                    let applicant_data = ApplicantProfile::new(name, team_size, description, applicant,time)
                        .map_err(|_| Error::UnexpectedError)?;
                    self.applicant_profile.insert(&wallet.key_pointer, &applicant_data);

                    // Register Keys
                    self.manage_keys.push(wallet);

                    Ok(())

                }else {
                    let mut wallet = KeyManagement {
                        admin: applicant,
                        key_pointer: applicant,
                        allowed_keys: vec![applicant],
                    };

                    let applicant_data = ApplicantProfile::new(name, team_size, description, applicant,time)
                        .map_err(|_| Error::UnexpectedError)?;
                    self.applicant_profile.insert(&wallet.key_pointer, &applicant_data);

                    // Register Keys
                    self.manage_keys.push(wallet);

                    Ok(())
                }
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

            // Check if the keys are less that MAX_KEYS
            if allowed_accounts.len() as u8 >= MAX_KEYS {
                return Err(Error::MaxKeysExceeded)
            }

            // Checking if the admin-account is already registered
            if let Some(_key) =  self.manage_keys.iter()
                .find(|&k| k.admin == issuer_admin){
                return Err(Error::AccountExists)?;

                // If not then create a new one
            }else {
                let time = Self::env().block_timestamp();
                let profile = IssuerProfile::new(name,chain,categories,time,description)
                    .map_err(|_|Error::UnexpectedError)?;

                let mut wallet = KeyManagement {
                    admin: issuer_admin,
                    key_pointer: issuer_admin,
                    allowed_keys: vec![issuer_admin]
                };

                wallet.allowed_keys.append(&mut allowed_accounts.clone());

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
        fn update_keys(&mut self, account: AccountId, action: KeyAction) -> CreateResult<()> {
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
                   acc_manage.update_keys_inner(account,action)?;
                    Ok(())
                },
                None => Err(Error::NotAuthorized)
            }
        }

        #[ink(message, payable, selector = 0xC0DE0004)]
        fn update_issuer_profile(

            &mut self, description: Option<String>,
            categories: Option<Vec<String>>,
            chain: Option<Option<String>>,
            status: Option<bool>

        ) -> CreateResult<()> {
            // Authorization logic
            let caller = Self::env().caller();
            // Checking if caller is in allowed_keys and then returning the key_pointer
            let key = self.manage_keys.clone().into_iter().find_map(|wallet| {
                if wallet.allowed_keys.contains(&caller) {
                    Some(wallet.key_pointer)
                } else {
                    None
                }
            });

            // Return error if the key is not available
            let key = key.ok_or(Error::NotAuthorized)?;

           // If the key is present
            for index in 1..=4 {

                if index == 1 && description.is_some(){
                    let mut profile = self.issuer_profile.get(key).ok_or(Error::ProfileDontExists)?;
                    profile.description = description.clone().ok_or(Error::UnexpectedError)?;
                    self.issuer_profile.insert(key,&profile);

                }else if index == 2 && categories.is_some() {
                    let mut profile = self.issuer_profile.get(key).ok_or(Error::ProfileDontExists)?;
                    profile.categories = categories.clone();
                    self.issuer_profile.insert(key,&profile);

                }else if index == 3 && chain.is_some(){
                    let mut profile = self.issuer_profile.get(key).ok_or(Error::ProfileDontExists)?;
                    profile.chain = chain.clone().ok_or(Error::UnexpectedError)?;
                    self.issuer_profile.insert(key,&profile);

                }else if index == 4 && status.is_some() {
                    let mut profile = self.issuer_profile.get(key).ok_or(Error::ProfileDontExists)?;
                    profile.is_active = status.ok_or(Error::UnexpectedError)?;
                    self.issuer_profile.insert(key,&profile);
                }
            };

            Ok(())
        }
    }
}
