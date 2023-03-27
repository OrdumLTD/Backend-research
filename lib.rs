#![cfg_attr(not(feature = "std"), no_std)]



use ink;
use ink::primitives::{AccountId, Key};
use ink::storage::traits::StorageLayout;
use scale::{Decode, Encode};
use ink::prelude::{vec::Vec,vec,string::String};
use ink::storage::Mapping;
use core::hash::Hash;
use ink_types::Timestamp;



/// Constants
const MAX_KEYS:u8 = 3;


// Enums & Structs

#[derive(Eq,PartialEq, Encode,Decode,Clone,Debug)]
#[cfg_attr(feature = "std",derive(StorageLayout,scale_info::TypeInfo))]
pub enum Categories {
    //DeSci,
    //DeFi,
    PublicGood,
    //NFT,
    //ProtocolResearch,
    Infrastructure,
    //DeCommerce,
    //Governance,
    //Miscellaneous,
    MediaArt,
}

impl Default for Categories {
    fn default() -> Self {
        Categories::PublicGood
    }
}

#[derive(Eq, PartialEq,Encode,Decode,Clone, Debug)]
#[cfg_attr(feature = "std",derive(StorageLayout,scale_info::TypeInfo))]
pub enum Chains {
    Polkadot,
    //Near,
    OffChain,
    //Ethereum,
    //Cardano
}

impl Default for Chains {
    fn default() -> Self {
        Chains::Polkadot
    }
}
// --------------------------------------------------------------------//


/// Application profile , this consist of `application_id`, `applicant_name`, `issuer_id / name`
/// and `reference of the application profile file`
#[derive(Clone,Encode, Decode, Debug)]
#[cfg_attr(feature = "std",derive(StorageLayout,scale_info::TypeInfo))]
pub struct Application{
    pub id: u32,
    pub team_name: String,
    pub issuer_id: u16,
    pub issuer_name: String,
}


/// Error type for Create Profile
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    // Profile creation errors
    AccountExists,
    NotAuthorized,
    AccountDontExists,
    ProfileDontExists,
    MaxKeysExceeded,
    AccountExistsOrMaxExceeded,
    // Grant Application errors
    /// Any system related error
    UnexpectedError,
}


///  A grant applicant profile
#[derive(Clone,Encode, Decode, Debug)]
#[cfg_attr(feature = "std",derive(StorageLayout,scale_info::TypeInfo))]
pub struct ApplicantProfile {
    name: String,
    team_size: u8,
    account_id: AccountId,
    description: String,
    registered_time: Timestamp,
    applications:Option<u8>,
    categories: Vec<Categories>
}

impl ApplicantProfile {
    pub fn new(
        name: String, team_size:u8,
        description: String,
        account: AccountId,
        time: Timestamp,
        categories: Vec<Categories>,
    ) -> CreateResult<Self> {

        Ok(Self {
            name,
            team_size,
            account_id: account,
            description,
            registered_time: time,
            applications: None,
            categories,
        })
    }
}


/// A grant issuer profile
/// The order is important in Contract Upgrades
#[derive(Encode,Clone,Default, Decode, Debug)]
#[cfg_attr(feature = "std", derive(StorageLayout,scale_info::TypeInfo))]
pub struct IssuerProfile {
    name: String,
    id: u16,
    chain: Chains,
    is_active: bool,
    registration_date: Timestamp,
    categories: Vec<Categories>,
    description: String,
    applications: Option<Vec<u16>>,
}


impl IssuerProfile {
    pub fn new(
        name: String,
        chain: Chains,
        categories: Vec<Categories>,
        time: Timestamp,
        description: String

    ) -> CreateResult<Self> {
        let id = (time % 999).try_into().map_err(|_|Error::UnexpectedError)?;
        Ok(Self{
            name,
            id,
            chain,
            is_active: true,
            registration_date: time,
            categories,
            description,
            applications: None,
        })
    }

}

/// Key management struct
/// This will allow multiple members in certain organization to manage the account
/// The allowed members will be granted by `admin` key
/// The `key_pointer` is the key used in the key to `IssuerProfile` mapping
#[derive(Clone,Encode,Hash, Decode, Debug)]
#[cfg_attr(feature = "std", derive(StorageLayout,scale_info::TypeInfo))]
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


//-------- Extra traits for custom data structure ------------

/*impl PackedAllocate for IssuerProfile{
    fn allocate_packed(&mut self, at: &Key) {
        PackedAllocate::allocate_packed(&mut self.categories, at);
        PackedAllocate::allocate_packed(&mut self.id, at);
        PackedAllocate::allocate_packed(&mut self.description, at);
        PackedAllocate::allocate_packed(&mut self.chain, at);
        PackedAllocate::allocate_packed(&mut self.applications, at);
        PackedAllocate::allocate_packed(&mut self.is_active, at);
        PackedAllocate::allocate_packed(&mut self.name, at);
        PackedAllocate::allocate_packed(&mut self.registration_date, at);
    }
}

impl SpreadAllocate for Categories {
    fn allocate_spread(ptr: &mut KeyPtr) -> Self {
        ptr.advance_by(<Self as SpreadLayout>::FOOTPRINT);
        Categories::PublicGood
    }
}

impl PackedAllocate for Categories {
    fn allocate_packed(&mut self, at: &Key) {
        if self == &Categories::PublicGood{
            PackedAllocate::allocate_packed(&mut Categories::PublicGood,at);
        } else if self == &Categories::MediaArt {
            PackedAllocate::allocate_packed(&mut Categories::MediaArt,at);
        } else if self == &Categories::Infrastructure {
            PackedAllocate::allocate_packed(&mut Categories::Infrastructure,at);
        }
    }
}

impl PackedAllocate for Application {
    fn allocate_packed(&mut self, at: &Key) {
        PackedAllocate::allocate_packed(&mut self.id,at);
        PackedAllocate::allocate_packed(&mut self.issuer_id, at);
        PackedAllocate::allocate_packed(&mut self.team_name,at);
        PackedAllocate::allocate_packed(&mut self.issuer_name,at);
    }
}

impl SpreadAllocate for Chains {
    fn allocate_spread(ptr: &mut KeyPtr) -> Self {
        ptr.advance_by(<Self as SpreadLayout>::FOOTPRINT);
        Chains::Polkadot
    }
}

impl PackedAllocate for Chains {
    fn allocate_packed(&mut self, at: &Key) {
        if self == &Chains::Polkadot {
            PackedAllocate::allocate_packed(&mut Chains::Polkadot, at);
        }else if self == &Chains::OffChain {
            PackedAllocate::allocate_packed(&mut Chains::OffChain,at);
        };

    }
}

impl PackedAllocate for ApplicantProfile {
    fn allocate_packed(&mut self, at: &Key) {
        PackedAllocate::allocate_packed(&mut self.categories, at);
        PackedAllocate::allocate_packed(&mut self.name, at);
        PackedAllocate::allocate_packed(&mut self.applications, at);
        PackedAllocate::allocate_packed(&mut self.description, at);
        PackedAllocate::allocate_packed(&mut self.team_size, at);
        PackedAllocate::allocate_packed(&mut self.registered_time, at);
        PackedAllocate::allocate_packed(&mut self.account_id, at);
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
*/

/// Result type for Create Profile
pub type CreateResult<T> = Result<T,Error>;
pub type ApplicationResult<T> = Result<T,Error>;

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
        allowed_accounts: Option<Vec<AccountId>>,
        categories: Vec<Categories>

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
        chain: Chains,
        categories: Vec<Categories>,
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
        categories: Option<Vec<Categories>>, // This replaces the existing categories
        chain: Option<Chains>,
        status: Option<bool>

    ) -> CreateResult<()>;


}


/// Trait defination for grant application process (Offchain e.g web3 foundation grant type)
/// This can be used by both Applicants and Issuers
#[ink::trait_definition]
pub trait OffchainApply {

    /// Naming convention for Offchain grant type
    /// This function creates new application, stores is externally and commits the reference to the chain
    #[ink(message,selector = 0xC0DE0005)]
    fn apply_grant(&mut self) -> ApplicationResult<()>;

    #[ink(message,selector = 0xC0DE0006)]
    fn update_application(&mut self) -> ApplicationResult<()>;

    #[ink(message,selector = 0xC0DE0007)]
    fn review_application(&mut self) -> ApplicationResult<()>;

}



#[ink::contract]
mod OrdumAstar {

    use ink::trait_definition;
    use ink::storage::Mapping;
    use crate::{Application, Categories, Chains, CreateResult, KeyAction, KeyManagement, MAX_KEYS};
    use super::{Vec,vec,CreateProfile,String, IssuerProfile,ApplicantProfile, Error};


    /// Ordum Global State
    #[ink(storage)]
    pub struct OrdumAstar {
        issuer_profile: Mapping<AccountId,IssuerProfile>,
        list_issuer_profile: Vec<IssuerProfile>,
        applicant_profile: Mapping<AccountId,ApplicantProfile>,
        list_applicant_profile: Vec<ApplicantProfile>,
        // Mapping issuer_id to all applications on specific to it.
        // The UX will be structured in the front end
        queue_applications:Mapping<u16,Vec<u32>>,
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

    impl OrdumAstar {

        #[ink(constructor)]
        pub fn initialize() -> Self{
            let contract_id = Self::env().account_id();
            let initializer_id = Self::env().caller();
            let initial_keys = KeyManagement {
                admin: contract_id,
                key_pointer: contract_id,
                allowed_keys: vec![contract_id,initializer_id],
            };
            let applicant_profile = ApplicantProfile{
                name: String::from("Ordum"),
                team_size: 0,
                account_id: contract_id,
                description: String::from("Pirates"),
                registered_time: 0,
                applications: None,
                categories: vec![],
            };

            let mut issuer = Mapping::default();
            let mut applicant = Mapping::default();
            let mut issuer_list = Vec::<IssuerProfile>::default();
            let mut applicant_list = Vec::<ApplicantProfile>::default();


            let _issuer_val_bytes = issuer.insert(contract_id,&IssuerProfile::default());
            issuer_list.push(IssuerProfile::default());
            let _applicant_val_bytes = applicant.insert(initializer_id,&applicant_profile);
            applicant_list.push(applicant_profile);

            Self {
                issuer_profile: issuer,
                list_issuer_profile: issuer_list,
                applicant_profile: applicant,
                list_applicant_profile: applicant_list,
                queue_applications: Mapping::default(),
                manage_keys: vec![initial_keys],
            }
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

    impl CreateProfile for OrdumAstar {
        #[ink(message,selector =0xC0DE0001)]
        fn create_applicant_profile(
            &mut self, name: String,
            account: Option<AccountId>, team_size: u8,
            description: String,
            allowed_accounts: Option<Vec<AccountId>>,
            categories: Vec<Categories>

        ) -> CreateResult<()> {

            let applicant = Self::env().caller();
            let time = Self::env().block_timestamp();

            // Check if account is provided or else use applicant account
            if let Some(account_inner) = account {
                // Check if account exists
                if self.applicant_profile.contains(account_inner){
                    return Err(Error::AccountExists);
                }

                if let Some(mut allowed_acc) = allowed_accounts {
                    // Create KeyManagement
                    let mut wallet = KeyManagement {
                        admin: applicant,
                        key_pointer: account_inner,
                        allowed_keys: vec![applicant],
                    };
                    wallet.allowed_keys.append(&mut allowed_acc);

                    let applicant_data = ApplicantProfile::new(name,team_size,description,account_inner,time,categories)
                        .map_err(|_|Error::UnexpectedError)?;
                    let _applicant_val_bytes = self.applicant_profile.insert(&wallet.key_pointer,&applicant_data);
                    self.list_applicant_profile.push(applicant_data.clone());

                    // Register Keys
                    self.manage_keys.push(wallet);
                    // Emits an event
                    Self::env().emit_event(ApplicantAccountCreated{
                        name:applicant_data.name,
                        time,
                    });
                    Ok(())

                } else {
                    // If no allowed-accounts provided
                    // Create KeyManagement
                    let wallet = KeyManagement {
                        admin: applicant,
                        key_pointer: account_inner,
                        allowed_keys: vec![applicant],
                    };

                    let applicant_data = ApplicantProfile::new(name,team_size,description,account_inner,time,categories)
                        .map_err(|_|Error::UnexpectedError)?;
                    let _applicant_val_bytes = self.applicant_profile.insert(&wallet.key_pointer,&applicant_data);
                    self.list_applicant_profile.push(applicant_data.clone());
                    // Register Keys
                    self.manage_keys.push(wallet);
                    // Emits an event
                    Self::env().emit_event(ApplicantAccountCreated{
                        name:applicant_data.name,
                        time,
                    });

                    Ok(())
                }
            } else {
                // Check if account Exists
                if self.applicant_profile.contains(applicant){
                    return  Err(Error::AccountExists)
                }
                // If no account provided, applicant will be used.
                if let Some(mut allowed_acc) = allowed_accounts {
                    let mut wallet = KeyManagement { admin: applicant,
                        key_pointer: applicant,
                        allowed_keys: vec![applicant],
                    };
                    wallet.allowed_keys.append(&mut allowed_acc);

                    let applicant_data = ApplicantProfile::new(name, team_size, description, applicant,time,categories)
                        .map_err(|_| Error::UnexpectedError)?;
                    let _applicant_val_byte = self.applicant_profile.insert(&wallet.key_pointer, &applicant_data);
                    self.list_applicant_profile.push(applicant_data.clone());
                    // Register Keys
                    self.manage_keys.push(wallet);
                    // Emits an event
                    Self::env().emit_event(ApplicantAccountCreated{
                        name:applicant_data.name,
                        time,
                    });

                    Ok(())

                }else {
                    let wallet = KeyManagement {
                        admin: applicant,
                        key_pointer: applicant,
                        allowed_keys: vec![applicant],
                    };

                    let applicant_data = ApplicantProfile::new(name, team_size, description, applicant,time,categories)
                        .map_err(|_| Error::UnexpectedError)?;
                    let _applicant_val_byte = self.applicant_profile.insert(&wallet.key_pointer, &applicant_data);
                    self.list_applicant_profile.push(applicant_data.clone());
                    // Register Keys
                    self.manage_keys.push(wallet);
                    // Emits an event
                    Self::env().emit_event(ApplicantAccountCreated{
                        name:applicant_data.name,
                        time,
                    });
                    Ok(())
                }
            }


        }
        #[ink(message, selector =0xC0DE0002)]
        fn create_issuer_profile(
            &mut self, name: String,
            chain: Chains,
            categories: Vec<Categories>,
            description: String,
            allowed_accounts: Vec<AccountId>
        ) -> CreateResult<()> {

            let issuer_admin = Self::env().caller();
            // Check if account is registered
            if self.issuer_profile.contains(issuer_admin){
                return Err(Error::AccountExists)
            }
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
                let _issuer_val_bytes = self.issuer_profile.insert(wallet.key_pointer,&profile);
                self.manage_keys.push(wallet);
                self.list_issuer_profile.push(profile.clone());

                // Emit an event
                Self::env().emit_event(IssuerAccountCreated {
                    name: profile.clone().name,
                    time,
                });

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
            categories: Option<Vec<Categories>>,
            chain: Option<Chains>,
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
                    let _issuer_val_bytes  = self.issuer_profile.insert(key,&profile);

                }else if index == 2 && categories.is_some() {
                    let mut profile = self.issuer_profile.get(key).ok_or(Error::ProfileDontExists)?;
                    profile.categories = categories.clone().ok_or(Error::UnexpectedError)?;
                    let _issuer_val_bytes = self.issuer_profile.insert(key,&profile);

                }else if index == 3 && chain.is_some(){
                    let mut profile = self.issuer_profile.get(key).ok_or(Error::ProfileDontExists)?;
                    profile.chain = chain.clone().ok_or(Error::UnexpectedError)?;
                    let _issuer_val_byes = self.issuer_profile.insert(key,&profile);

                }else if index == 4 && status.is_some() {
                    let mut profile = self.issuer_profile.get(key).ok_or(Error::ProfileDontExists)?;
                    profile.is_active = status.ok_or(Error::UnexpectedError)?;
                    let _issuer_val_bytes = self.issuer_profile.insert(key,&profile);
                }
            };

            Ok(())
        }


    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_test_ordum_astar{
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn test_grant_issuer(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // given
            let constructor = OrdumAstarRef::initialize();
            let contract_id = client
                .instantiate("Ordum_Astar", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let create_issuer_profile = build_message::<OrdumAstarRef>(contract_id.clone())
                .call(|ordum|
                     ordum.create_issuer_profile(
                        String::from("ORDUM"),
                        Chains::Polkadot,
                        vec![Categories::PublicGood,Categories::Infrastructure],
                        String::from("Grant Management for degens"),
                        vec![]
            
                    ));
            
            
            let create_issuer_ok = client.call(&ink_e2e::alice(), create_issuer_profile, 0, None).await;
            assert!(matches!(create_issuer_ok.unwrap().return_value(), Ok(())));

            // Test getter
            let get_grant_issuer = build_message::<OrdumAstarRef>(contract_id.clone())
            .call(|ordum|ordum.get_issuer_profile());
                    
            let grant_issuer = client.call(&ink_e2e::alice(),get_grant_issuer,0,None).await;
            
            Ok(())
        }

    }

}
