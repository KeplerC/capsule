/*
* Copyright 2019 Comcast Cable Communications Management, LLC
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
* http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*
* SPDX-License-Identifier: Apache-2.0
*/

use super::{Batch, Disposition};
use crate::Result;

/// A batch that calls a closure on packets in the underlying batch.
pub struct Inspect<B: Batch, F>
where
    F: FnMut(&Disposition<B::Item>) -> Result<()>,
{
    batch: B,
    f: F,
}

impl<B: Batch, F> Inspect<B, F>
where
    F: FnMut(&Disposition<B::Item>) -> Result<()>,
{
    #[inline]
    pub fn new(batch: B, f: F) -> Self {
        Inspect { batch, f }
    }
}

impl<B: Batch, F> Batch for Inspect<B, F>
where
    F: FnMut(&Disposition<B::Item>) -> Result<()>,
{
    type Item = B::Item;

    #[inline]
    fn replenish(&mut self) {
        self.batch.replenish();
    }

    #[inline]
    fn next(&mut self) -> Option<Disposition<Self::Item>> {
        self.batch.next().map(|disp| {
            let _ = (self.f)(&disp);
            disp
        })
    }
}
