// Copyright 2019-2020 Parity Technologies (UK) Ltd.
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

use super::{
    Bits256,
    Bitvec as StorageBitvec,
};
use crate::storage2::{
    traits::{
        forward_clear_packed,
        forward_pull_packed,
        forward_push_packed,
        KeyPtr,
        PackedLayout,
        SpreadLayout,
    },
    Pack,
    Vec as StorageVec,
};
use ink_primitives::Key;

impl SpreadLayout for Bits256 {
    const FOOTPRINT: u64 = 1;
    const REQUIRES_DEEP_CLEAN_UP: bool = false;

    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        forward_pull_packed::<Self>(ptr)
    }

    fn push_spread(&self, ptr: &mut KeyPtr) {
        forward_push_packed::<Self>(self, ptr)
    }

    fn clear_spread(&self, ptr: &mut KeyPtr) {
        forward_clear_packed::<Self>(self, ptr)
    }
}

impl PackedLayout for Bits256 {
    fn pull_packed(&mut self, _at: &Key) {}
    fn push_packed(&self, _at: &Key) {}
    fn clear_packed(&self, _at: &Key) {}
}

impl SpreadLayout for StorageBitvec {
    const FOOTPRINT: u64 = 1 + <StorageVec<Pack<Bits256>> as SpreadLayout>::FOOTPRINT;

    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        Self {
            len: SpreadLayout::pull_spread(ptr),
            bits: SpreadLayout::pull_spread(ptr),
        }
    }

    fn push_spread(&self, ptr: &mut KeyPtr) {
        SpreadLayout::push_spread(&self.len, ptr);
        SpreadLayout::push_spread(&self.bits, ptr);
    }

    fn clear_spread(&self, ptr: &mut KeyPtr) {
        SpreadLayout::clear_spread(&self.len, ptr);
        SpreadLayout::clear_spread(&self.bits, ptr);
    }
}
