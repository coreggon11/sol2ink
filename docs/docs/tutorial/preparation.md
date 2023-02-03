---
sidebar_position: 1
title: Preparation
---

In this tutorial, we will transpile the [ERC-20 contract](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/ERC20.sol) from OpenZeppelin.

#### Preparation

Before we run the program, we will make some adjustments to the ERC-20 file. We will add the events from the [IERC-20 interface](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/IERC20.sol) to our contract. We do this because later in the contract, we want to emit these events. Another thing we will change is to rewrite all functions `_msgSender()` to `msg.sender`. We do this because we want to demonstrate how to call `msg.sender` in ink!.

#### Running Sol2Ink

Running Sol2Ink is easy. First we need to navigate to the folder where we saved `sol2ink` and we will call `./sol2ink path`, where `path` is the path to our `ERC20` file. The output file will be stored in the newly created folder `generated`, containing all of the files of the contract. Now we will try to build it!
