#[derive(Clone)]
pub struct TreeNode {
    pub id: String,
    pub children: Vec<TreeNode>,
}

pub fn calculate_triangular_number(num: i64) -> i64 {
    if num > 0 {
        num + calculate_triangular_number(num - 1)
    } else {
        0
    }
}

impl TreeNode {
    pub fn append_node(&mut self, node_to_find: &str, node_to_add: &TreeNode) {
        //println!("{}", node_to_find);
        if node_to_find == self.id {
            self.children.push(node_to_add.clone());
            return;
        }

        for i in &mut self.children {
            i.append_node(node_to_find, node_to_add);
        }
    }

    pub fn find_node_mut(&mut self, node_to_find: &str) -> Option<&mut TreeNode> {
        if node_to_find == self.id {
            return Some(self);
        }

        for i in &mut self.children {
            let node = i.find_node_mut(node_to_find);
            if node.is_some() {
                return node;
            }
        }

        None
    }

    pub fn find_node(&self, node_to_find: &str) -> Option<&TreeNode> {
        if node_to_find == self.id {
            return Some(self);
        }

        for i in &self.children {
            let node = i.find_node(node_to_find);
            if node.is_some() {
                return node;
            }
        }

        None
    }

    pub fn count_orbits(node: TreeNode, distance_from_root: i64) -> i64 {
        let mut val_to_return = 0;

        for i in node.children {
            val_to_return += TreeNode::count_orbits(i, distance_from_root + 1);
        }

        val_to_return + distance_from_root
    }
}

pub fn part_one() {
    let contents: String = common::read_file_to_string("2019/six_input.txt");

    let mut orbits_str: Vec<&str> = contents.split('\n').collect();

    let mut root: TreeNode = TreeNode {
        id: "Incorrect".to_string(),
        children: vec![],
    };

    let mut idx: usize = 0;

    let mut all_orbits: Vec<TreeNode> = vec![];
    // build ALL TreeNodes first
    for i in &orbits_str {
        let (parent, child) = i.split_at(3);
        let parent_node = TreeNode {
            id: parent.to_string(),
            children: vec![],
        };

        let child_node = TreeNode {
            id: child.get(1..).unwrap().trim().to_string(),
            children: vec![],
        };

        let mut push_parent = true;
        let mut push_child = true;
        for j in &orbits_str {
            if !push_parent && !push_child {
                break;
            }
            if *j == parent_node.id {
                push_parent = false;
                continue;
            }
            if *j == child_node.id {
                push_child = false;
                continue;
            }
        }
        if push_parent {
            all_orbits.push(parent_node);
        }
        if push_child {
            all_orbits.push(child_node);
        }
    }

    // for i in &all_orbits {
    //     println!("{}", i.id);
    // }
    /*
    while idx < all_orbits.len() {
        let parent_id = &all_orbits[idx].id;
        let parent_id_copy = all_orbits[idx].id.clone();
        let mut child_id : String = "".to_string();
        for i in &orbits_str {
            let (parent, child) = i.split_at(3);
            if parent == parent_id {
                child_id = child.get(1..).unwrap().trim().to_string();
            }
        }

        let mut jdx : usize = 0;
        while jdx < all_orbits.len() {
            if all_orbits[jdx].id == child_id {
                break;
            }
            jdx += 1;
            if jdx == all_orbits.len() {
                let x = 5;
            }
        }
        let node : TreeNode = all_orbits[jdx].clone();
        all_orbits[idx].append_node(parent_id_copy.as_str(), &node);
        all_orbits.remove(jdx);
        if jdx >= idx {
            idx += 1;
        }
    }*/
    let mut indices_completed: Vec<usize> = vec![];
    while all_orbits.len() > 1 {
        let mut index: usize = 0;
        for i in &mut orbits_str {
            if indices_completed.contains(&index) {
                continue;
            }

            let (parent, child) = i.split_at(3);

            //println!("{}", child.get(1..).unwrap().trim());

            let mut child_node: TreeNode = TreeNode {
                id: "Incorrect".to_string(),
                children: vec![],
            };

            for j in 0..all_orbits.len() {
                let found_node = all_orbits[j].find_node(child.get(1..).unwrap().trim());
                if found_node.is_some() {
                    child_node = found_node.unwrap().clone();
                    all_orbits.remove(j);
                    break;
                }
            }

            let mut parent_node: Option<&mut TreeNode> = None;
            for j in 0..all_orbits.len() {
                parent_node = all_orbits[j].find_node_mut(parent);
                if parent_node.is_some() {
                    println!("{}", j);
                    break;
                }
            }

            if parent_node.is_none() {
                continue;
            }

            //let parent_node_unwrapped = child_node.clone();
            parent_node.unwrap().append_node(parent, &child_node);

            //all_orbits.push(parent_node);
            /*let child_node = TreeNode {
                id: child.get(1..).unwrap().trim().to_string(),
                children: vec![],
            };*/

            //root.append_node(parent, &child_node);
            //idx += 1;

            indices_completed.push(index);
            index += 1;
        }
    }

    let orbit_count: i64 = TreeNode::count_orbits(root, 0);

    println!("{}", orbit_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doubles() {
        let root = TreeNode {
            id: "COM".to_string(),
            children: vec![TreeNode {
                id: "B".to_string(),
                children: vec![
                    TreeNode {
                        id: "C".to_string(),
                        children: vec![TreeNode {
                            id: "D".to_string(),
                            children: vec![
                                TreeNode {
                                    id: "E".to_string(),
                                    children: vec![
                                        TreeNode {
                                            id: "J".to_string(),
                                            children: vec![TreeNode {
                                                id: "K".to_string(),
                                                children: vec![TreeNode {
                                                    id: "L".to_string(),
                                                    children: vec![],
                                                }],
                                            }],
                                        },
                                        TreeNode {
                                            id: "F".to_string(),
                                            children: vec![],
                                        },
                                    ],
                                },
                                TreeNode {
                                    id: "I".to_string(),
                                    children: vec![],
                                },
                            ],
                        }],
                    },
                    TreeNode {
                        id: "G".to_string(),
                        children: vec![TreeNode {
                            id: "H".to_string(),
                            children: vec![],
                        }],
                    },
                ],
            }],
        };

        assert_eq!(TreeNode::count_orbits(root, 0), 42);
    }
}
