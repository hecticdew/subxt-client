use subxt::{ClientBuilder, DefaultConfig, PolkadotExtrinsicParams};

#[subxt::subxt(runtime_metadata_path = "bsx_metadata.scale")]
pub mod basilisk {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let api = ClientBuilder::new()
        .set_url("wss://basilisk-rpc.hydration.cloud:443")
        .build()
        .await?
        .to_runtime_api::<basilisk::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

    // https://rustdocs.bsx.fi/pallet_asset_registry/pallet/struct.Pallet.html#method.assets (I think!)
    let asset_details = api.storage().asset_registry().assets(&0, None).await?;

    // asset_details =
    // {
    //   name: BSX
    //   assetType: Token
    //   existentialDeposit: 1,000,000,000,000
    //   locked: false
    // }

    // Unwrap the Option<AssetDetails> and assign the BoundedVec<u8> to asset_name
    let asset_name = asset_details.unwrap().name;

    // asset_name is of type subxt_client::basilisk::runtime_types::frame_support::storage::bounded_vec::BoundedVec.
    // This struct seems to not behave in the same as other BoundedVec<T>'s and its unclear which version this explictly is.
    // My current thoughts are that this is generated as part of the subxt metadata extract and that it is incorrect/incomplete.

    println!("Asset response:\t\t{:?}", asset_name); // -> BoundedVec([66, 83, 88])

    let inner_vec = asset_name.0;
    println!("Extracted slice(?):\t{:?}", inner_vec);

    let iter = inner_vec.iter();

    let mut asset_name_pretty = String::from("");

    for c in iter {
        println!("\nThis iteration char:\t{}", (*c as char).to_string());
        asset_name_pretty += &(*c as char).to_string();
        println!("String being built:\t{}", asset_name_pretty);
    }

    println!("\nFinal string (builder):\t\t{}", asset_name_pretty); // -> "BSX"

    // A much cleaner vec u8 -> str
    println!("Final string (from_utf8):\t{}", std::str::from_utf8(&inner_vec).unwrap());

    Ok(())
}
