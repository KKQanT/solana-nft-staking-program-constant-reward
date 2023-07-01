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
install, run, "How to use the project" and Frontend section will comming soon as soon as I have time. 