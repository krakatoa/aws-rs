pub struct BlockInfo {
  hash: f64, // make it a String
  size: f64,  // make it a u8
  offset: f64, // make it a u8
  leaf_blocks_info: Vec<BlockInfo>
}

impl Clone for BlockInfo {
  fn clone(&self) -> BlockInfo {
    let mut leaf_blocks = self.leaf_blocks_info.iter().map(|b| (*b).clone());
    BlockInfo {
      hash: self.hash,
      size: self.size,
      offset: self.offset,
      leaf_blocks_info: leaf_blocks.collect()
    }
  }
}

fn calc_tree_hash(size: f64, offset: f64) -> BlockInfo {
  let mut root_block_info: BlockInfo = BlockInfo {
    hash: 0.0,
    size: size,
    offset: offset,
    leaf_blocks_info: vec![]
  };

  if size <= 1.0 {
    let hash: f64 = size;

    root_block_info.hash = hash;
    root_block_info.leaf_blocks_info.push(BlockInfo {
      hash: hash,
      size: size,
      offset: offset,
      leaf_blocks_info: vec![]
    });
  } else if size > 1.0 && size < 2.0 {
    let a_block_info = calc_tree_hash(1.0, offset);
    for block in a_block_info.leaf_blocks_info.iter() {
      root_block_info.leaf_blocks_info.push((*block).clone());
      root_block_info.hash += block.hash;
    };

    let b_block_info = calc_tree_hash(size - 1.0, offset + 1.0);
    for block in b_block_info.leaf_blocks_info.iter() {
      root_block_info.leaf_blocks_info.push((*block).clone());
      root_block_info.hash += block.hash;
    };
  } else {
    let tail: f64 = size % 2.0;
    let head: f64 = size - tail;

    if tail > 0.0 {
      let mut a_block_info = calc_tree_hash(head, offset);
      root_block_info.hash += a_block_info.hash;
      root_block_info.leaf_blocks_info.push(a_block_info);

      let mut c_block_info = calc_tree_hash(tail, head);
      root_block_info.hash += c_block_info.hash;
      root_block_info.leaf_blocks_info.push(c_block_info);
    } else {
      let mut a_block_info = calc_tree_hash(head / 2.0, offset);
      root_block_info.hash += a_block_info.hash;
      root_block_info.leaf_blocks_info.push(a_block_info);

      let mut b_block_info = calc_tree_hash(head / 2.0, offset + head / 2.0);
      root_block_info.hash += b_block_info.hash;
      root_block_info.leaf_blocks_info.push(b_block_info);
    }
  }

  root_block_info
}

fn main() {
  let s: f64 = 8.5;
  let root_block_info: BlockInfo = calc_tree_hash(s, 0.0);

  println!("Root Hash: {}", root_block_info.hash);
  println!("Root Size: {}", root_block_info.size);
  println!("Root Offset: {}\n", root_block_info.offset);

  for b in root_block_info.leaf_blocks_info.iter() {
    println!("Hash: {}", b.hash);
    println!("Offset: {}", b.offset);
    println!("Size: {}\n", b.size);

    for c in b.leaf_blocks_info.iter() {
      println!("  Hash: {}", c.hash);
      println!("  Offset: {}", c.offset);
      println!("  Size: {}\n", c.size);

      for d in c.leaf_blocks_info.iter() {
        println!("    Hash: {}", d.hash);
        println!("    Offset: {}", d.offset);
        println!("    Size: {}\n", d.size);
      }
    }
  }

  println!("finished")
}
