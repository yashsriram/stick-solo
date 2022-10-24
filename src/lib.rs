use bevy::{
    prelude::Mesh,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

pub mod act;
pub mod game;
pub mod plan;

pub struct AxesHuggingUnitSquare;

impl From<AxesHuggingUnitSquare> for Mesh {
    fn from(_: AxesHuggingUnitSquare) -> Self {
        let (u_left, u_right) = (0.0, 1.0);
        let vertices = [
            ([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [u_left, 1.0]),
            ([0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [u_left, 0.0]),
            ([1.0, 1.0, 0.0], [0.0, 0.0, 1.0], [u_right, 0.0]),
            ([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [u_right, 1.0]),
        ];
        let indices = Indices::U32(vec![0, 2, 1, 0, 3, 2]);
        let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }
}
