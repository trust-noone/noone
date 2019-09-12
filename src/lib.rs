#[macro_use]
extern crate specs_derive;

use std::net::UdpSocket;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use futures::prelude::*;
use futures::future;
use num_bigint_dig::BigUint;
use rsa::{RSAPrivateKey, RSAPublicKey};
use specs::{Component, VecStorage, DenseVecStorage, System, ReadStorage, WriteStorage};

#[cfg(test)]
mod tests {
    #[test]
    fn update_index() {
    }
}

pub struct Session {
    directory: Directory,
    identity: Identity,
    config: Config,
}

impl Session {
    // This is the part where we need an IP address to start with.
    // Certain people will run public indices that can then point to other indices.
    pub fn new(path: &str) -> Session {
        Session {
            directory: Directory::new(),
            identity: Identity::new(),
            config: Config::new(),
        }
    }

    pub fn directory(&self) -> &Directory {
        &self.directory
    }

    pub fn directory_mut(&mut self) -> &mut Directory {
        &mut self.directory
    }

    pub fn identity(&self) -> &Identity {
        &self.identity
    }

    pub fn identity_mut(&mut self) -> &mut Identity {
        &mut self.identity
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    pub fn fetch(&self, locator: Locator) -> impl Future<Item=u32, Error=u32> {
        // automatically find and use identity this will work for
        future::ok::<u32, u32>(1)
    }

    pub fn fetch_sync(&self, locator: Locator) -> Result<u32, u32> {
        self.fetch(locator).wait()
    }

    pub fn place(&self, locator: Locator) -> impl Future<Item=u32, Error=u32> {
        future::ok::<u32, u32>(1)
    }

    pub fn place_sync(&self, locator: Locator) -> Result<u32, u32> {
        self.place(locator).wait()
    }

    pub fn delete(&self, locator: Locator) -> impl Future<Item=u32, Error=u32> {
        future::ok::<u32, u32>(1)
    }

    pub fn delete_sync(&self, locator: Locator) -> Result<u32, u32> {
        self.delete(locator).wait()
    }
}

enum NetError {
    Timeout,
    PermissionDenied,
}

pub struct Identity {
    hash: Hash,
    pubkey: RSAPublicKey,
    privkey: Option<RSAPrivateKey>,
}

impl Identity {
    pub fn new() -> Identity {
        Identity {
            hash: Hash("".to_string()),
            pubkey: RSAPublicKey::new(BigUint::new(vec![0]), BigUint::new(vec![0])).unwrap(),
            privkey: None,
        }
    }
}

pub struct Config {
}

impl Config {
    pub fn new() -> Config {
        Config {
        }
    }
}

pub enum Locator {
    Hash,
    Absolute,
    Relative,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub enum Fact {
    Blob,
    Identity,
    Directory,
    Voucher,
    Link,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub enum Network {
    Node,
    Request,
    Response,
}

#[derive(Component)]
#[storage(VecStorage)]
struct Name(String);

#[derive(Component)]
#[storage(VecStorage)]
struct Data(Vec<u8>);

#[derive(Component)]
#[storage(VecStorage)]
struct Hash(String);

#[derive(Component)]
#[storage(VecStorage)]
struct Parent(Hash);

#[derive(Component)]
#[storage(VecStorage)]
struct Owner(Hash);

#[derive(Component)]
#[storage(VecStorage)]
struct Requester(Hash);

#[derive(Component)]
#[storage(VecStorage)]
struct Responder(Hash);

#[derive(Component)]
#[storage(VecStorage)]
struct Signature(String);

#[derive(Component)]
#[storage(VecStorage)]
struct IncomingRequest;

#[derive(Component)]
#[storage(VecStorage)]
struct OutgoingRequest;

#[derive(Component)]
#[storage(VecStorage)]
enum RequestDirection {
    Internal,
    External,
}

#[derive(Component)]
#[storage(VecStorage)]
struct Keys {
    pubkey: RSAPublicKey,
    privkey: Option<RSAPrivateKey>,
}

impl Keys {
    fn new() -> Keys {
        Keys {
            pubkey: RSAPublicKey::new(BigUint::new(vec![0]), BigUint::new(vec![0])).unwrap(),
            privkey: None,
        }
    }
}

#[derive(Component)]
#[storage(VecStorage)]
struct Voucher {
    source: String,
    target: String,
}

impl Voucher {
    fn new(source: &str, target: &str) -> Voucher {
        Voucher {
            source: source.to_string(),
            target: target.to_string(),
        }
    }
}

struct SendRequests;

struct ReadResponses;

struct ServeRequests;

impl<'a> System<'a> for ServeRequests {
    type SystemData = (ReadStorage<'a, Fact>,
                       ReadStorage<'a, Requester>,
                       ReadStorage<'a, Responder>,
                       WriteStorage<'a, Data>);

    fn run(&mut self, data: Self::SystemData) {
        let (kind, source, dest, mut response) = data;
    }
}

struct EvictData;

// ECS -- what components do I have?
// name -- String
// data -- arbitrary bytes
// prev -- hash of dependency in history (multiple for merges?)
// hash -- String (or raw byte array?)
// directory -- containing directory
// index -- being accessed through this index
// children -- hashes of other entities
//
// TODO(schottm): just make a voucher component?
// source -- identity extending trust
// dest -- identity being trusted
// signature -- proof source trusts dest
//
// friends -- voucher to/from friend + IP address/socket/whatever

// Should directories and indices be one thing?

// Special files and subdirectories of .noone:
// indices -- external indices
// blobs -- raw files by hash
// identities -- other known users, their friends and contexts, how to find them
// vouchers -- vouchers either to or from you, or rejections
// config.toml -- bandwidth settings, indices, user lists, ignored files, etc.

pub struct Directory {
    children: Vec<Fact>,
}

impl Directory {
    pub fn new() -> Directory {
        Directory {
            children: vec![]
        }
    }

    pub fn add() {
    }

    pub fn remove() {
    }
}
