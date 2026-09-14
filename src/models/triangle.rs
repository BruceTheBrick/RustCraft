use crate::vertex::Vertex;

pub struct Triangle {
    vertices: [Vertex; 3],
}

impl Triangle {
    pub fn new() -> Self {
        Self {
            vertices: [
                Vertex::new([0.0, 0.5], [1.0, 0.0, 0.0]),
                Vertex::new([-0.5, -0.5], [0.0, 0.0, 1.0]),
                Vertex::new([0.5, -0.5], [0.0, 1.0, 0.0]),
            ],
        }
    }

    /// Creates a new <c>Triangle</c> from a set of 3 Vertex objects
    pub fn from_vertices(vertices: [Vertex; 3]) -> Self {
        Self { vertices }
    }

    pub fn vertices(&self) -> &[Vertex; 3] {
        return &self.vertices;
    }
}
