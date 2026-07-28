use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Bundle, Debug)]
pub struct CloudPart {
    collider: Collider,
    mesh: Mesh2d,
    mesh_material: MeshMaterial2d<ColorMaterial>,
}

impl CloudPart {
    pub fn build(
        shape: Circle,
        mut meshes: ResMut<Assets<Mesh>>,
        material: Handle<ColorMaterial>,
    ) -> impl Bundle {
        Self {
            collider: Collider::from(shape),
            mesh: Mesh2d(meshes.add(shape)),
            mesh_material: MeshMaterial2d(material),
        }
    }
}
