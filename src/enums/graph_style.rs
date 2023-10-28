#[derive(Debug, PartialEq, Clone)]
pub enum GraphStyle {
    Grid,
    Graph,
}

impl GraphStyle {
    pub fn reverse(self) -> GraphStyle {
        if self == GraphStyle::Grid {
            return GraphStyle::Graph;
        }
        GraphStyle::Grid
    }
}
