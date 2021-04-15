pub mod graph {
    pub mod graph_items {
        pub mod node {
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Node {
                pub data: String,
                pub attrs: std::collections::HashMap<String, String>,
            }

            impl Node {
                pub fn new(data: &str) -> Self {
                    Self {
                        data: data.to_owned(),
                        attrs: Default::default(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|&(key, value)| (key.to_owned(), value.to_owned()))
                        .collect();
                    self
                }

                pub fn get_attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(AsRef::as_ref)
                }
            }
        }

        pub mod edge {
            use super::node::Node;

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Edge {
                u: Node,
                v: Node,
                attrs: std::collections::HashMap<String, String>,
            }

            impl Edge {
                pub fn new(u: &str, v: &str) -> Self {
                    Self {
                        u: Node::new(u),
                        v: Node::new(v),
                        attrs: Default::default(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|&(key, value)| (key.to_owned(), value.to_owned()))
                        .collect();
                    self
                }
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: std::collections::HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Default::default(),
                edges: Default::default(),
                attrs: Default::default(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[graph_items::node::Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &[graph_items::edge::Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|&(key, value)| (key.to_owned(), value.to_owned()))
                .collect();
            self
        }

        pub fn get_node(&self, key: &str) -> Option<&graph_items::node::Node> {
            self.nodes.iter().find(|&node| node.data == key)
        }
    }
}
