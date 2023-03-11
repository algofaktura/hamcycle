use crate::graph::structs::Cycle;
use ndarray::{Array2, Array3};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

pub type Adjacency = HashMap<Node, Nodes>;
pub type AdjC<'a> = [(Node, &'a [Node])];
pub type Bobbins = Vec<Node>;
pub type Count = usize;
pub type Done = HashSet<usize>;
pub type Edge = (Node, Node);
pub type Edges = HashSet<Edge>;
pub type EdgeAdjacency = HashMap<Edge, HashSet<Edge>>;
pub type Idx = Count;
pub type Idxs = Vec<Idx>;
pub type Loom = Vec<Thread>;
pub type Neighbors = HashSet<Node>;
pub type Node = u32;
pub type Nodes = HashSet<Node>;
pub type Order = u32;
pub type Point = i32;
pub type Points = HashSet<Point>;
pub type Solution = Tour;
pub type Spool = HashMap<u32, Yarn>;
pub type Subtours = Vec<Tour>;
pub type Tour = Vec<Node>;
pub type TourSlice<'a> = &'a [Node];
pub type Thread = VecDeque<Node>;
pub type V2d = [Point; 2];
pub type V2Slice<'a> = &'a [V2d];
pub type V3d = [Point; 3];
pub type V3Slice<'a> = &'a [V3d];
pub type Varr = Vec<V3d>;
pub type Vert = (i32, i32, i32);
pub type Verts = Vec<Vert>;
pub type Vert2d = (Point, Point);
pub type Vert3d = (Point, Point, Point);
pub type Vert2dd = Vec<Vert2d>;
pub type Vert3dd = Vec<Vert3d>;
pub type VertsC2 = [Vert2d];
pub type VertsC3 = [Vert3d];
pub type VIMap = HashMap<Vert, Node>;
// pub type WarpedLoom<'a> = HashMap<usize, RefCell<&'a mut Cycle<'a>>>;
pub type WarpedLoom<'a> = HashMap<usize, &'a mut Cycle<'a>>;
pub type Warps = Subtours;
pub type WarpWefts = [Loom];
pub type Wefts = [Thread];
pub type Weights = HashMap<Node, i32>;
pub type Woven = Vec<usize>;
pub type Yarn = Array2<Point>;
pub type Yarn3 = Array3<Point>;
pub type ZlevelNodesMap = HashMap<Point, Nodes>;
pub type ZOrder = Vec<(Point, usize)>;

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Graph{
//     // pub phantom: PhantomData<&'a ()>,
//     pub verts: String, // Verts
//     pub vi_map: String, // VIMap
//     pub adj: String, // Adjacency
//     pub edge_adj: String, // EdgeAdjacency
//     pub z_adj: String, // Adjacency
//     pub z_order: String, // ZOrder
// }

#[derive(Serialize, Deserialize)]
pub struct Graph {
    data: HashMap<String, String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct NodeStruct(pub Node);

// #[derive(Serialize, Deserialize)]
// pub struct Graph<'a>{
//     pub phantom: PhantomData<&'a ()>,
//     pub verts: Verts,
//     pub vi_map: VIMap<'a>,
//     pub adj: Adjacency,
//     pub edge_adj: EdgeAdjacency,
//     pub z_adj: Adjacency,
//     pub z_order: ZOrder,
// }
