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

use crate::ParamABI;

pub trait HasComponents {
    fn has_components() -> bool {
        false
    }
}

pub trait GenerateComponents {
    fn generate_components() -> Vec<ParamABI>;
}

macro_rules! impl_primitive_tys {
    ($( $t:ty ),*) => {
        $(
            impl GenerateComponents for $t {
                fn generate_components() -> Vec<ParamABI> {
                    Vec::new()
                }
            }

            impl HasComponents for $t {}
        )*
    };
}
impl_primitive_tys!(bool, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, String);
