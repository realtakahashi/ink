// Copyright 2018-2019 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(const_fn)]

#[cfg(feature = "ink-generate-abi")]
mod abi;

mod contract;
mod cross_calling;
mod dispatcher;
mod env_access;
mod error;
mod testable;
mod traits;

pub use ink_lang_macro::contract;

#[cfg(feature = "ink-generate-abi")]
pub use self::abi::GenerateAbi;

pub use self::{
    contract::{
        Contract,
        ContractBuilder,
        DispatchMode,
        DispatchUsingMode,
    },
    cross_calling::{
        ForwardCall,
        ForwardCallMut,
        ToAccountId,
    },
    dispatcher::{
        Dispatch,
        DispatchList,
        DispatchableFn,
        DispatchableFnMut,
        Dispatcher,
        DispatcherMut,
        EmptyDispatchList,
        PushDispatcher,
        UnreachableDispatcher,
    },
    env_access::{
        Env,
        EnvAccess,
        StaticEnv,
    },
    error::{
        DispatchError,
        DispatchResult,
        DispatchRetCode,
    },
    testable::InstantiateTestable,
    traits::{
        FnInput,
        FnOutput,
        FnSelector,
        Message,
        Storage,
    },
};
