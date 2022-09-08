#![allow(unused_variables, dead_code, unused_imports)]

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::sync::{Arc, Mutex, RwLock};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Derivative)]
#[derivative(Default)]
struct ItemPtr {
    #[derivative(Default(value = "-1"))]
    page_no: i32,
    #[derivative(Default(value = "0"))]
    offset: usize,
}

impl ItemPtr {
    pub fn new(page_no: i32, offset: usize) -> Self {
        ItemPtr { page_no, offset }
    }
}

impl Display for ItemPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ItemPtr [page_no={} offset={}]",
            self.page_no, self.offset
        )
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Node<T> {
    leaf: bool,
    order: u32,
    loc: ItemPtr,
    link: ItemPtr,
    high_key: T,
    keys: Vec<T>,
    children: Vec<ItemPtr>,
}

impl<'a, T> Node<T>
where
    T: Deserialize<'a> + Serialize + Ord + Clone + Copy,
{
    pub fn encode(self) -> Option<Vec<u8>> {
        let item = bincode::serialize(&self);
        match item {
            Ok(item) => Some(item),
            Err(_) => None,
        }
    }

    pub fn decode(bytes: &'a Vec<u8>) -> Option<Self> {
        let page = bincode::deserialize(&bytes[..]);
        match page {
            Ok(page) => Some(page),
            Err(_) => None,
        }
    }

    pub fn new(
        order: u32,
        loc: ItemPtr,
        link: ItemPtr,
        keys: Vec<T>,
        children: Vec<ItemPtr>,
    ) -> Self {
        let high_key = *keys.iter().max().unwrap();
        Node {
            leaf: true,
            order,
            loc,
            link,
            keys,
            children,
            high_key,
        }
    }

    pub fn into_buffer(self) -> [u8; 512] {
        let bytes: Vec<u8> = self.encode().unwrap();
        let mut node_buf = [0u8; 512];
        node_buf[..std::mem::size_of_val(&*bytes)].copy_from_slice(&bytes);
        node_buf
    }
}

impl<T> Display for Node<T>
where
    T: Display + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node [leaf={} order={} loc={} link={} high_key={} \nkeys={:#?} \nchildren={:#?}\n]",
            self.leaf, self.order, self.loc, self.link, self.high_key, self.keys, self.children
        )
    }
}

type Page = Arc<Mutex<[u8; 512]>>;

fn main() {
    let first = ItemPtr::new(1, 2);
    let second = ItemPtr::new(1, 2);
    let third = ItemPtr::new(1, 2);
    let fourth = ItemPtr::new(1, 2);
    let fifth = ItemPtr::new(1, 2);
    let sixth = ItemPtr::new(1, 2);
    let seventh = ItemPtr::new(1, 2);
    let eighth = ItemPtr::new(1, 2);

    let test_node = Node::new(
        2,
        first,
        second,
        vec![1, 2, 3, 4],
        vec![third, fourth, fifth, sixth, seventh],
    );

    println!("{}", std::mem::size_of_val(&first));
    println!("{}", std::mem::size_of_val(&test_node));

    let raw: [u8; 512] = test_node.into_buffer();
    println!("{:#?}", raw);
    let raw_bytes: Vec<u8> = Vec::from(raw);
    let raw_decoded = Node::<u32>::decode(&raw_bytes).unwrap();
    println!("RAW DECODED: {}", raw_decoded);
}
