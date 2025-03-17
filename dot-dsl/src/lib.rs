pub mod graph {
    use std::collections::HashMap;
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                from: &'static str,
                to: &'static str,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &'static str, to: &'static str) -> Self {
                    Self {
                        from,
                        to,
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs.iter()
                        .map(|pair| (String::from(pair.0), String::from(pair.1)))
                        .collect();
                    self
                }

                pub fn attr(&self, name: &str) -> Option<&str> {
                    let Some(value) = self.attrs.get(name) else { return None };
                    Some(value)
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                pub name: &'static str,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &'static str) -> Self {
                    Self {
                        name,
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs.iter()
                        .map(|pair| (String::from(pair.0), String::from(pair.1)))
                        .collect();
                    self
                }

                pub fn attr(&self, name: &str) -> Option<&str> {
                    let Some(value) = self.attrs.get(name) else { return None };
                    Some(value)
                }
            }
        }
    }

    #[derive(Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs.iter()
                .map(|pair| (String::from(pair.0), String::from(pair.1)))
                .collect();
            self
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.into();
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.into();
            self
        }

        pub fn node(&self, name: &str) -> Option<Node> {
            self.nodes.clone().into_iter()
                .find(|node| node.name == name)
        }
    }
}