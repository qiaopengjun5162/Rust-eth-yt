# Rust-eth-yt

## Explain this code

### simple_transactions

这段代码使用了ethers库来与以太坊网络进行交互。代码的主要功能是通过Ganache节点发送一笔交易，并查询交易前后的账户余额。

代码的具体步骤如下：

1. 导入所需的库。
2. 定义一个异步函数 `main` ，返回一个 `Result` 类型。
3. 定义一个助记词 `mnemonic` ，用于生成钱包。
4. 创建一个Ganache节点，并设置助记词为上一步定义的助记词。
5. 打印Ganache节点的HTTP端点地址。
6. 从Ganache节点获取钱包的密钥，并转换为本地钱包类型。
7. 获取钱包的第一个地址，并打印该地址。
8. 创建一个Provider，连接到Ganache节点的端点，并设置轮询间隔为10毫秒。
9. 使用Provider查询第一个地址的余额，并打印余额。
10. 定义一个随机的账户地址，并将其转换为Address类型。
11. 使用Provider查询随机账户的余额，并打印余额。
12. 创建一个交易请求，向随机账户发送1000个以太币，并设置发送者为第一个地址。
13. 使用Provider发送交易，并等待交易确认。
14. 打印交易所在的区块号。
15. 查询随机账户的余额，并打印余额。
16. 返回Ok，表示程序执行成功。

总结：这段代码使用ethers库连接到Ganache节点，发送一笔交易，并查询交易前后的账户余额。

### contract_deploy

这段代码是一个使用ethers库与以太坊进行交互的Rust程序。以下是代码的步骤：

1. 导入所需的库。
2. 定义了一个类型别名`SignerDeployedContract<T>`，表示一个已部署的合约。
3. 定义了一个async的main函数。
4. 设置助记词。
5. 创建一个Ganache实例，并使用助记词生成钱包。
6. 打印Ganache的HTTP端点和钱包的第一个地址。
7. 创建一个Provider实例，并获取Ganache的链ID。
8. 编译Solidity项目。
9. 打印项目的信息。
10. 获取钱包地址的余额。
11. 定义要部署的合约名称。
12. 从项目中找到合约。
13. 获取合约的ABI和字节码。
14. 创建一个签名客户端。
15. 部署合约。
16. 打印部署后的合约地址。

compile函数的作用是编译Solidity项目。它接受一个项目根目录的路径作为参数，并返回一个包含编译输出的结果。

print_project函数用于打印项目的信息。它接受一个`ProjectCompileOutput<ConfigurableArtifacts>`类型的参数，并打印出每个合约的名称、构造函数和函数。

总之，这段代码的目的是编译Solidity项目并部署一个合约。

## 运行

```shell
rust-eth-yt on  master [?] is 📦 0.1.0 via 🦀 1.72.1 via 🅒 base 
➜ cargo run --bin transact                  
   Compiling rust-eth-yt v0.1.0 (/Users/qiaopengjun/Code/rust/rust-eth-yt)
    Finished dev [unoptimized + debuginfo] target(s) in 2.01s
     Running `target/debug/transact`
HTTP Ganache Endpoint: http://localhost:62063
wallet first address: 329399cc650af0a81ef92b6c66292bc237407960
wallet first address balance: 100000000000000000000
other address: 0xaf206dCE72A0ef76643dfeDa34DB764E2126E646 balance: 0
Pending transfer: 0x7aebc4611e96b891c8caa7353b874f4ff44f9f188b0c2f648b9c7604e8dde5d4
TX mined in block 1
Balance of address: 0xaf206dCE72A0ef76643dfeDa34DB764E2126E646 balance: 1000


➜ cargo run --bin deploy
   Compiling rust-eth-yt v0.1.0 (/Users/qiaopengjun/Code/rust/rust-eth-yt)
    Finished dev [unoptimized + debuginfo] target(s) in 2.51s
     Running `target/debug/deploy`
HTTP Ganache Endpoint: http://localhost:60998
wallet first address: 3d15b4e21871114b8ddec7785cc0e625e4b8b01e
Ganache started with chain_id: 1337
Compiling solidity project: "examples/"
================================================================================
CONTRACT: BUSDImplementation
CONSTRUCTOR args: []
FUNCTION: EIP712_DOMAIN_HASH []
FUNCTION: allowance [Param { name: "_owner", kind: Address, internal_type: None }, Param { name: "_spender", kind: Address, internal_type: None }]
FUNCTION: approve [Param { name: "_spender", kind: Address, internal_type: None }, Param { name: "_value", kind: Uint(256), internal_type: None }]
FUNCTION: assetProtectionRole []
FUNCTION: balanceOf [Param { name: "_addr", kind: Address, internal_type: None }]
FUNCTION: betaDelegateWhitelister []
FUNCTION: betaDelegatedTransfer [Param { name: "sig", kind: Bytes, internal_type: None }, Param { name: "to", kind: Address, internal_type: None }, Param { name: "value", kind: Uint(256), internal_type: None }, Param { name: "fee", kind: Uint(256), internal_type: None }, Param { name: "seq", kind: Uint(256), internal_type: None }, Param { name: "deadline", kind: Uint(256), internal_type: None }]
FUNCTION: betaDelegatedTransferBatch [Param { name: "r", kind: Array(FixedBytes(32)), internal_type: None }, Param { name: "s", kind: Array(FixedBytes(32)), internal_type: None }, Param { name: "v", kind: Array(Uint(8)), internal_type: None }, Param { name: "to", kind: Array(Address), internal_type: None }, Param { name: "value", kind: Array(Uint(256)), internal_type: None }, Param { name: "fee", kind: Array(Uint(256)), internal_type: None }, Param { name: "seq", kind: Array(Uint(256)), internal_type: None }, Param { name: "deadline", kind: Array(Uint(256)), internal_type: None }]
FUNCTION: claimOwnership []
FUNCTION: decimals []
FUNCTION: decreaseSupply [Param { name: "_value", kind: Uint(256), internal_type: None }]
FUNCTION: disregardProposeOwner []
FUNCTION: freeze [Param { name: "_addr", kind: Address, internal_type: None }]
FUNCTION: increaseSupply [Param { name: "_value", kind: Uint(256), internal_type: None }]
FUNCTION: initialize []
FUNCTION: initializeDomainSeparator []
FUNCTION: isFrozen [Param { name: "_addr", kind: Address, internal_type: None }]
FUNCTION: isWhitelistedBetaDelegate [Param { name: "_addr", kind: Address, internal_type: None }]
FUNCTION: name []
FUNCTION: nextSeqOf [Param { name: "target", kind: Address, internal_type: None }]
FUNCTION: owner []
FUNCTION: pause []
FUNCTION: paused []
FUNCTION: proposeOwner [Param { name: "_proposedOwner", kind: Address, internal_type: None }]
FUNCTION: proposedOwner []
FUNCTION: reclaimBUSD []
FUNCTION: setAssetProtectionRole [Param { name: "_newAssetProtectionRole", kind: Address, internal_type: None }]
FUNCTION: setBetaDelegateWhitelister [Param { name: "_newWhitelister", kind: Address, internal_type: None }]
FUNCTION: setSupplyController [Param { name: "_newSupplyController", kind: Address, internal_type: None }]
FUNCTION: supplyController []
FUNCTION: symbol []
FUNCTION: totalSupply []
FUNCTION: transfer [Param { name: "_to", kind: Address, internal_type: None }, Param { name: "_value", kind: Uint(256), internal_type: None }]
FUNCTION: transferFrom [Param { name: "_from", kind: Address, internal_type: None }, Param { name: "_to", kind: Address, internal_type: None }, Param { name: "_value", kind: Uint(256), internal_type: None }]
FUNCTION: unfreeze [Param { name: "_addr", kind: Address, internal_type: None }]
FUNCTION: unpause []
FUNCTION: unwhitelistBetaDelegate [Param { name: "_addr", kind: Address, internal_type: None }]
FUNCTION: whitelistBetaDelegate [Param { name: "_addr", kind: Address, internal_type: None }]
FUNCTION: wipeFrozenAddress [Param { name: "_addr", kind: Address, internal_type: None }]
================================================================================
CONTRACT: Migrations
FUNCTION: last_completed_migration []
FUNCTION: setCompleted [Param { name: "completed", kind: Uint(256), internal_type: None }]
FUNCTION: upgrade [Param { name: "new_address", kind: Address, internal_type: None }]
================================================================================
CONTRACT: AddressUtils
================================================================================
CONTRACT: AdminUpgradeabilityProxy
CONSTRUCTOR args: [Param { name: "_implementation", kind: Address, internal_type: None }]
FUNCTION: admin []
FUNCTION: changeAdmin [Param { name: "newAdmin", kind: Address, internal_type: None }]
FUNCTION: implementation []
FUNCTION: upgradeTo [Param { name: "newImplementation", kind: Address, internal_type: None }]
FUNCTION: upgradeToAndCall [Param { name: "newImplementation", kind: Address, internal_type: None }, Param { name: "data", kind: Bytes, internal_type: None }]
================================================================================
CONTRACT: Proxy
================================================================================
CONTRACT: SafeMath
================================================================================
CONTRACT: UpgradeabilityProxy
CONSTRUCTOR args: [Param { name: "_implementation", kind: Address, internal_type: None }]
Wallet first address 3d15b4e21871114b8ddec7785cc0e625e4b8b01e balance: 100000000000000000000
Error: Failed to get the base fee for the next block

Location:
    src/contract_deploy.rs:75:10

```
