use crate::proto::message::ProtoMessage;
use crate::unity::generated::il2cpp_2022333f1::root::TypeIndex;

use hashbrown::HashMap;
use nohash_hasher::IntSet;
use petgraph::algo::tarjan_scc;
use petgraph::graph::Graph;

#[derive(Clone, PartialEq)]
pub struct ProtoMessageGroup(#[doc(hidden)] pub Vec<ProtoMessage>);

pub type ProtoMessageGroups = Vec<ProtoMessageGroup>;

pub fn messages_to_message_groups(messages: Vec<ProtoMessage>) -> ProtoMessageGroups {
    let mut graph = Graph::<usize, ()>::new();
    let mut node_indices = Vec::new();

    for (i, msg) in messages.iter().enumerate() {
        let idx = graph.add_node(i);
        node_indices.push((msg.type_index, idx));
    }

    let index_map: HashMap<TypeIndex, _> = node_indices.into_iter().collect();
    for node_idx in graph.node_indices() {
        let msg_idx = graph[node_idx];
        let msg = &messages[msg_idx];
        for used in msg.get_used_types() {
            if let Some(&target_idx) = index_map.get(&used) {
                graph.add_edge(node_idx, target_idx, ());
            }
        }
    }

    let sccs = tarjan_scc(&graph);
    let mut messages_map: HashMap<usize, ProtoMessage> =
        messages.into_iter().enumerate().collect::<HashMap<_, _>>();

    let groups = sccs
        .into_iter()
        .map(|component| {
            let group_messages: Vec<ProtoMessage> = component
                .into_iter()
                .map(|node_idx| {
                    let msg_idx = graph[node_idx];
                    messages_map.remove(&msg_idx).expect("Message not found")
                })
                .collect();
            ProtoMessageGroup(group_messages)
        })
        .collect();

    debug_assert!(messages_map.is_empty(), "Some messages were not processed");

    groups
}

impl ProtoMessageGroup {
    pub fn get_used_types(&self) -> IntSet<TypeIndex> {
        let mut used_types = IntSet::default();
        for msg in &self.0 {
            used_types.extend(msg.get_used_types());
        }
        used_types
    }

    pub fn get_contained_types(&self) -> IntSet<TypeIndex> {
        let mut contained_types = IntSet::default();
        for msg in &self.0 {
            contained_types.extend(msg.get_contained_types());
        }
        contained_types
    }

    pub fn get_primary(&self) -> &ProtoMessage {
        if self.0.len() == 1 {
            return &self.0[0];
        }
        let all_contained = self.get_contained_types();
        let counts = self
            .0
            .iter()
            .flat_map(|msg| msg.get_used_types())
            .filter(|ty_idx| all_contained.contains(ty_idx))
            .fold(HashMap::new(), |mut map, ty_idx| {
                *map.entry(ty_idx).or_insert(0) += 1;
                map
            });

        if let Some((best_ty_idx, _)) = counts.into_iter().max_by_key(|&(_, count)| count) {
            self.0
                .iter()
                .find(|msg| msg.type_index == best_ty_idx)
                .unwrap_or_else(|| self.0.first().unwrap())
        } else {
            self.0.first().unwrap()
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &ProtoMessage> {
        self.0.iter()
    }
}
