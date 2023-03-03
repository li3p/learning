#[derive(Debug)]
struct TreeNode {
    level: i32,
    name: String,
}

// struct Traveller<'a> {
//     // iter: dyn Iterator<Item = &'a TreeNode>,
//     iter: std::slice::Iter<'a, TreeNode>,
// }

// impl<'a> Traveller<'a> {
//     fn pretty(&mut self, parent: &TreeNode, current: &TreeNode) -> Option<&'a TreeNode> {
//         assert!(parent.level + 1 == current.level);

//         println!(
//             "++ save current - {} under parent - {}",
//             current.name, parent.name
//         );

//         let mut next: Option<&TreeNode> = None;

//         if let Some(node) = self.iter.next() {
//             if node.level == current.level + 1 {
//                 next = self.pretty(current, node);
//             } else {
//                 next = Some(node);
//             }

//             if next.is_some() && parent.level + 1 == next.unwrap().level {
//                 next = self.pretty(parent, next.unwrap())
//             }
//         }

//         next
//     }
// }

fn pretty_print<'a>(
    iter: &mut std::slice::Iter<'a, TreeNode>,
    parent: &TreeNode,
    current: &TreeNode,
) -> Option<&'a TreeNode> {
    assert!(parent.level + 1 == current.level);

    println!(
        "++ save current - {} under parent - {}",
        current.name, parent.name
    );

    let mut next: Option<&TreeNode> = None;

    if let Some(node) = iter.next() {
        if node.level == current.level + 1 {
            next = pretty_print(iter, current, node);
        } else {
            next = Some(node);
        }

        if next.is_some() && parent.level + 1 == next.unwrap().level {
            next = pretty_print(iter, parent, next.unwrap())
        }
    }

    next
}

fn main() {
    let data = vec![
        TreeNode {
            level: 1,
            name: "Node 01".to_string(),
        },
        TreeNode {
            level: 2,
            name: "Node 02".to_string(),
        },
        TreeNode {
            level: 3,
            name: "Node 03".to_string(),
        },
        TreeNode {
            level: 4,
            name: "Node 04".to_string(),
        },
        TreeNode {
            level: 5,
            name: "Node 05".to_string(),
        },
        TreeNode {
            level: 5,
            name: "Node 06".to_string(),
        },
        TreeNode {
            level: 4,
            name: "Node 07".to_string(),
        },
        TreeNode {
            level: 5,
            name: "Node 08".to_string(),
        },
        TreeNode {
            level: 2,
            name: "Node 09".to_string(),
        },
        TreeNode {
            level: 3,
            name: "Node 10".to_string(),
        },
        TreeNode {
            level: 4,
            name: "Node 11".to_string(),
        },
        TreeNode {
            level: 5,
            name: "Node 12".to_string(),
        },
        TreeNode {
            level: 3,
            name: "Node 13".to_string(),
        },
        TreeNode {
            level: 4,
            name: "Node 14".to_string(),
        },
        TreeNode {
            level: 5,
            name: "Node 15".to_string(),
        },
        TreeNode {
            level: 2,
            name: "Node 16".to_string(),
        },
        TreeNode {
            level: 3,
            name: "Node 17".to_string(),
        },
        TreeNode {
            level: 4,
            name: "Node 18".to_string(),
        },
        TreeNode {
            level: 5,
            name: "Node 19".to_string(),
        },
    ];

    let mut iter = data.iter();
    let parent = iter.next().unwrap();
    let current = iter.next().unwrap();
    pretty_print(&mut iter, parent, current);

    // let mut traveller = Traveller { iter: data.iter() };
    // let parent = traveller.iter.next().unwrap();
    // let current = traveller.iter.next().unwrap();

    // traveller.pretty(parent, current);
}
