#![feature(box_patterns)]
// extern crate rand;

use std::panic;
use rand::{rngs::SmallRng, SeedableRng};
use rand::prelude::*;

// ref https://zhuanlan.zhihu.com/p/430867594
#[derive(Debug)]
enum AVLTree {
    Empty,
    Node(Box<AVLTree>, i64, Box<AVLTree>, i64),
}

fn height(x: &AVLTree) -> i64 {
    match x {
        AVLTree::Empty => 0,
        AVLTree::Node(_, _, _, h) => *h,
    }
}

fn create(l: Box<AVLTree>, v: i64, r: Box<AVLTree>) -> Box<AVLTree> {
    let hl = height(&l);
    let hr = height(&r);
    Box::new(AVLTree::Node(
        l,
        v,
        r,
        if hl >= hr { hl + 1 } else { hr + 1 },
    ))
}

fn add(x: i64, tree: Box<AVLTree>) -> Box<AVLTree> {
    match tree {
        box AVLTree::Empty => Box::new(AVLTree::Node(Box::new(AVLTree::Empty), x, Box::new(AVLTree::Empty), 1)),
        box AVLTree::Node(l, v, r, h) => {
            if x == v {
                Box::new(AVLTree::Node(l, v, r, h))
            } else if x < v {
                bal(add(x, l), v, r)
            } else {
                bal(l, v, add(x, r))
            }
        }
    }
}

fn bal(l: Box<AVLTree>, v: i64, r: Box<AVLTree>) -> Box<AVLTree> {
    let hl = height(&l);
    let hr = height(&r);
    if hl > hr + 2 {
        match l {
            box AVLTree::Node(ll, lv, lr, _) if height(&ll) >= height(&lr) => {
                create(ll, lv, create(lr, v, r))
            },
            box AVLTree::Node(ll, lv, box AVLTree::Node(lrl,lrv,lrr,_), _) =>{
                create(create(ll, lv, lrl), lrv, create(lrr, v, r))
            }
            _ => panic!()
        }
    } else if hr > hl + 2{
        match r {
            box AVLTree::Node(rl, rv, rr, _) if height(&rr) >= height(&rl) => {
                create(create(l, v, rl), rv, rr)
            },
            box AVLTree::Node(box AVLTree::Node(rll,rlv,rlr, _), rv, rr, _) =>{
                create(create(l, v, rll), rlv, create(rlr, rv, rr))
            }
            _ => panic!()
        }
    } else {
        create(l, v, r)
    }
}

fn main() {
    let mut tree = Box::new(AVLTree::Empty);
    let mut rng = SmallRng::seed_from_u64(42);
    for _ in 0..100{
        let num = rng.gen_range(0..10000);
        tree = add(num, tree)
    }
    println!("{:?}", tree)
}
