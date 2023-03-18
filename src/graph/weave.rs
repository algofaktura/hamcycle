use itertools::Itertools;
use ndarray;
use rayon::prelude::*;
use std::cell::RefCell;

use super::{
    structs::Cycle,
    types::{
        Adjacency, Bobbins, Count, EdgeAdjacency, Idx, Loom, Node, Point, Solution, Spool,
        Subtours, Tour, TourSlice, VIMap, Vert, Verts, WarpedLoom, Warps, Woven, Yarn, YarnEnds,
        ZOrder,
    },
    utils::xy::{absumv, axis},
};

pub fn weave(
    adj: &Adjacency,
    vi_map: &VIMap,
    edge_adj: &EdgeAdjacency,
    verts: &Verts,
    z_adj: &Adjacency,
    z_order: &ZOrder,
    max_xyz: Point,
) -> Solution {
    join_loops(
        prepare_loom(vi_map, verts, z_adj, z_order),
        adj,
        verts,
        edge_adj,
        max_xyz,
    )
}

fn prepare_loom(vi_map: &VIMap, verts: &Verts, z_adj: &Adjacency, z_order: &ZOrder) -> Loom {
    let spool: Spool = spin_and_color_yarn(z_adj, verts);
    let mut bobbins: Bobbins = Vec::new();
    let mut loom: Loom = Loom::new();
    for (zlevel, order) in z_order {
        let warps: Warps = get_warps(*zlevel, *order, &bobbins, &spool, vi_map);
        let woven: Woven = join_threads(&mut loom, &warps);
        affix_loose_threads(&mut loom, warps, woven);
        if *zlevel != -1 {
            bobbins = wind(&mut loom, verts, vi_map);
        }
    }
    reflect_loom(&mut loom, verts, vi_map);
    loom
}

fn spin_and_color_yarn(z_adj: &Adjacency, verts: &Verts) -> Spool {
    let natural: Yarn = spin(z_adj, verts);
    let colored: Yarn = color_yarn(&natural);
    Spool::from([(3, natural), (1, colored)])
}

fn spin(z_adj: &Adjacency, verts: &Verts) -> Yarn {
    let path: &mut Tour = &mut vec![*z_adj.keys().max().unwrap() as Node];
    let order: Count = z_adj.len();
    (1..order).for_each(|idx| path.push(next_node(path, z_adj, verts, idx, order)));
    convert_nodes_to_yarn(path, verts)
}

fn next_node(path: TourSlice, adj: &Adjacency, verts: &Verts, idx: usize, order: usize) -> Node {
    if idx < order - 5 {
        adj[path.last().unwrap()]
            .iter()
            .filter(|n| !path.contains(*n))
            .copied()
            .max_by_key(|&n| absumv(verts[n as usize]))
            .unwrap()
    } else {
        let curr: &Node = &path[path.len() - 1];
        let curr_vert: &Vert = &verts[*curr as usize];
        adj[curr]
            .iter()
            .filter(|n| !path.contains(*n))
            .map(|&n| (n, axis(curr_vert, &verts[n as usize])))
            .filter(|(_, next_axis)| {
                *next_axis != axis(&verts[path[path.len() - 2] as usize], curr_vert)
            })
            .max_by_key(|&(n, _)| absumv(verts[n as usize]))
            .unwrap()
            .0
    }
}

fn convert_nodes_to_yarn(path: &mut Tour, verts: &Verts) -> Yarn {
    Yarn::from(
        path.iter()
            .map(|&n| [verts[n as usize].0, verts[n as usize].1])
            .collect::<Vec<[Point; 2]>>(),
    )
}

fn color_yarn(a: &Yarn) -> Yarn {
    a.clone().dot(&ndarray::arr2(&[[-1, 0], [0, -1]])) + ndarray::arr2(&[[0, 2]])
}

fn wind(loom: &mut Loom, verts: &Verts, vi_map: &VIMap) -> Bobbins {
    loom.iter_mut()
        .flat_map(|thread| {
            let (left, right) = get_upper_nodes(
                verts[thread[0] as usize],
                verts[thread[thread.len() - 1] as usize],
                vi_map,
            );
            thread.push_front(left);
            thread.push_back(right);
            vec![left, right]
        })
        .collect()
}

fn get_upper_nodes((x, y, z): Vert, (x1, y1, z1): Vert, vi_map: &VIMap) -> (u32, u32) {
    (vi_map[&(x, y, z + 2)], vi_map[&(x1, y1, z1 + 2)])
}

fn get_warps(
    zlevel: Point,
    order: Count,
    bobbins: &Bobbins,
    spool: &Spool,
    vi_map: &VIMap,
) -> Warps {
    let node_yarn: Tour = preallocate_node_yarn(
        spool[&(zlevel % 4 + 4).try_into().unwrap()].clone(),
        zlevel,
        order,
        vi_map,
    );
    if bobbins.is_empty() {
        vec![node_yarn]
    } else {
        cut(node_yarn, bobbins)
    }
}

fn preallocate_node_yarn(mut yarn: Yarn, zlevel: Point, order: Count, vi_map: &VIMap) -> Tour {
    yarn.slice_axis_inplace(
        ndarray::Axis(0),
        ndarray::Slice::new(
            (yarn.len_of(ndarray::Axis(0)) - order).try_into().unwrap(),
            None,
            1,
        ),
    );
    yarn.outer_iter()
        .map(|row| vi_map[&(row[0], row[1], zlevel)])
        .collect()
}

fn cut(tour: Tour, subset: &Bobbins) -> Subtours {
    let mut subtours: Subtours = Vec::new();
    let last_ix: Idx = tour.len() - 1;
    let last_idx: Idx = subset.len() - 1;
    let mut prev: i32 = -1_i32;
    for (e, idx) in tour
        .iter()
        .enumerate()
        .filter_map(|(i, &node)| {
            if subset.contains(&node) {
                Some(i)
            } else {
                None
            }
        })
        .sorted()
        .enumerate()
    {
        if e == last_idx && idx != last_ix {
            for subtour in vec![tour[prev as usize + 1..idx].to_vec(), tour[idx..].to_vec()] {
                if !subtour.is_empty() {
                    subtours.push(if subset.contains(&subtour[0]) {
                        subtour
                    } else {
                        subtour.iter().rev().cloned().collect()
                    });
                }
            }
        } else {
            let subtour = tour[prev as usize + 1..=idx].to_vec();
            if !subtour.is_empty() {
                subtours.push(if subset.contains(&subtour[0]) {
                    subtour
                } else {
                    subtour.iter().rev().cloned().collect()
                });
            }
            prev = idx as i32;
        }
    }
    subtours
}

fn join_threads(loom: &mut Loom, warps: &Warps) -> Woven {
    let mut woven: Woven = Woven::new();
    for thread in loom {
        for (idx, warp) in warps.iter().enumerate() {
            if !woven.contains(&idx) {
                match (thread.front(), thread.back()) {
                    (Some(front), _) if *front == warp[0] => {
                        *thread = warp[1..]
                            .iter()
                            .rev()
                            .chain(thread.iter())
                            .cloned()
                            .collect()
                    }
                    (_, Some(back)) if *back == warp[0] => thread.extend(warp.iter().skip(1)),
                    _ => continue,
                }
                woven.extend([idx])
            }
        }
    }
    woven
}

fn affix_loose_threads(loom: &mut Loom, warps: Warps, woven: Woven) {
    for (_, seq) in warps
        .iter()
        .enumerate()
        .filter(|(idx, _)| !woven.contains(idx))
    {
        loom.extend(vec![seq.iter().cloned().collect::<YarnEnds>()])
    }
}

fn reflect_loom(loom: &mut Loom, verts: &Verts, vi_map: &VIMap) {
    loom.par_iter_mut().for_each(|thread| {
        thread.extend(
            thread
                .iter()
                .rev()
                .map(|&node| verts[node as usize])
                .map(|(x, y, z)| vi_map[&(x, y, -z)])
                .collect::<Tour>(),
        )
    });
}

pub fn join_loops<'a>(
    mut warp_wefts: Loom,
    adj: &'a Adjacency,
    verts: &'a Verts,
    edge_adj: &'a EdgeAdjacency,
    max_xyz: Point,
) -> Solution {
    let mut core_cord: Cycle = Cycle::new(
        warp_wefts[0].split_off(0),
        adj,
        edge_adj,
        verts,
        true,
        max_xyz,
    );
    let loom: WarpedLoom = warp_wefts
        .split_off(1)
        .into_iter()
        .enumerate()
        .map(|(idx, seq)| {
            (
                idx,
                RefCell::new(Cycle::new(seq, adj, edge_adj, verts, false, max_xyz)),
            )
        })
        .collect();
    for key in loom.keys() {
        let other = &mut loom[key].borrow_mut();
        let other_edges = other.make_edges();
        if let Some(warp_e) = (&core_cord.make_edges() & &other.make_eadjs(&other_edges))
            .into_iter()
            .next()
        {
            if let Some(weft_e) = (&edge_adj[(&warp_e)] & &other_edges).into_iter().next() {
                core_cord.join(warp_e, weft_e, other);
            }
        }
    }
    core_cord.retrieve_nodes()
}
