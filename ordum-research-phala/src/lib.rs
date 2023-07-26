#![cfg_attr(not(feature = "std"), no_std,no_main)]

//--------ORDUM FIRST ITERATION IMPLEMENTATION----------//


use ink::{self};
use ink::primitives::{AccountId};
use ink::storage::traits::{StorageLayout};
use scale::{Decode, Encode};
use ink::prelude::{vec::Vec,vec,string::String};

use core::hash::Hash;
use ink_types::Timestamp;

//------- The contracts has 5 parts --------------------------------//


//1. Team Accounts & Profile Management
//2. Key Managements for agile profiles ( Team as Daos)
//3. Project and Milestone Tracking
//4. Token and User Auth for Offchain Db
//5. Contract Upgradability

//------------------------------------------------------------------//





/// Constants
const MAX_KEYS:u8 = 3;


// Enums & Structs

#[derive(Eq,PartialEq, Encode,Decode,Clone,Debug)]
#[cfg_attr(feature = "std",derive(StorageLayout,scale_info::TypeInfo))]
#[derive(Default)]
pub enum Categories {
   Defi,
   Identity,
   Privacy,
   Infrastructure,
   NetworkChanges,
   Art,
   Media,
   Gaming,
   Events,
   Education,
   NFTs,
   Translation,
   Governance,
   #[default]
   PublicGood
}



#[derive(Eq, PartialEq,Encode,Decode,Clone, Debug)]
#[cfg_attr(feature = "std",derive(StorageLayout,scale_info::TypeInfo))]
#[derive(Default)]
pub enum Chains {
    #[default]
    Polkadot,
    Kusama,
    //Near,
    OffChain,
    //Ethereum,
    //Cardano
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

/// Error types for Milestone Tracking
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum MilestoneError {
    NotAuthorized,
    UnexpectedError,
    StorageExceeded,
    MilestoneNotFound,
    ProjectNotFound
}



/// Team Member Roles
#[derive(Clone, Encode, Decode, Debug,PartialEq)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
pub enum MemberRole {
    Admin,
    Regular
}

#[derive(Clone, Encode, Decode, Debug,PartialEq)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
#[derive(Default)]
pub enum UserRole {
    Individual,
    #[default]
    Foundation
}





///  A grant applicant profile
#[derive(Clone,Encode, Decode, Debug)]
#[cfg_attr(feature = "std",derive(StorageLayout,scale_info::TypeInfo))]
pub struct ApplicantProfile {
    name: String,
    account_id: AccountId,
    description: String,
    chain: Vec<Chains>,
    members: Option<Vec<(AccountId,MemberRole)>>,
    pub ref_team: Vec<AccountId>,// Team Id / Team Account Id
    registered_time: Timestamp,
    applications:Option<u8>,
    categories: Option<Vec<Categories>>,
    links:Option<Vec<String>>,
    role: UserRole 
}

impl ApplicantProfile {
    pub fn new(
        name: String, 
        description: String,
        account: AccountId,
        time: Timestamp,
        categories: Option<Vec<Categories>>,
        chain: Vec<Chains>,
        members: Option<Vec<(AccountId,MemberRole)>>,
        links: Option<Vec<String>>,
        role:UserRole
    ) -> CreateResult<Self> {

        Ok(Self {
            name,
            account_id: account,
            description,
            members,
            ref_team: vec![],
            registered_time: time,
            applications: None,
            categories,
            chain,
            links,
            role
        })
    }

    pub fn update_ref_team(&mut self,team:AccountId) -> CreateResult<()>{
        let prev_state = &mut self.ref_team;
        prev_state.push(team);
        self.ref_team = prev_state.to_vec();
        Ok(())
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


//----------------------Milestones Struct------------------------------------------------------
#[derive(Encode,Clone,Default, Decode, Debug)]
#[cfg_attr(feature = "std", derive(StorageLayout,scale_info::TypeInfo))]
pub struct EditedMile {
    pub edited_index:u8,
    pub main_index:u8,
    data:String,
    mem:u32 // Storing the byte memory of the stored file pointing to IPFS
}

impl EditedMile {
    pub fn new(edited_index:u8,main_index:u8,data:String,mem:u32) -> Self{
        Self{
            edited_index,
            main_index,
            data,
            mem
        }
    }
}

#[derive(Encode,Clone,Default, Decode, Debug)]
#[cfg_attr(feature = "std", derive(StorageLayout,scale_info::TypeInfo))]
pub struct AddMilestone {
    pub main_index:u8,
    pub no_edits:u8,
    data: String,
    mem: u32
}

impl AddMilestone{
    pub fn new(main_index:u8,no_edits:u8,data:String,mem:u32) -> Self{
        Self{
            main_index,
            no_edits,
            data,
            mem,
        }
    }

}



pub const MAX_MEM:u32 = 41_943_040; // 5Mbs



#[derive(Encode,Clone,Default, Decode,Debug)]
#[cfg_attr(feature = "std", derive(StorageLayout,scale_info::TypeInfo))]
pub struct FetchedMilestone{
    pub id:u8,
    pub edited_per_mile: Option<Vec<EditedMile>>,
    pub all_edits: Option<Vec<(u8,Vec<EditedMile>)>>,
    pub main: Option<Vec<AddMilestone>>,
    pub pivoted: Option<Vec<AddMilestone>>,
}



#[derive(Encode,Clone,Default, Decode,Debug)]
#[cfg_attr(feature = "std", derive(StorageLayout,scale_info::TypeInfo))]
pub struct InnerProject {
    pub chain: Chains,
    pub file: String,
    pub referenda_no: Option<u32>
}

#[derive(Encode,Clone,Default, Decode,Debug)]
#[cfg_attr(feature = "std", derive(StorageLayout,scale_info::TypeInfo))]
pub struct Project{
    id: u8,
    pub data: InnerProject,
    pub edited: Vec<(u8,Vec<EditedMile>)>, // (index == main milestone, value == Vec<EditedMilestonesPerMilestone>)
    pub main: Vec<AddMilestone>,
    pub pivoted: Vec<Vec<AddMilestone>>,
    //Utils
    pub pivot_reason: Option<Vec<String>>,
    pub pivot_index: Option<Vec<u8>>,
    pub total_mem: u32, // total memory used
}

// Implement Debug Trait manually for Project

impl Project {
    

    pub fn add_main(&mut self,mile:AddMilestone,mem:u32) -> Result<(),MilestoneError>{
        // Check if still u have the memory bandwidth
        let used_mem = self.total_mem.saturating_add(mem);
        if used_mem < MAX_MEM {
            return Err(MilestoneError::StorageExceeded)
        }
      
        self.main.push(mile);
        // Update the sorage
        self.total_mem = self.total_mem.saturating_add(mem);
        Ok(())
    }


    pub fn add_edit(&mut self, mile_no: u8, mile:EditedMile,mem:u32) -> Result<(),MilestoneError>{
        // Check if still u have the memory bandwidth
        let used_mem = self.total_mem.saturating_add(mem);
        if used_mem < MAX_MEM {
            return Err(MilestoneError::StorageExceeded)
        }

        // Check if the main milestone is there
        if let Some(_latest_main) = self.main.get_mut(mile_no as usize - 1){
            // Update the edited milestines list
            // -- check if there are edits in place associated with the milestone
            self.edited.iter_mut().for_each(|v|{
                // There exists edits for the milestone
                if v.0 == mile_no {
                    v.1.push(mile.clone());
                }else{
                    
                }
            });
            self.edited.push((mile_no,vec![mile]));

            Ok(())?

        }else{
            Err(MilestoneError::MilestoneNotFound)?
        }
       
        Ok(())
    }

}



/// Result type for Create Profile
pub type CreateResult<T> = Result<T,Error>;
/// Result type for Appliction
pub type ApplicationResult<T> = Result<T,Error>;
/// Result for milestone tracking
pub type MilestoneResult<T> = Result<T,MilestoneError>;



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
        description: String,
        categories: Option<Vec<Categories>>,
        chain:Vec<Chains>,
        members: Option<Vec<(AccountId, MemberRole)>>,
        links:Option<Vec<String>>,
        role:UserRole
    ) -> CreateResult<()>;


    // / Creates Grant Issuer Profile, a function which takes on `name: String`,
    // / `grant_levels`: This is an optional parameter whereby Issuer can choose different levels
    // / of grants to provide based on amount, `chain type if the grants is an on-chain type and
    // / None if its Off-chain`, `categories`: This specifies which categories this grant is on.
    // / `description`: extra details of the grants.
    // /
    // / Allowed Accounts act as privileged members that can control the account `Multi-Key system`
    // / In Phala context this function will be dispatched following block production
    // / as it takes in `&mut self`.
    //--------------------------------------------------------------------------------------------
    
   


    /// Adding and removing allowed accounts by the `admin`
    /// This will allow not only one person who is privileged to manage an account but also
    /// multiple allowed accounts. Max allowed accounts is a fixed constant [MAX_KEYS].
    ///
    /// Worst case scenario, time complexity will be `O(n)` with a best case of `O(1)`
    /// In Phala context this function will be dispatched following block production
    /// as it takes in `&mut self`.
    #[ink(message, selector = 0xC0DE0003)]
    fn update_keys(&mut self,account: AccountId,action: KeyAction) -> CreateResult<()>;



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

/// Trait definition for on-chain grant application process (e.g Kusama treasury )
#[ink::trait_definition]
pub trait OnchainGrant {

    /// This will be used to attest the success of the grant and the team
    /// This can only be used after community consensus and form the parameters of legitimacy
     #[ink(message,selector = 0xC0DE0008)]
    fn issue_treasury_certificate(&self) -> ApplicationResult<()>;

}

//---------------Milestone Tracking---------------------------------------------------//

//------- Structure-------------//
//
//
//                  - - - - - - - - - - - - - - - - - - - - -> M6_E1 --> M6_E2 --> M6_E3
//             -                                          -
//         -                                            -
// Edited  ------------> M2_E1 --> M2_E2              -
//                    -                            -
//                  -                           -  
// Main --> M1 --> M2 --> M3 --> M4 --> M5 --> M6 --> M7
//                                                     -
//                                                        -
// Pivoted --------------------------------------------------> P_M7 --> P_M8 --> P_M9

/// Trait fot Milestone tracking functionalities
#[ink::trait_definition]
pub trait MilestoneTracker {
    
    #[ink(message, selector = 0xC0DE0009)]
    fn add_milestone(&mut self,project:u8,file:String,mem:u32) -> MilestoneResult<()>;

    #[ink(message, selector = 0xC0DE0010)]
    fn edit_milestone(&mut self,project:u8,mile_no:u8,file:String,mem:u32) -> MilestoneResult<()>;

    #[ink(message, selector = 0xC0DE0011)]
    fn pivote_milestone(&mut self,project:u8,mile_no:u8,file:String,mem:u32) -> MilestoneResult<()>;

    /// Flexible to fetch any stage of the milestone
    /// Annotate which depth of the edits you want to receive, default set to all edits
    #[ink(message, selector = 0xC0DE0012)]
    fn fetch_milestone(&self,project_id:u8,mile_no:Option<u8>) -> MilestoneResult<FetchedMilestone>;
    
}


/// Trait fot Proposal application
#[ink::trait_definition]
pub trait Proposer {
    
    #[ink(message, selector = 0xC0DE0013)]
    fn add_proposal(&mut self,chain:Chains,ref_no:Option<u32>,file:String,mem:u32) -> MilestoneResult<()>;


    #[ink(message, selector = 0xC0DE0014)]
    fn fetch_proposal(&self,proposal_id:u8) -> MilestoneResult<Project>;
    
}


// Offchain DB Auth Trait
#[ink::trait_definition]
pub trait OffchainDbAuth {

    #[ink(message, selector = 0xC0DE0015)]
    fn get_random(&self) -> CreateResult<Vec<u8>>;

    #[ink(message, selector= 0xC0DE0016)]
    fn set_passcode(&mut self,rand:Vec<u8>) -> CreateResult<()>;

    #[ink(message, selector= 0xC0DE0017)]
    fn get_passcode(&self) -> CreateResult<String>;

}


// ----------CONTRACT IMPLEMENTATION--------------------------------------//

#[ink::contract]
mod ordum {

    use ink::storage::Mapping;
    use pink_extension as pink;
    use ink_env::hash::{CryptoHash,Blake2x128};
    use scale::{Encode,Decode};
    use hex;

    use crate::{Categories,AddMilestone,EditedMile,
        Chains, CreateResult, 
        KeyAction, KeyManagement, MemberRole,UserRole, 
        Project, FetchedMilestone,Proposer,OffchainDbAuth
    };
    use super::{Vec,vec,CreateProfile,String,
        ApplicantProfile,
        Error,MilestoneError,MilestoneResult,
        MilestoneTracker,InnerProject};


    /// Ordum Global State
    #[ink(storage)]
    pub struct OrdumState {
        applicant_profile: Mapping<AccountId,ApplicantProfile>,
        manage_keys: Vec<KeyManagement>,
        proposal: Mapping<AccountId,Vec<Project>>,
        db_auth: Mapping<AccountId,Vec<u8>>
        // Mapping issuer_id to a mapping of  application number to application profile
        // As this will enable specifi grant issuer to have dedicated list of queue application
        // and also teams to have numerous application per one issuer
        //queue_applications:Mapping<u16,Mapping<u32,u32>>,
        

    }


    /// Event emitted when new Applicant is registered
        #[ink(event)]
        pub struct ApplicantAccountCreated {
            #[ink(topic)]
            name: String,
            time:  Timestamp
        }
    /// Event emitted when Applicant updates the profile
        #[ink(event)]
        pub struct ApplicantUpdated {
            #[ink(topic)]
            name: String,
            time: Timestamp
        }
    
    /// Event for setting passcode
        #[ink(event)]
        pub struct PasscodeSet {
            #[ink(topic)]
            account: AccountId
        }

    impl OrdumState {

        #[ink(constructor)]
        pub fn new() -> Self{
                              
                Self {
                    applicant_profile: Mapping::default(),
                    manage_keys: vec![],
                    proposal: Mapping::default(),
                    db_auth: Mapping::default()
                }
        }

        // Remember Upgradability
        /// Modifies the code which is used to execute calls to this contract address (`AccountId`).
        ///
        /// We use this to upgrade the contract logic. We don't do any authorization here, any caller
        /// can execute this method. In a production contract you would do some authorization here.
        #[ink(message)]
        pub fn set_code(&mut self, code_hash: [u8; 32]) {
            ink::env::set_code_hash(&code_hash).unwrap_or_else(|err| {
                panic!(
                    "Failed to `set_code_hash` to {:?} due to {:?}",
                    code_hash, err
                )
            });
            ink::env::debug_println!("Switched code hash to {:?}.", code_hash);
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
                Err(Error::NotAuthorized)
            }

        }

      
    }

    impl CreateProfile for OrdumState {
        #[ink(message,selector =0xC0DE0001)]
        fn create_applicant_profile(
            
            &mut self, name: String,
            account: Option<AccountId>,
            description: String,
            categories: Option<Vec<Categories>>,
            chain:Vec<Chains>,
            members: Option<Vec<(AccountId, MemberRole)>>,
            links:Option<Vec<String>>,
            role:UserRole

        ) -> CreateResult<()> {

            let applicant = Self::env().caller();
            let time = Self::env().block_timestamp();

            // Check if account is provided or else use applicant account
            if let Some(account_inner) = account {
                // Check using wallet key pointer ?????????
                // Check if account exists
                if self.applicant_profile.contains(account_inner){
                    return Err(Error::AccountExists);
                }

                let applicant_data = ApplicantProfile::new(name,description,account_inner,time,categories,chain,members.clone(),links,role)
                    .map_err(|_|Error::UnexpectedError)?;

                // Check for member addition reference
                if let Some(member) = members {
                        // Update the Key Mangement
                        let mut keys = vec![applicant];

                        member.iter().for_each(|mem|{
                            if mem.1 == MemberRole::Admin{
                                keys.push(mem.0);
                            }
                        });

                        let wallet_data = KeyManagement {
                            admin: applicant,
                            key_pointer: account_inner,
                            allowed_keys: keys,
                        };


                        // Check if the AccountId does have a profile
                        member.iter().for_each(|mem|{
                            if self.applicant_profile.contains(mem.0) {
                                let mut acc_data = self.applicant_profile.get(mem.0).unwrap();
                                acc_data.update_ref_team(account_inner).unwrap();


                                let _applicant_val_bytes = self.applicant_profile.insert(wallet_data.key_pointer,&applicant_data);
                                

                                // Register Keys
                                self.manage_keys.push(wallet_data.clone());
                                // Emits an event
                                Self::env().emit_event(ApplicantAccountCreated{
                                    name:applicant_data.clone().name,
                                    time,
                                });

                            }else{
                                let _applicant_val_bytes = self.applicant_profile.insert(wallet_data.key_pointer,&applicant_data);
                               

                                // Register Keys
                                self.manage_keys.push(wallet_data.clone());
                                // Emits an event
                                Self::env().emit_event(ApplicantAccountCreated{
                                    name:applicant_data.clone().name,
                                    time,
                                });
                                
                            }
                        });

                        Ok(())
                        
                    }else{

                        let wallet_data = KeyManagement {
                            admin: applicant,
                            key_pointer: account_inner,
                            allowed_keys: vec![applicant],
                        };

                        let _applicant_val_bytes = self.applicant_profile.insert(wallet_data.key_pointer,&applicant_data);
                        

                        // Register Keys
                        self.manage_keys.push(wallet_data.clone());
                        // Emits an event
                        Self::env().emit_event(ApplicantAccountCreated{
                            name:applicant_data.clone().name,
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
                let applicant_data = ApplicantProfile::new(name,description,applicant,time,categories,chain,members.clone(),links,role)
                    .map_err(|_|Error::UnexpectedError)?;

                // Check for member addition reference
                if let Some(member) = members {
                        // Update the Key Mangement
                        let mut keys = vec![applicant];

                        member.iter().for_each(|mem|{
                            if mem.1 == MemberRole::Admin{
                                keys.push(mem.0);
                            }
                        });

                        let wallet_data = KeyManagement {
                            admin: applicant,
                            key_pointer: applicant,
                            allowed_keys: keys,
                        };


                        // Check if the AccountId does have a profile
                        member.iter().for_each(|mem|{
                            if self.applicant_profile.contains(mem.0) {
                                let mut acc_data = self.applicant_profile.get(mem.0).unwrap();
                                acc_data.update_ref_team(applicant).unwrap();


                                let _applicant_val_bytes = self.applicant_profile.insert(wallet_data.key_pointer,&applicant_data);
                                

                                // Register Keys
                                self.manage_keys.push(wallet_data.clone());
                                // Emits an event
                                Self::env().emit_event(ApplicantAccountCreated{
                                    name:applicant_data.clone().name,
                                    time,
                                });

                            }else{
                                let _applicant_val_bytes = self.applicant_profile.insert(wallet_data.key_pointer,&applicant_data);
                                

                                // Register Keys
                                self.manage_keys.push(wallet_data.clone());
                                // Emits an event
                                Self::env().emit_event(ApplicantAccountCreated{
                                    name:applicant_data.clone().name,
                                    time,
                                });
                                
                            }
                        });

                        Ok(())
                    }else{

                        let wallet_data = KeyManagement {
                            admin: applicant,
                            key_pointer: applicant,
                            allowed_keys: vec![applicant],
                        };

                        let _applicant_val_bytes = self.applicant_profile.insert(wallet_data.key_pointer,&applicant_data);
                       

                        // Register Keys
                        self.manage_keys.push(wallet_data.clone());
                        // Emits an event
                        Self::env().emit_event(ApplicantAccountCreated{
                            name:applicant_data.clone().name,
                            time,
                        });
                        Ok(())

                    }
                
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
       
    }



    impl Proposer for OrdumState {

        #[ink(message, selector = 0xC0DE0013)]
        fn add_proposal(&mut self,chain:Chains,ref_no:Option<u32>,file:String,mem:u32) -> MilestoneResult<()>{

            let caller = Self::env().caller();
            // Check if the caller has a profile account
            if let Some(wallet) = self.manage_keys.iter().find(|&key|{
                key.allowed_keys.contains(&caller)
            }){

                // Check if there is existing Projects
                if let Some(mut projects) = self.proposal.get(wallet.key_pointer){

                    let no_projects = projects.len() as u8;
                    // Build the Project Object and InnerProject
                    let inner_project = InnerProject {
                        chain,
                        file,
                        referenda_no:ref_no
                    };

                    let project = Project {
                        id: no_projects.saturating_add(1),
                        data: inner_project,
                        edited: vec![],
                        main: vec![],
                        pivoted: vec![],
                        pivot_reason: None,
                        pivot_index: None,
                        total_mem: mem
                    };

                    projects.push(project);
                    
                    
                }else{
                    
                    let inner_project = InnerProject {
                        chain,
                        file,
                        referenda_no:ref_no
                    };

                    let project = Project {
                        id: 1,
                        data: inner_project,
                        edited: vec![],
                        main: vec![],
                        pivoted: vec![],
                        pivot_reason: None,
                        pivot_index: None,
                        total_mem: mem
                    };

                    self.proposal.insert(wallet.key_pointer,&vec![project]);
                }

            }else{
                Err(MilestoneError::NotAuthorized)?
            }
            Ok(())
        }


        #[ink(message, selector = 0xC0DE0014)]
        fn fetch_proposal(&self,proposal_id:u8) -> MilestoneResult<Project>{

            let caller = Self::env().caller();
            // Check if the caller has a profile account
            if let Some(wallet) = self.manage_keys.iter().find(|&key|{
                key.allowed_keys.contains(&caller)
            }){

                let projects = self.proposal.get(wallet.key_pointer).ok_or(MilestoneError::ProjectNotFound)?;
                if !projects.is_empty(){

                    if let Some(project) = projects.get(proposal_id as usize - 1){
                        Ok(project.clone())
                    }else{
                        Err(MilestoneError::NotAuthorized)
                    }

                }else{
                    Err(MilestoneError::ProjectNotFound)
                }

            }else{
                Err(MilestoneError::NotAuthorized)
            }
        }
    }



    impl MilestoneTracker for OrdumState {
       
        #[ink(message, selector = 0xC0DE0009)]
        fn add_milestone(&mut self,project_id:u8,file:String,mem:u32) -> MilestoneResult<()>{

            let caller = Self::env().caller();
            // Check if the caller has a profile account
            if let Some(wallet) = self.manage_keys.iter().find(|&key|{
                key.allowed_keys.contains(&caller)
            }){
                let _applicant = self.applicant_profile.get(wallet.key_pointer).unwrap();
                
                // Check if there id a registered project
                if let Some(projects) = self.proposal.get(wallet.key_pointer){
                    let mut current_project = projects[project_id as usize -1].clone();
                    
                    // Check if there are milestones
                    if let Some(mile) = current_project.main.last(){
                        // Build a Milestone
                        let current_main_index = current_project.main.len() as u8;
                        
                        let milestone = AddMilestone::new(current_main_index +1, mile.no_edits, file, mem);
                        current_project.add_main(milestone, mem)?

                    }else{

                        let milestone = AddMilestone::new(1, 0, file, mem);
                        current_project.add_main(milestone, mem)?
                    }
                   Ok(())? 
                }else{
                    Err(MilestoneError::ProjectNotFound)?
                }
            }
            Ok(())
        }
    
    

        #[ink(message, selector = 0xC0DE0010)]
        fn edit_milestone(&mut self,project_id:u8,mile_no:u8,file:String,mem:u32) -> MilestoneResult<()>{

            let caller = Self::env().caller();

             // Check if the caller has a profile account
             if let Some(wallet) = self.manage_keys.iter().find(|&key|{
                key.allowed_keys.contains(&caller)
            }){
                let _applicant = self.applicant_profile.get(wallet.key_pointer).unwrap();

                 // Check if there id a registered project
                 if let Some(projects) = self.proposal.get(wallet.key_pointer){
                    let mut current_project = projects[project_id as usize -1].clone();
                    
                    // Check if there are milestones
                   if current_project.main.is_empty(){
                        Err(MilestoneError::MilestoneNotFound)?
                   }
                   // get the latest no of edits in the specified milestone
                   let specified_mile = current_project.main.get(mile_no as usize -1).ok_or(MilestoneError::MilestoneNotFound)?;
        
                   // Build the edit milestone object
                   let edited_milestone = EditedMile::new(specified_mile.no_edits + 1,mile_no,file,mem);
                   // Store it
                   current_project.add_edit(mile_no, edited_milestone, mem)?;
                   Ok(())? 
                }else{
                    Err(MilestoneError::ProjectNotFound)?
                }
            }            

            Ok(())
        }
    

        #[ink(message, selector = 0xC0DE0011)]
        fn pivote_milestone(&mut self,_project:u8,_mile_no:u8,_file:String,_mem:u32) -> MilestoneResult<()>{

            let _caller = Self::env().caller();
            Ok(())
        }
    

        #[ink(message, selector = 0xC0DE0012)]
        fn fetch_milestone(&self,project_id:u8,mile_no:Option<u8>) -> MilestoneResult<FetchedMilestone>{

            let caller = Self::env().caller();

            // FetchMilestone Object
            let mut result_milestone = FetchedMilestone::default();

             // Check if the caller has a profile account
             if let Some(wallet) = self.manage_keys.iter().find(|&key|{
                key.allowed_keys.contains(&caller)
            }){

                
                // Check if the projects are there
                if let Some(projects) = self.proposal.get(wallet.key_pointer){
                    // Check if the specific project is there
                    if let Some(project) = projects.get(project_id as usize - 1){

                        // Check if the milestone is there
                        if project.main.is_empty() {
                            Err(MilestoneError::MilestoneNotFound)?
                        }

                        // Check if specific milestone is given
                        if let Some(m_no) = mile_no{
                            // Fetch the milestone in the main and the edits
                            if let Some(mile) = project.main.get(m_no as usize -1){

                                // Fetch the edits associated with the milestone
                                let edit = project.edited.iter().find(|&v| v.0 == m_no);
                                if edit.is_some(){
                                    result_milestone.main = Some(vec![mile.clone()]); 
                                    result_milestone.edited_per_mile = Some(edit.ok_or(MilestoneError::UnexpectedError)?.1.clone());
                                }
                                // If there are no edits
                                result_milestone.main = Some(vec![mile.clone()]); 

                            }else{
                                Err(MilestoneError::MilestoneNotFound)?
                            }  

                        }else{
                            // Construct a fetchedMilestone object to fetch whole tree of milestone nodes
                            result_milestone.main = Some(project.main.clone());
                            // Fetch all the edits per milestons
                            // -- check in the edited section if there is a main_index value and push it

                            //NOTE: We can optimize here as for now the algorithm is searching the whole edited vector and it doest need to; 0(N^N)
                            let mut edits_value:Vec<(u8,Vec<EditedMile>)> = Vec::new();

                            project.main.iter().for_each(|m|{
                                project.edited.iter().for_each(|edit|{
                                   if edit.0 == m.main_index{
                                        edits_value.push(edit.clone())
                                   }
                                })
                            });
                            // Check if the edits_value contain any value if not the leave the ResultMilestone as it is;
                            if !edits_value.is_empty(){
                                result_milestone.all_edits = Some(edits_value);
                            }
                        }

                    }else{
                        Err(MilestoneError::ProjectNotFound)?
                    }     
                    
                }else{
                    Err(MilestoneError::ProjectNotFound)?
                }
            }else{
                Err(MilestoneError::NotAuthorized)?
            }
            Ok(result_milestone)
                  
        }
    }

    

        // Account abstraction Research
        // -- Lite implementation of token abstraction , this will be part of ANTA
        
        // 2. Offchain DB auth
        impl OffchainDbAuth for OrdumState {
            
            #[ink(message, selector = 0xC0DE0015)]
            fn get_random(&self) -> CreateResult<Vec<u8>>{
                // Get Random 20 bits number
                let rand = pink::ext().getrandom(20);
                Ok(rand)
            }
        
            #[ink(message, selector= 0xC0DE0016)]
            fn set_passcode(&mut self,rand:Vec<u8>) -> CreateResult<()>{
                let caller = Self::env().caller();
                // Hash caller + rand
                let mut preimage = caller.encode().to_vec();
                preimage.append(&mut rand.encode().to_vec());

                let mut hash:[u8;16] = Default::default();
                <Blake2x128 as CryptoHash>::hash(&preimage[..], &mut hash);

                // Store in the storage
                let passcode = hash.to_vec();
                self.db_auth.insert(caller.clone(),&passcode);
                
                // Emit Event
                Self::env().emit_event(
                    PasscodeSet {
                        account: caller
                    }
                );

                Ok(())
            }

        
            #[ink(message, selector= 0xC0DE0017)]
            fn get_passcode(&self) -> CreateResult<String>{
                // Get the Vector bit 
                // Hex encode
                let caller = Self::env().caller();
                let passcode_value = self.db_auth.get(&caller).ok_or(Error::NotAuthorized)?;

                let passcode = hex::encode(passcode_value);

                Ok(passcode)
            }
    }

}
