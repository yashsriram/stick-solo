use crate::sense::pos_conf_space::PosConfSpace;
use ggez::{
    graphics,
    graphics::{DrawMode, DrawParam, MeshBuilder, Rect},
    Context, GameResult,
};
use nalgebra::Point2;

struct PRMVertexSearchState {
    dist_from_start: f32,
    dist_to_finish: f32,
    is_explored: bool,
    parent_in_path_from_start: Option<usize>,
    color: [f32; 4],
}

impl PRMVertexSearchState {
    fn reset() -> Self {
        Self {
            dist_from_start: 0.0,
            dist_to_finish: 0.0,
            is_explored: false,
            parent_in_path_from_start: None,
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    // void addToFringeFrom(PRMVertex parent) {
    //     isExplored = true;
    //     pathFromStart.addAll(parent.searchState.pathFromStart);
    //     pathFromStart.add(PRMVertex.this);
    //     color.headSet(0, 1, 0);
    // }

    // void setExplored() {
    //     color.headSet(1, 0, 0);
    // }
}

struct PRMVertex {
    pos: Point2<f32>,
    neighbours: Vec<usize>,
    search_state: PRMVertexSearchState,
}

impl PRMVertex {
    fn new(pos: Point2<f32>) -> Self {
        Self {
            pos: pos,
            neighbours: Vec::new(),
            search_state: PRMVertexSearchState::reset(),
        }
    }
}

pub struct PRM {
    sampling_area: Rect,
    vertices: Vec<PRMVertex>,
    num_edges: usize,
}

impl PRM {
    const DRAW_VERTICES: bool = true;
    const DRAW_EDGES: bool = true;

    pub fn new(
        sampling_area: Rect,
        try_num_vertices: usize,
        max_edge_len: f32,
        pos_conf_space: &PosConfSpace,
        margin: f32,
    ) -> Self {
        use rand::distributions::Standard;
        use rand::Rng;
        let Rect { x, y, w, h } = sampling_area;
        let isolated_vertcies: Vec<PRMVertex> = rand::thread_rng()
            .sample_iter(Standard)
            .map(|(x01, y01): (f32, f32)| (x + x01 * w, y + y01 * h))
            .filter(|&(x, y)| !pos_conf_space.does_intersect_with_point(Point2::new(x, y), margin))
            .map(|(x, y)| PRMVertex::new(Point2::new(x, y)))
            .take(try_num_vertices)
            .collect();
        let mut vertices = isolated_vertcies;
        let mut num_edges = 0usize;
        let vertices_len = vertices.len();
        for i in 0..vertices_len - 1 {
            for j in i + 1..vertices_len {
                let dist = (vertices[i].pos - vertices[j].pos).norm();
                if dist > max_edge_len {
                    continue;
                }
                if pos_conf_space.does_intersect_with_line_segment(vertices[i].pos, vertices[j].pos)
                {
                    continue;
                }
                vertices[i].neighbours.push(j);
                vertices[j].neighbours.push(i);
                num_edges += 1;
            }
        }
        println!("num edges = {:?}", num_edges);
        Self {
            sampling_area,
            vertices: vertices,
            num_edges: num_edges,
        }
    }
}

impl crate::Draw for PRM {
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let mesh = MeshBuilder::new()
            .rectangle(
                DrawMode::stroke(1.0),
                self.sampling_area,
                [0.0, 0.0, 1.0, 1.0].into(),
            )?
            .build(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::default())?;
        if PRM::DRAW_VERTICES {
            let mut mb = MeshBuilder::new();
            for vert in self.vertices.iter() {
                mb.rectangle(
                    DrawMode::fill(),
                    Rect::new(vert.pos[0], vert.pos[1], 2.0, 2.0),
                    vert.search_state.color.into(),
                )?;
            }
            mb.build(ctx)?;
            graphics::draw(ctx, &mesh, DrawParam::default())?;
        }
        if PRM::DRAW_EDGES {
            let mut mb = MeshBuilder::new();
            for vert in self.vertices.iter() {
                for neigh in vert.neighbours.iter().map(|&idx| &self.vertices[idx]) {
                    mb.line(&[vert.pos, neigh.pos], 1.0, [1.0, 1.0, 1.0, 1.0].into())?;
                }
            }
            let mesh = mb.build(ctx)?;
            graphics::draw(ctx, &mesh, DrawParam::default())?;
        }
        Ok(())
    }
}

// public class PRM {

//
//
//
//     private void resetSearchState(final Vec goalPosition) {
//         PApplet.println("Resetting search states of vertices");
//         for (Milestone v : milestones) {
//             v.searchState.reset(goalPosition);
//         }
//     }

//     private void addToFringe(final Stack<Milestone> fringe, final Milestone current, final Milestone next) {
//         fringe.add(next);
//         next.searchState.addToFringeFrom(current);
//     }

//     public List<Milestone> dfs(final Vec startPosition, final Vec goalPosition, float minEdgeLen, float maxEdgeLen, PositionConfigurationSpace cs) {
//         PApplet.println("DFS");

//         Milestone start = addMilestone(startPosition.get(0), startPosition.get(1), minEdgeLen, maxEdgeLen, cs);
//         Milestone goal = addMilestone(goalPosition.get(0), goalPosition.get(1), minEdgeLen, maxEdgeLen, cs);
//         resetSearchState(goalPosition);

//         int numVerticesExplored = 0;
//         final Stack<Milestone> fringe = new Stack<>();
//         addToFringe(fringe, start, start);
//         while (fringe.size() > 0) {
//             // Pop one vertex
//             Milestone current = fringe.pop();
//             numVerticesExplored++;
//             // Check if goal milestone
//             if (current.id == goal.id) {
//                 PApplet.println("Reached goal, # vertices explored: " + numVerticesExplored);
//                 return goal.searchState.pathFromStart;
//             }
//             // Mark this vertex as explored
//             current.searchState.setExplored();
//             // Update fringe
//             for (Milestone neighbour : current.neighbours) {
//                 if (!neighbour.searchState.isExplored) {
//                     addToFringe(fringe, current, neighbour);
//                 }
//             }
//         }

//         PApplet.println("Could not reach goal milestone, # vertices explored: " + numVerticesExplored);
//         return Collections.singletonList(start);
//     }

//     private void addToFringe(final Queue<Milestone> fringe, final Milestone current, final Milestone next) {
//         next.searchState.distanceFromStart = current.searchState.distanceFromStart + next.position.minus(current.position).norm();
//         fringe.add(next);
//         next.searchState.addToFringeFrom(current);
//     }

//     private List<Milestone> search(final Queue<Milestone> fringe, final Vec startPosition, final Vec goalPosition, float minEdgeLen, float maxEdgeLen, PositionConfigurationSpace cs) {
//         Milestone start = addMilestone(startPosition.get(0), startPosition.get(1), minEdgeLen, maxEdgeLen, cs);
//         Milestone goal = addMilestone(goalPosition.get(0), goalPosition.get(1), minEdgeLen, maxEdgeLen, cs);
//         resetSearchState(goalPosition);

//         int numVerticesExplored = 0;
//         addToFringe(fringe, start, start);
//         while (fringe.size() > 0) {
//             // Pop one vertex
//             Milestone current = fringe.remove();
//             numVerticesExplored++;
//             // Check if goal
//             if (current.id == goal.id) {
//                 PApplet.println("Reached goal, # vertices explored: " + numVerticesExplored);
//                 return goal.searchState.pathFromStart;
//             }
//             // Mark this vertex as explored
//             current.searchState.setExplored();
//             // Update fringe
//             for (Milestone neighbour : current.neighbours) {
//                 if (!neighbour.searchState.isExplored) {
//                     addToFringe(fringe, current, neighbour);
//                 }
//             }
//         }

//         PApplet.println("Could not reach goal, # vertices explored: " + numVerticesExplored);
//         return Collections.singletonList(start);
//     }

//     public List<Milestone> bfs(Vec startPosition, Vec goalPosition, float minEdgeLen, float maxEdgeLen, PositionConfigurationSpace cs) {
//         PApplet.println("BFS");
//         return search(new LinkedList<>(), startPosition, goalPosition, minEdgeLen, maxEdgeLen, cs);
//     }

//     public List<Milestone> ucs(Vec startPosition, Vec goalPosition, float minEdgeLen, float maxEdgeLen, PositionConfigurationSpace cs) {
//         PApplet.println("UCS");
//         return search(new PriorityQueue<>((v1, v2) ->
//                         (int) (v1.searchState.distanceFromStart - v2.searchState.distanceFromStart)),
//                 startPosition, goalPosition, minEdgeLen, maxEdgeLen, cs);
//     }

//     public List<Milestone> aStar(Vec startPosition, Vec goalPosition, float minEdgeLen, float maxEdgeLen, PositionConfigurationSpace cs) {
//         PApplet.println("A*");
//         return search(new PriorityQueue<>((v1, v2) -> (int) (
//                         (v1.searchState.distanceFromStart + v1.searchState.heuristicDistanceToGoal)
//                                 - (v2.searchState.distanceFromStart + v2.searchState.heuristicDistanceToGoal))),
//                 startPosition, goalPosition, minEdgeLen, maxEdgeLen, cs);
//     }

//     public List<Milestone> weightedAStar(Vec startPosition, Vec goalPosition, float minEdgeLen, float maxEdgeLen, PositionConfigurationSpace cs, final float epislon) {
//         PApplet.println("Weighted A* with epsilon = " + epislon);
//         return search(new PriorityQueue<>((v1, v2) -> (int) (
//                         (v1.searchState.distanceFromStart + epislon * v1.searchState.heuristicDistanceToGoal)
//                                 - (v2.searchState.distanceFromStart + epislon * v2.searchState.heuristicDistanceToGoal))),
//                 startPosition, goalPosition, minEdgeLen, maxEdgeLen, cs);
//     }
// 	public void removeMilestones(List<Milestone> milestones) {
// 		for(Milestone milestone: milestones) {
// 			for(Milestone neighbor: milestone.neighbours) {
// 	    		neighbor.neighbours.remove(milestone);
// 	    	}
// 	    	this.milestones.remove(milestone);
// 		}
// 	}
// }
