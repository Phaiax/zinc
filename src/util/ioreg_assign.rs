// Zinc, the bare metal stack for rust.
// Copyright 2016 Daniel Seemer 'phaiax' <phaiax-zinc@invisibletower.de>
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

//! Create accessor function REGISTR() for an instance of Registr at linker label $linkname.

macro_rules! ioreg_assign {
  ($linkname:ident, $name:ident, $regcls:ty) => (

    extern {
      #[link_name="$linkname"]
      pub static $linkname: $regcls ;
    }

    #[allow(non_snake_case, dead_code)]
    /// Placement getter for register $reg at address $linkname
    #[inline(always)]
    pub fn $name() -> &'static $regcls {
        unsafe { & $linkname }
    }

  )
}