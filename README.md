# NFT staking program on Solana chain (constant reward)

This program vault state account to store the info of the staker. Basically, when user invoke staking transaction it will embed these info 
* vault owner (for query convenience)
* pool (pool account address for query convenience)
* token address of NFT
* time when user invoke staking transaction
* recent reward claiming time (for calculating reward)

on chain along with transfer the nft of the user to vault token account (token account own by vault state account)

When claiming, reward will be calculated using simple proportion equation below

$$ reward = {  {t_{now} - t_{claimed} \over 86400} * R}$$

where 
* t_now = timestamp of claiming time
* t_claimed = timestamp of the previous claiming
* R = constant value of reward per day

This program has not been audited yet and the frontend side is still in the development.
