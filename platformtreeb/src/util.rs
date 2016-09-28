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

use std::path::Path;
use std::fs::{File, DirBuilder};
use std::io::{Write, Result, ErrorKind, Error};
use std::collections::HashMap;
use std::ops::{Add, Sub};

/// Writes data into filename. The current working dir is
/// [CARGO_MANIFEST_DIR](http://doc.crates.io/environment-variables.html)
/// , so relative paths are fine.
pub fn write<P: AsRef<Path>>(data : &str, filename : P) -> Result<()> {
    let filename = filename.as_ref();
    if let Some(dir) = filename.parent() {
        if dir.to_string_lossy().len() > 0 {   
            if !dir.exists() {
                DirBuilder::new().recursive(true).create(dir).unwrap();
            }
            if !dir.is_dir() {
                return Err(Error::new(ErrorKind::AlreadyExists, format!("{:?} is a file but it should be a dir", dir)));
            }
        }
    }
    let mut f = try!(File::create(filename));
    f.write_all(data.as_bytes())
}

#[derive(Clone, Copy)]
pub struct Bytes(pub u32);

impl Bytes {
    pub fn b(b : u32) -> Bytes {
        Bytes(b)
    }
    pub fn k(kb : u32) -> Bytes {
        Bytes(kb * 1024)
    }
    pub fn str(&self) -> String {
        if self.0 % 1024 == 0 && self.0 != 0 {
            format!("{}K", self.0 / 1024)
        } else {
            format!("{}", self.0)
        }
    }
}

impl Add for Bytes {
    type Output = Bytes;

    fn add(self, rhs: Bytes) -> Bytes {
        Bytes::b(self.0 + rhs.0)
    }
}

impl Sub for Bytes {
    type Output = Bytes;

    fn sub(self, rhs: Bytes) -> Bytes {
        Bytes::b(self.0 - rhs.0)
    }
}


/// Some inefficient but sufficient template engine
pub struct Template<'a> {
    template : &'a str,
    replace : HashMap<String, String>
}

impl<'a> Template<'a> {
    pub fn new(template : &'a str) -> Template<'a> {
        Template {
            template : template,
            replace : HashMap::new(),
        }
    }
    pub fn replace(&mut self, param : &str, value : &str) {
        self.replace.insert(param.into(), value.into());
    }
    pub fn render(&self) -> String {
        let mut curr = self.template.to_string();
        for (k, v) in self.replace.iter() {
            curr = curr.replace(k, v);
        }
        curr
    }
}