use ndarray::Array2;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::structs::vector::Vector3D;
use crate::structs::cycle::Cycle;

pub type Adjacency = HashMap<u32, HashSet<u32>>;
pub type Bobbins = Vec<u32>;
pub type Loom = Vec<VecDeque<u32>>;
pub type WarpedLoom<'a> = HashMap<usize, &'a mut Cycle<'a>>;
pub type Spool = HashMap<u32, Array2<i32>>;
pub type Vert2d = (i32, i32);
pub type Edge = (u32, u32);
pub type Edges = HashSet<Edge>;
pub type EdgeAdjacency = HashMap<Edge, HashSet<Edge>>;
pub type Path = Vec<u32>;
pub type Processed = HashSet<usize>;
pub type Solution = Vec<u32>;
pub type Thread = VecDeque<u32>;
pub type Vectors3d = Vec<Vector3D>;
pub type VertIdx<'a> = HashMap<&'a Vector3D, u32>;
pub type Verts2d = Vec<Vert2d>;
pub type Wefts = Vec<VecDeque<u32>>;
pub type Weights = HashMap<u32, i32>;
pub type Yarn = Array2<i32>;