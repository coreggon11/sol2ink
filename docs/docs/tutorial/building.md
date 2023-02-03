---
sidebar_position: 2
title: Building the ink! smart contract
---

To build the ink! smart contract we will need [cargo-contract](https://github.com/paritytech/cargo-contract). Note that the generated contracts use ink! version 3.4.0, so we will need to have `cargo-contract` version `1.5.1` installed. So if we satisfy this condition, we will navigate to the generated folder `generated/contracts/erc_20` and call cargo contract build. The contract will start building; we will wait for a while and...

It fails! So let's look at the issue.

The first issue looks like this:

![issue0](https://user-images.githubusercontent.com/43150707/216478364-737a042a-bfde-4061-b879-0e11f8e46b64.png)

The original code said `_balances[account] = accountBalance - amount;`, and Sol2Ink transpiled it as `self.data().balances.insert(&(account), &account_balance - amount);`, forgetting to put the argument of insert into parenthesis. We will ewritw it to `self.data().balances.insert(&(account), &(account_balance - amount));` (adding parenthesis), and the issue is gone.

Another issue will look like this:

![issue1](https://user-images.githubusercontent.com/43150707/216478810-85683b50-a944-435c-89dc-fb6c72266209.png)

The Solidity expression `type(uint256).max` is parsed as `type_of(u128)?.max`. The correct form is `u128::MAX`, so we will rewrite it to that form. And try to build.

And we failed again.

![issue2](https://user-images.githubusercontent.com/43150707/216479049-fdc2359b-1eb1-462d-ace6-3d0f4ded5074.png)

These issues have the same reason; we want to return a String which is the type that can not be copied. Fixing both of these issues is simple; we will return the clones of these strings by calling `.clone()` on them. 

The last issues:

![issue3](https://user-images.githubusercontent.com/43150707/216480272-4d8c9546-1ccc-4164-bd42-7c7868aa960d.png)

These occur because the original contract has some bases, which we did not provide, but Sol2Ink assumed we did, so it tried to implement them for the contract. We will just remove these line from the contract. Now when we build, everything works! Congratulations!

### Warnings

You could have noticed some warnings. The cause of these warnings is that Sol2Ink implicitly sets all variables as mutable, even if we do not mutate them, so we will simply remove the `mut` keyword on the highlighted places. Also, there are some functions, which have parameters that are unused inside that function. It is not an issue, but if we want to remove these warnings, we will add `_` to the front of the names of these parameters (or rename them to `_`), implying that those parameters are unused. 

### More things to notice

Sol2Ink is able to parse most of the Solidity contracts, however, there may be some code which we did not have the chance to cover properly yet. It will be parsed, but there is a way in ink! on how to handle these cases specifically. We are working on it to make Sol2Ink even more usable and we will cover them in the `Known issues` section. And that's it! Now it is the developer's job to optimize the contract for Rust and ink!, but the dirty work is already behind us!