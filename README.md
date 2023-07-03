# <img src="https://cryptologos.cc/logos/solana-sol-logo.png?v=025" with="25" height="25"> NFT staking program on Solana Blockchain (constant reward)

## Description

This is an NFT staking program with constant reward calculated using simple formular below.

$$ reward = {  {t_{now} - t_{claimed} \over 86400} * R}$$

where 
* t_now = timestamp of claiming time
* t_claimed = timestamp of the previous claiming
* R = constant value of reward per day

There are two mains accounts (if you are not familiar with the "account" concept of Solana, you can check it out [here](https://medium.com/@lianxiongdi/a-deep-dive-into-solana-account-model-1-introduction-7b0408656593)). 

- pool account : will be owned by owner of pool reward.
- vault account : 
    - an escrow account where it will be temporary own the nft of the staker. 
    - This account will be owned by staker.
    - use to hold the information using for calcurating reward.
-----

## Usage
### Setup
---
Please input the first creator address of NFTs collection you wish to whitelist where they can be allowed to be staked.

For example, if you wish to whitelist all NFTs with the first creator address of ```D3XrkNZz6wx6cofot7Zohsf2KSsu2ArngNk8VqU9cTY3``` you can input this address for ```EXPECTED_NFT_CREATOR_ADDRESS``` in ```the program/nft-staking/src/constant/address.rs.``` 

In the same file, don't forget to input the token address you wish to use as a token reward. 

```
pub const METADATA_PROGRAM_ID: &'static str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";
pub const EXPECTED_NFT_CREATOR_ADDRESS: &'static str = ""; //nft first verified creator for the collection
pub const REWARD_TOKEN_ADDRESS: &'static str = ""; // reward token address
```

After that, config reward amount per day in the file ```programs/nft-staking/src/constant/reward.rs```. Don't forget to take token-decimals into accounts. For example, if you want to use $Bonk token as a reward, you can see from [here](https://solscan.io/token/DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263) that $Bonk has a decimals of 5. If you wish to reward staker with 100 $Bonk per day then you should set REWARD_PER_DAY as 100x(10^5)
```
pub const REWARD_PER_DAY: i64 = 10_000_000;
```

If you haven't have anchor installed, please refer to this [doccument](https://www.anchor-lang.com/docs/installation). Don't forget to install Solana Cli also!

After things have been set then run.
```
anchor build
```

### Get your own program address.
---
As you can see, the program address is still ```STKh7qfE1zp635vi1gBReVpR4ijsbqLFgrkKyo1sbLf``` where you won't have the keypair of this address you wish to deploy to. So, either get your program address from the Pubkey of ```target/deploy/nft_staking-keypair.json``` or you can create your own vanity program or just a random plain program address.

You can simply run 
```
solana-keygen new
```
or
```
solana-keygen grind --starts-with stk:1 
```
for generating a vanity address starts with "stk" (you can replace stk with any other words you like)


Anyway, after you got the program keypair please copy the keypair array and replace the one in this file ```target/deploy/nft_staking-keypair.json```.

Copy your pubkey of the generated keypair and replace existing program id in ```programs/nft-staking/src/lib.rs```
```
declare_id!("STKh7qfE1zp635vi1gBReVpR4ijsbqLFgrkKyo1sbLf");
```
replace this with the new ones, my guy.

Also, don't forget to do the same with ```Anchor.toml```.

### Deployment
---
create or use your existing keypair (don't use the same keypair as the program address) that you will use as an update authority. Your authority keypair should be renamed to ```update_auth.json``` and place it in the root directory as I configed the wallet path to "update_auth.json" in ```Anchor.toml```

Before deploy or upgrade the program, don't forget to send SOL to "update_auth" wallet. Test on devnet first to see how much SOL it will cost in order to deploy and update. (I have set the default cluster to devnet if you wish to deploy on mainnet please set cluster to "mainnet" in Anchor.toml file.)

run 
```
anchor deploy
```

### Program update
---
run
```
anchor update --program-id STKh7qfE1zp635vi1gBReVpR4ijsbqLFgrkKyo1sbLf target/deploy/nft_staking.so 
```

# Disclaimer
This program has low flexibility as it accept only a set of NFTs with the same first creators address and still use reward as an SPL token as I try to make it as simple as possible so that you can cuztomize on top of it as you please.

This program is still unaudit but I have already checked for the possible exploitation as much as possible. 
