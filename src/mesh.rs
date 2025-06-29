use bevy::prelude::*;

#[derive(Resource)]
pub struct SphereMesh {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<u32>
}

impl SphereMesh {
    pub fn new(resolution: usize) -> Self {
        let resolution = resolution.max(0);
        let divisions = resolution;
        let vertices_per_face = ((divisions + 3) * (divisions + 3) - (divisions + 3)) / 2;
        let num_vertices = vertices_per_face * 8 - (divisions + 2) * 12 + 6;
        let triplets_per_face = (divisions + 1) * (divisions + 1);

        let mut vertices: Vec<Vec3> = Vec::with_capacity(num_vertices);
        let mut indices: Vec<u32> = Vec::with_capacity(triplets_per_face * 24);

        let base_vertices = [
            Vec3::Y,    // up
            -Vec3::X,   // left
            -Vec3::Z,   // back
            Vec3::X,    // right
            Vec3::Z,    // forward
            -Vec3::Y    // down
        ];

        vertices.extend_from_slice(&base_vertices);

        let vertex_pairs = [
            0, 1, 0, 2, 0, 3, 0, 4,
            1, 2, 2, 3, 3, 4, 4, 1,
            5, 1, 5, 2, 5, 3, 5, 4,
        ];

        let edge_triplets = [
            0, 1, 4, 1, 2, 5, 2, 3, 6, 3, 0, 7,
            8, 9, 4, 9, 10, 5, 10, 11, 6, 11, 8, 7
        ];

        let mut edges = vec![vec![0; divisions + 2]; 12];
        
        for i in (0..vertex_pairs.len()).step_by(2) {
            let start_vertex = vertices[vertex_pairs[i]];
            let end_vertex = vertices[vertex_pairs[i+1]];

            let mut edge_vertex_indices = vec![0; divisions + 2];
            edge_vertex_indices[0] = vertex_pairs[i] as u32;

            for division_index in 0..divisions {
                let t = (division_index as f32 + 1.0) / (divisions as f32 + 1.0);
                edge_vertex_indices[division_index + 1] = vertices.len() as u32;
                
                let interpolated = Dir3::new(start_vertex).unwrap().slerp(Dir3::new(end_vertex).unwrap(), t);
                vertices.push(interpolated.as_vec3());
            }
            edge_vertex_indices[divisions + 1] = vertex_pairs[i + 1] as u32;
            let edge_index = i / 2;
            edges[edge_index] = edge_vertex_indices;
        }

        for i in (0..edge_triplets.len()).step_by(3) {
            let reverse = (i / 3) >= 4;
            
            Self::create_face(
                &mut vertices,
                &mut indices,
                &edges[edge_triplets[i]],
                &edges[edge_triplets[i+1]],
                &edges[edge_triplets[i+2]],
                divisions,
                reverse
            );
        }

        SphereMesh { vertices,indices }
    }

    fn create_face(
        vertices: &mut Vec<Vec3>,
        indices: &mut Vec<u32>,
        side_a: &[u32],
        side_b: &[u32],
        bottom: &[u32],
        divisions: usize,
        reverse: bool 
    ) {
        let points_in_edge = side_a.len();
        let mut vertex_map: Vec<u32> = Vec::with_capacity(points_in_edge * points_in_edge);
        vertex_map.push(side_a[0]);

        for i in 1..points_in_edge - 1 {
            vertex_map.push(side_a[1]);

            let side_a_vertex = vertices[side_a[i] as usize];
            let side_b_vertex = vertices[side_b[i] as usize];
            let inner_points = i - 1;

            for j in 0..inner_points {
                let t = (j as f32 + 1.0) / (inner_points as f32 + 1.0);
                vertex_map.push(vertices.len() as u32);

                let interpolated = Dir3::new(side_a_vertex).unwrap().slerp(Dir3::new(side_b_vertex).unwrap(), t);
                vertices.push(interpolated.as_vec3());
            }

            vertex_map.push(side_b[i]);
        }

        for i in 0..points_in_edge {
            vertex_map.push(bottom[i]);
        }

        for row in 0..=divisions {
            let mut top_vertex = ((row + 1) * (row + 1) - (row + 1)) / 2;
            let mut bottom_vertex = ((row + 2) * (row + 2) - (row + 2)) / 2;

            let triangles_in_row = 1 + 2* row;
            
            for column in 0..triangles_in_row {
                let (v0, v1, v2) = if column % 2 == 0 {
                    let result = (top_vertex, bottom_vertex + 1, bottom_vertex);
                    top_vertex += 1;
                    bottom_vertex += 1;
                    result
                } else {
                    (top_vertex, bottom_vertex, top_vertex - 1)
                };

                indices.push(vertex_map[v0]);
                indices.push(vertex_map[if reverse { v2 } else { v1 }]);
                indices.push(vertex_map[if reverse { v1 } else { v2 }]);
            }
        }
    }
}

