use subxt::{OnlineClient, SubstrateConfig, PolkadotConfig};
use subxt::ext::codec;
use sp_keyring::AccountKeyring;
use subxt::ext::sp_core::sr25519::Pair;
use subxt::tx::PairSigner;
use subxt::utils::{AccountId32, H256};
use subxt::Metadata;
use frame_support::traits::{Bounded};
use frame_support::traits::schedule::DispatchTime;
use pallet_referenda::BoundedCallOf;
use hex_literal::hex;
use pallet_referenda::PalletsOriginOf;
use pallet_referenda;
// ------------------

#[subxt::subxt(runtime_metadata_path = "testmetadata.scale")]
pub mod runtime {}


#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {

    let api = OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:8000").await.unwrap();
    // Sending a txn testing
    let signer = PairSigner::<PolkadotConfig,Pair>::new(AccountKeyring::Alice.pair());
    let alice = AccountKeyring::Alice.to_account_id();

    let dest = AccountKeyring::Bob.to_account_id();

    // Preimage Lookup type
    let hash = H256::from(hex!("1d0d7261146a0a9d5b27fb110385e945fda74543b4cc4c509ca2c21e6df0eac6"));

    // Referenda submit parameter types

    let len = 44u32;
    pub type RuntimeCall = runtime::runtime_types::kusama_runtime::RuntimeCall;

    let preimage = runtime::runtime_types::frame_support::traits::preimages::Bounded::<RuntimeCall>::Lookup{hash,len};

    // Runtime types for referenda
    let ref_origin = runtime::runtime_types::
                    kusama_runtime::governance::
                    origins::pallet_custom_origins::
                    Origin::BigSpender;

    let ref_origin_caller = runtime::runtime_types::kusama_runtime::OriginCaller::Origins(ref_origin);

    // DispatchTime
    let d_time = runtime::runtime_types::frame_support::traits::schedule::DispatchTime::After(1);
    // Construct referenda submit call
    let referenda_call_type = runtime::tx().referenda().submit(ref_origin_caller,preimage,d_time);

    // Call submission
    //let tx_call = api.tx().sign_and_submit_then_watch_default(&referenda_call_type,&signer).await?;


    Ok(())
}
