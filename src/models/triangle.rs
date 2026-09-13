use crate::vertex::Vertex;

pub struct Triangle{
    vertices: [Vertex; 3]
}

impl Triangle{
    pub fn new () -> Self{
        Self{
            vertices: [
                Vertex::new([0.0, 0.5], [1.0, 0.0, 0.0]),
                Vertex::new([0.5, -0.5], [0.0, 1.0, 0.0]),
                Vertex::new([-0.5, -0.5], [0.0, 0.0, 1.0]),
            ]
        }
    }

    pub fn from_vertices (vertices: [Vertex; 3]) -> Self{
        Self{
            vertices
        }
    }
}