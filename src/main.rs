use frame_support::{Blake2_256, StorageHasher};
use subxt::{OnlineClient, SubstrateConfig, PolkadotConfig};
use subxt::ext::codec;
use sp_keyring::AccountKeyring;
use subxt::ext::sp_core::sr25519::Pair;
use subxt::tx::PairSigner;
use subxt::utils::{AccountId32, H256};
use subxt::Metadata;
use frame_support::traits::{Bounded, Len};
use frame_support::traits::schedule::DispatchTime;
use pallet_referenda::BoundedCallOf;
use hex_literal::hex;
use pallet_referenda::PalletsOriginOf;
use parity_scale_codec::Encode;
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

    // Constructing a treasury spend preimage

    let treasury_spend_call = runtime::runtime_types::kusama_runtime::RuntimeCall::Treasury(
        runtime::runtime_types::pallet_treasury::pallet::Call::spend {
            amount:100000000,
            beneficiary:dest.into()
        }
    ).encode();

    let call_encoded_hex = hex::encode(treasury_spend_call.clone());
    let call_encoded_len = treasury_spend_call.clone().len();
    let call_encoded_hash = Blake2_256::hash(&treasury_spend_call);
    let pre_preimage = H256::from(call_encoded_hash);
    let hash_non_native = H256::from(hex!("0222bac530a16d345fcf8f8a0d5ca3bcfae1d4768f7920ae4fa62c3506caebb0"));
    // Issues on hashing the call preimage data

    // hex hashing encoded call
    /*let hex_hashed_call = Blake2_256::hash(hex!(call_encoded_hex));
    let hex_hashed_hex = H256::from(hex_hashed_call);*/
    println!("{:?}",call_encoded_hex);
    println!("{:?}",call_encoded_len);
    // Call type
    let preimage_call_type = runtime::tx().preimage().note_preimage(treasury_spend_call.to_vec());
    let preimage_tx = api.tx().sign_and_submit_default(&preimage_call_type,&signer).await?;
    println!("Preimage txn hash:  {:?}",preimage_tx);
    //----------------------------------***----------------------------------

    // Referenda submit parameter types

    let len:u32 = call_encoded_len as u32;
    pub type RuntimeCall = runtime::runtime_types::kusama_runtime::RuntimeCall;

    let preimage = runtime::runtime_types::frame_support::traits::preimages::Bounded::<RuntimeCall>::Lookup{hash:hash_non_native,len};

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

    // Referenda submission
    //let referenda_tx_call = api.tx().sign_and_submit_then_watch_default(&referenda_call_type,&signer).await?;



    Ok(())
}
