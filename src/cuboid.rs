//! A module for constructing cuboid shapes.

extern crate cgmath;
extern crate glium;

use errors::ShapeCreationError;
use self::cgmath::*;
use vertex::Vertex;

/// A polygonal `Cuboid` object.
///
/// This object is constructed using a `CuboidBuilder` object.
pub struct Cuboid {
    vertices: glium::vertex::VertexBufferAny,
}

/// Allows a `Cuboid` object to be passed as a source of vertices.
impl<'a> glium::vertex::IntoVerticesSource<'a> for &'a Cuboid {
    fn into_vertices_source(self) -> glium::vertex::VerticesSource<'a> {
        return self.vertices.into_vertices_source();
    }
}

/// Allows a `Cuboid` object to be passed as a source of indices.
impl<'a> Into<glium::index::IndicesSource<'a>> for &'a Cuboid {
    fn into(self) -> glium::index::IndicesSource<'a> {
        return glium::index::IndicesSource::NoIndices {
            primitives: glium::index::PrimitiveType::TrianglesList,
        };
    }
}

/// Responsible for building and returning a `Cuboid` object.
///
/// By default, the cuboid is defined as a unit-cube with its centre-of-mass
/// located at the origin. This can be overriden using the transformation
/// methods on this object.
///
/// The resultant geometry is constructed to suit OpenGL defaults - assuming
/// a right-handed coordinate system, front-facing polygons are defined in
/// counter-clock-wise order. Vertex normals point in the direction of their
/// respective face (such that the shape appears faceted when lit). Vertex
/// texture coordinates define a planar-projection on each face.
pub struct CuboidBuilder {
    matrix: cgmath::Matrix4<f32>,
}

impl Default for CuboidBuilder {
    fn default() -> Self {
        CuboidBuilder { matrix: cgmath::Matrix4::<f32>::identity() }
    }
}

impl CuboidBuilder {
    /// Create a new `CuboidBuilder` object.
    pub fn new() -> CuboidBuilder {
        Default::default()
    }

    /// Apply a scaling transformation to the shape.
    ///
    /// The `scale`, `translate`, and `rotate` functions accumulate, and are
    /// not commutative. The transformation functions are intended to provide
    /// flexibility in model-space. For per-instance world-space transformations,
    /// one should prefer to share as few shapes as possible across multiple
    /// instances, and instead rely on uniform constants in the shader and/or
    /// instanced drawing.
    pub fn scale(mut self, x: f32, y: f32, z: f32) -> Self {
        self.matrix = cgmath::Matrix4::from_nonuniform_scale(x, y, z) * self.matrix;
        return self;
    }

    /// Apply a translation transformation to the shape.
    ///
    /// The `scale`, `translate`, and `rotate` functions accumulate, and are
    /// not commutative. The transformation functions are intended to provide
    /// flexibility in model-space. For per-instance world-space transformations,
    /// one should prefer to share as few shapes as possible across multiple
    /// instances, and instead rely on uniform constants in the shader and/or
    /// instanced drawing.
    pub fn translate(mut self, x: f32, y: f32, z: f32) -> Self {
        self.matrix = cgmath::Matrix4::from_translation([x, y, z].into()) * self.matrix;
        return self;
    }

    /// Apply a rotation transformation to the shape about the x-axis.
    ///
    /// The `scale`, `translate`, and `rotate` functions accumulate, and are
    /// not commutative. The transformation functions are intended to provide
    /// flexibility in model-space. For per-instance world-space transformations,
    /// one should prefer to share as few shapes as possible across multiple
    /// instances, and instead rely on uniform constants in the shader and/or
    /// instanced drawing.
    pub fn rotate_x(mut self, radians: f32) -> Self {
        self.matrix = cgmath::Matrix4::<f32>::from(
            cgmath::Matrix3::<f32>::from_angle_x(
                cgmath::Rad::<f32>(radians)
            )
        ) * self.matrix;
        return self;
    }

    /// Apply a rotation transformation to the shape about the y-axis.
    ///
    /// The `scale`, `translate`, and `rotate` functions accumulate, and are
    /// not commutative. The transformation functions are intended to provide
    /// flexibility in model-space. For per-instance world-space transformations,
    /// one should prefer to share as few shapes as possible across multiple
    /// instances, and instead rely on uniform constants in the shader and/or
    /// instanced drawing.
    pub fn rotate_y(mut self, radians: f32) -> Self {
        self.matrix = cgmath::Matrix4::<f32>::from(
            cgmath::Matrix3::<f32>::from_angle_y(
                cgmath::Rad::<f32>(radians)
            )
        ) * self.matrix;
        return self;
    }

    /// Apply a rotation transformation to the shape about the z-axis.
    ///
    /// The `scale`, `translate`, and `rotate` functions accumulate, and are
    /// not commutative. The transformation functions are intended to provide
    /// flexibility in model-space. For per-instance world-space transformations,
    /// one should prefer to share as few shapes as possible across multiple
    /// instances, and instead rely on uniform constants in the shader and/or
    /// instanced drawing.
    pub fn rotate_z(mut self, radians: f32) -> Self {
        self.matrix = cgmath::Matrix4::<f32>::from(
            cgmath::Matrix3::<f32>::from_angle_z(
                cgmath::Rad::<f32>(radians)
            )
        ) * self.matrix;
        return self;
    }

    /// Build a new `Cuboid` object.
    pub fn build<F>(self, display: &F) -> Result<Cuboid, ShapeCreationError>
        where F: glium::backend::Facade
    {
        let vertices = &self.build_vertices()?;
        let vbuffer = glium::vertex::VertexBuffer::<Vertex>::new(display, vertices)?;
        Ok(Cuboid { vertices: glium::vertex::VertexBufferAny::from(vbuffer) })
    }

    /// Build the shape vertices and return them in a vector.
    ///
    /// Useful if you wish to do other things with the vertices besides constructing
    /// a `Cuboid` object (e.g. unit testing, further processing, etc).
    pub fn build_vertices(&self) -> Result<Vec<Vertex>, ShapeCreationError> {

        // Define lookup-tables used during construction of the cuboid geometry
        let index_lut = [
            0, 4, 1, 5, // -X
            6, 2, 7, 3, // +X
            0, 2, 4, 6, // -Y
            5, 7, 1, 3, // +Y
            2, 0, 3, 1, // -Z
            4, 6, 5, 7, // +Z
        ];
        let poly_lut = [0, 1, 2, 2, 1, 3];
        let num_sides = 6;
        let verts_per_side = 6;

        // Compute the normal transformation matrix.
        let normal_matrix = Matrix3::<f32>::from_cols(self.matrix.x.truncate(),
                                                      self.matrix.y.truncate(),
                                                      self.matrix.z.truncate())
            .invert()
            .unwrap_or(Matrix3::<f32>::identity())
            .transpose();

        // Generate cuboid vertices.
        let mut vertices = Vec::<Vertex>::with_capacity(verts_per_side * num_sides);

        for side in 0..num_sides {

            // Compute side normal.
            let mut normal = Vector3::<f32>::new(0.0, 0.0, 0.0);
            normal[side / 2] = (((side % 2) * 2) as f32) - 1.0;

            // Build side vertices.
            for vert in 0..verts_per_side {
                let coord = index_lut[poly_lut[vert] + (side * 4)];
                let vpos = Vector4::<f32>::new((((coord & 2) - 1) as f32) * 0.5,
                                               (((coord & 1) * 2 - 1) as f32) * 0.5,
                                               ((((coord >> 1) & 2) - 1) as f32) * 0.5,
                                               1.0);
                vertices.push(Vertex {
                    position: Point3::<f32>::from_homogeneous(self.matrix * vpos).into(),
                    normal: (normal_matrix * normal).normalize().into(),
                    texcoord: [(poly_lut[vert] % 2) as f32, (poly_lut[vert] / 2) as f32],
                });
            }
        }

        return Ok(vertices);
    }
}

#[test]
pub fn ensure_default_cuboid_has_unit_dimensions() {
    let vertices = CuboidBuilder::new()
        .build_vertices()
        .expect("Failed to build vertices");
    for ref vertex in vertices {
        assert_eq!(vertex.position[0].abs(), 0.5);
        assert_eq!(vertex.position[1].abs(), 0.5);
        assert_eq!(vertex.position[2].abs(), 0.5);
    }
}

#[test]
pub fn ensure_default_cuboid_has_centroid_at_origin() {
    let vertices = CuboidBuilder::new()
        .build_vertices()
        .expect("Failed to build vertices");
    let mut sum = Vector3::<f32>::zero();
    for ref vertex in vertices {
        sum = sum + Vector3::<f32>::from(vertex.position);
    }
    assert_eq!(sum, Vector3::<f32>::zero());
}

#[test]
pub fn ensure_default_cuboid_has_outward_facing_normals() {
    let vertices = CuboidBuilder::new()
        .scale(2.0, 2.0, 2.0)
        .build_vertices()
        .expect("Failed to build vertices");
    for ref vertex in vertices {
        let position = Vector3::<f32>::from(vertex.position);
        let normal = Vector3::<f32>::from(vertex.normal);
        let outside = position + normal;
        assert!(outside.x.abs() >= position.x.abs());
        assert!(outside.y.abs() >= position.y.abs());
        assert!(outside.z.abs() >= position.z.abs());
    }
}

#[test]
pub fn ensure_default_cuboid_has_uvs_in_unit_range() {
    use std::f32;
    let vertices = CuboidBuilder::new()
        .build_vertices()
        .expect("Failed to build vertices");
    let mut min = Vector2::<f32>::new(f32::MAX, f32::MAX);
    let mut max = -min;
    for ref vertex in vertices {
        min.x = f32::min(min.x, vertex.texcoord[0]);
        min.y = f32::min(min.y, vertex.texcoord[1]);
        max.x = f32::max(max.x, vertex.texcoord[0]);
        max.y = f32::max(max.y, vertex.texcoord[1]);
    }
    assert!(min == Vector2::<f32>::zero());
    assert!(max == Vector2::<f32>::from_value(1.0));
}

#[test]
pub fn ensure_default_cuboid_has_ccw_triangles() {
    let vertices = CuboidBuilder::new()
        .build_vertices()
        .expect("Failed to build vertices");
    for chunk in vertices.chunks(3) {
        let v0 = Vector3::<f32>::from(chunk[0].position);
        let v1 = Vector3::<f32>::from(chunk[1].position);
        let v2 = Vector3::<f32>::from(chunk[2].position);
        let eyepos = v0 + Vector3::<f32>::from(chunk[0].normal);
        let e0 = v1 - v0;
        let e1 = v2 - v0;
        let n = e0.cross(e1);
        assert!(n.dot(v0 - eyepos) <= 0.0);
        assert!(n.dot(v1 - eyepos) <= 0.0);
        assert!(n.dot(v2 - eyepos) <= 0.0);
    }
}

#[test]
pub fn ensure_default_cuboid_has_faceted_normals() {
    let vertices = CuboidBuilder::new()
        .build_vertices()
        .expect("Failed to build vertices");
    for chunk in vertices.chunks(3) {
        let v0 = Vector3::<f32>::from(chunk[0].position);
        let v1 = Vector3::<f32>::from(chunk[1].position);
        let v2 = Vector3::<f32>::from(chunk[2].position);
        let n0 = Vector3::<f32>::from(chunk[0].normal);
        let n1 = Vector3::<f32>::from(chunk[1].normal);
        let n2 = Vector3::<f32>::from(chunk[2].normal);
        let e0 = v1 - v0;
        let e1 = v2 - v0;
        let n = e0.cross(e1).normalize();
        assert_ulps_eq!(n, n0);
        assert_ulps_eq!(n, n1);
        assert_ulps_eq!(n, n2);
    }
}
