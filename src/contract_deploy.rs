use ethers::contract::Contract;
use ethers::prelude::{
    BlockNumber, ContractFactory, LocalWallet, Middleware, Provider, Signer, SignerMiddleware, U256,
};
use ethers::utils::Ganache;
// use ethers_solc::info::ContractInfo;

use std::path::PathBuf;
use std::time::Duration;

use ethers_solc::{
    Artifact, ConfigurableArtifacts, Project, ProjectCompileOutput, ProjectPathsConfig,
};
use eyre::{eyre, ContextCompat, Result};
use hex::ToHex;

pub type SignerDeployedContract<T> = Contract<SignerMiddleware<Provider<T>, LocalWallet>>;

#[tokio::main]
async fn main() -> Result<()> {
    let mnemonic =
        "crater diet priority pact thrive duck setup junior update fashion because pelican";
    let ganache = Ganache::new().mnemonic(mnemonic).spawn();
    println!("HTTP Ganache Endpoint: {}", ganache.endpoint());

    let wallet: LocalWallet = ganache.keys()[0].clone().into();
    let first_address = wallet.address();
    println!(
        "wallet first address: {}",
        first_address.encode_hex::<String>()
    );

    let provider = Provider::try_from(ganache.endpoint())?.interval(Duration::from_millis(10));
    let chain_id = provider.get_chainid().await?.as_u64();
    println!("Ganache started with chain_id: {}", chain_id);

    let project = compile("examples/").await?;

    print_project(project.clone()).await?;

    let balance = provider.get_balance(wallet.address(), None).await?;
    println!(
        "Wallet first address {} balance: {}",
        wallet.address().encode_hex::<String>(),
        balance
    );

    let contract_name = "BUSDImplementation";
    // let info = ContractInfo::new("examples/BUSDImplementation.sol:BUSDImplementation");
    let contract = project
        // .find("examples/BUSDImplementation.sol", contract_name)
        // .find_contract(&info)
        .find_first(contract_name)
        .context("Contract not found")?
        .clone();
    // get abi and bytecode, which are only available in a compiled contract
    let (abi, bytecode, _) = contract.into_parts();
    let abi = abi.context("Missing abi from contract")?;
    let bytecode = bytecode.context("Missing bytecode from contract")?;
    // create signer client
    let wallet = wallet.with_chain_id(chain_id);
    let client = SignerMiddleware::new(provider.clone(), wallet).into();
    // deploy contract
    let factory = ContractFactory::new(abi.clone(), bytecode, client);

    let mut deployer = factory.deploy(())?;
    let block = provider
        .clone()
        .get_block(BlockNumber::Latest)
        .await?
        .context("Failed to get block")?;

    let gas_price = block
        .next_block_base_fee()
        .context("Failed to get the base fee for the next block")?;
    println!("Block gas price: {:?}", gas_price);
    deployer.tx.set_gas_price::<U256>(gas_price);

    let contract = deployer.clone().legacy().send().await?;
    println!(
        "BUSDImpl contract address: {}",
        contract.address().encode_hex::<String>()
    );

    Ok(())
}

pub async fn compile(root: &str) -> Result<ProjectCompileOutput<ConfigurableArtifacts>> {
    let root = PathBuf::from(root);
    if !root.exists() {
        return Err(eyre!("Project root {root:?} does not exist!"));
    }

    let paths = ProjectPathsConfig::builder()
        .root(&root)
        .sources(&root)
        .build()?;

    let project = Project::builder()
        .paths(paths)
        .set_auto_detect(true)
        .no_artifacts()
        .build()?;

    println!("Compiling solidity project: {:?}", root);

    let output = project.compile()?;

    if output.has_compiler_errors() {
        Err(eyre!(
            "Compiling solidity project failed: {:?}",
            output.output().errors
        ))
    } else {
        Ok(output.clone())
    }
}

pub async fn print_project(project: ProjectCompileOutput<ConfigurableArtifacts>) -> Result<()> {
    let artifacts = project.into_artifacts();
    for (id, artifact) in artifacts {
        let name = id.name;
        let abi = artifact.abi.context("No ABI found for artifact {name}")?;

        println!("{}", "=".repeat(80));
        println!("CONTRACT: {}", name);

        let contract = &abi.abi;
        let functions = contract.functions();
        let functions = functions.cloned();
        let constructor = contract.constructor();

        if let Some(constructor) = constructor {
            let args = &constructor.inputs;
            println!("CONSTRUCTOR args: {args:?}");
        }

        for func in functions {
            let name = func.name;
            let params = &func.inputs;
            println!("FUNCTION: {name} {params:?}");
        }
    }
    Ok(())
}
