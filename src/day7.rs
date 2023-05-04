/* 
 * I've learnt that rust isn't ideal for this problem, so skipping it for now.
 * ( parent pointer struggle )
 * */
enum ParentOrRoot {
    Parent(TreeNode),
    Root
}


struct TreeNode { 
    children : Vec<TreeNode>,
    size : u32,
    name : String,
    parent : Box<ParentOrRoot>
}

fn get_parent(node : &TreeNode) -> TreeNode {
    match *node.parent {
        ParentOrRoot :: Parent(parent_node) => parent_node,
        ParentOrRoot :: Root => *node.parent
    }
}

fn make_child(node : TreeNode, location : &str) ->  TreeNode {
    let new_node = TreeNode {
        size : 0,
        name : String::from(location),
        children : Vec :: new(),
        parent : Box :: new(ParentOrRoot :: Root)
    };
    return new_node;
}

pub fn day7(){
    let input = include_str!("./day3.txt").split('\n');
    let mut my_node = TreeNode { size : 0, name : String::from("/"), children : Vec :: new(), parent : Box :: new(ParentOrRoot :: Root) };

    for elem in input {
        if elem.starts_with("$") {
            let mut split_elem = elem.split(' ');
            let command = split_elem.nth(1).unwrap();
            if command == "cd" {
                let location = split_elem.nth(2).unwrap();
                if location == ".." {
                    let parent = get_parent(&my_node);
                }
                else {
                    let new_node = TreeNode {
                        size : 0,
                        name : String::from(location),
                        children : Vec :: new(),
                        parent : Box :: new(ParentOrRoot::Parent(my_node))
                    };
                    // let new_node = make_child(my_node,location);

                }


            }
            else { // ls

            }
            // cmd -> cd or ls. 
        }
    }

}

