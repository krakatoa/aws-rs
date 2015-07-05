use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::{Seek, SeekFrom, Read, Take, Bytes};

extern crate crypto;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

extern crate rustc_serialize;
use rustc_serialize::json;
use rustc_serialize::json::{Json, Parser};

pub struct BlockInfo {
  hash: String,
  size: u64,
  offset: u64,
  leaf_blocks_info: Vec<BlockInfo>,
  chunk_blocks_info: Vec<BlockInfo>
}

impl Clone for BlockInfo {
  fn clone(&self) -> BlockInfo {
    let mut leaf_blocks = self.leaf_blocks_info.iter().map(|b| (*b).clone());
    BlockInfo {
      hash: self.hash.clone(),
      size: self.size,
      offset: self.offset,
      leaf_blocks_info: leaf_blocks.collect(),
      chunk_blocks_info: vec![]
    }
  }
}

fn calc_hash(path: &Path, size: u64, offset: u64) -> String {
  let mut sha = Sha256::new();

  let mut f = match File::open(path) {
    Err(e) => panic!("couldn't open {}", path.display()),
    Ok(file) => file,
  };

  let seek_from: SeekFrom = SeekFrom::Start(offset);

  match f.seek(seek_from) {
    Err(e) => panic!("couldn't seek file at {offset}"),
    Ok(_) => ()
  };

  let mut reader: Take<File> = f.take(size);
  let s: &mut [u8; 1024*1024] = &mut [0; 1024*1024];
  match reader.read(s) {
    Err(e) => panic!("could not read partial file at {}", offset),
    Ok(_) => sha.input(s)
  }

  sha.result_str()
}

fn merge_hashes(a_hash: String, b_hash: String) -> String {
  let mut m_hash = a_hash.clone();
  m_hash.push_str(&b_hash[..]);

  let mut sha = Sha256::new();
  sha.input_str(&m_hash[..]);
  sha.result_str()
}

fn calc_tree_hash(path: &Path, size: u64, offset: u64) -> BlockInfo {
  let mut root_block_info: BlockInfo = BlockInfo {
    hash: String::new(),
    size: size,
    offset: offset,
    leaf_blocks_info: vec![],
    chunk_blocks_info: vec![]
  };

  if size <= 1024 * 1024 {
    let block: BlockInfo = BlockInfo {
      hash: calc_hash(path, size, offset),
      size: size,
      offset: offset,
      leaf_blocks_info: vec![],
      chunk_blocks_info: vec![]
    };

    root_block_info.hash = block.hash.clone();
    root_block_info.leaf_blocks_info.push(block.clone());
    root_block_info.chunk_blocks_info.push(block);
  } else if size > 1024 * 1024 && size < 1024 * 1024 * 2 {
    let a_block_info = calc_tree_hash(path, 1024 * 1024, offset);
    for block in a_block_info.leaf_blocks_info.iter() {
      root_block_info.leaf_blocks_info.push((*block).clone());
      root_block_info.chunk_blocks_info.push((*block).clone());
    };

    let b_block_info = calc_tree_hash(path, size - 1024 * 1024, offset + 1024 * 1024);
    for block in b_block_info.leaf_blocks_info.iter() {
      root_block_info.leaf_blocks_info.push((*block).clone());
      root_block_info.chunk_blocks_info.push((*block).clone());
    };

    root_block_info.hash = merge_hashes(a_block_info.hash.clone(), b_block_info.hash.clone());
  } else {
    let tail: u64 = size % (1024 * 1024 * 2);
    let head: u64 = size - tail;

    if tail > 0 {
      let mut a_block_info = calc_tree_hash(path, head, offset);
      for block in a_block_info.chunk_blocks_info.iter() {
        root_block_info.chunk_blocks_info.push((*block).clone());
      };

      let mut c_block_info = calc_tree_hash(path, tail, head);
      for block in c_block_info.chunk_blocks_info.iter() {
        root_block_info.chunk_blocks_info.push((*block).clone());
      };

      root_block_info.hash = merge_hashes(a_block_info.hash.clone(), c_block_info.hash.clone());

      root_block_info.leaf_blocks_info.push(a_block_info);
      root_block_info.leaf_blocks_info.push(c_block_info);
    } else {
      let mut a_block_info = calc_tree_hash(path, head / 2, offset);
      for block in a_block_info.chunk_blocks_info.iter() {
        root_block_info.chunk_blocks_info.push((*block).clone());
      };

      let mut b_block_info = calc_tree_hash(path, head / 2, offset + head / 2);
      for block in b_block_info.chunk_blocks_info.iter() {
        root_block_info.chunk_blocks_info.push((*block).clone());
      };

      root_block_info.hash = merge_hashes(a_block_info.hash.clone(), b_block_info.hash.clone());

      root_block_info.leaf_blocks_info.push(a_block_info);
      root_block_info.leaf_blocks_info.push(b_block_info);
    }
  }

  root_block_info
}

fn main() {
  let path = Path::new("158827.mp4");
  let len = match fs::metadata(path) {
    Ok(r) => r.len(),
    Err(e) => panic!("could not find file")
  };

  let root_block_info: BlockInfo = calc_tree_hash(path, len, 0);

  println!("Root Hash: {}", root_block_info.hash);
  println!("Root Size: {}", root_block_info.size);
  println!("Root Offset: {}\n", root_block_info.offset);

  for b in root_block_info.leaf_blocks_info.iter() {
    println!("Offset: {}", b.offset);
    println!("Size: {}\n", b.size);

    for chunk in b.chunk_blocks_info.iter() {
      println!("  Root Hash: {}", b.hash);
      println!("  Chunk Hash: {}", chunk.hash);
      println!("  Offset: {}", chunk.offset);
      println!("  Size: {}\n", chunk.size);
    }
  }

  println!("finished")
}
