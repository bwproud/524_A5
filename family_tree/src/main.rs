use std::io;

struct TreeNode {
    name: String,
    mother: Option<Box<TreeNode>>,
    father: Option<Box<TreeNode>>,
}

fn init_node(name: String) -> TreeNode {
    TreeNode {
        name: name,
        mother: None,
        father: None,
    }
}

fn print_tree(root: &TreeNode, tabs: u32) {
    for _x in 0..tabs {
        print!("{}", "\t".to_string());
    }
    println!("{}",root.name);
    if let Some(ref mother) = root.mother {
        print_tree(mother, tabs+1);
    }

    if let Some(ref father) = root.father {
        print_tree(father, tabs+1);
    }
}

fn add(root: &mut TreeNode, new_name: &String, relation: &String, old_name: &String) -> bool {
    let mut added: bool = false;
    if root.name == *old_name {
        if *relation ==  "mother"{
            if let Some(ref _mother) = root.mother {
                println!("Relationship already exists");
                return true;
            }
            root.mother = Some(Box::new(init_node(new_name.to_string())));
        } else {
            if let Some(ref _father) = root.father {
                println!("Relationship already exists");
                return true;
            }
            root.father = Some(Box::new(init_node(new_name.to_string())));
        }
        return true;
    }
    if let Some(ref mut mother) = root.mother {
        added = added || add(mother, new_name, relation, old_name);
    }
    if let Some(ref mut father) = root.father {
        added = added || add(father, new_name, relation, old_name);
    }
    return added;
}

fn delete(root: &mut TreeNode, name: &String) -> bool {
    let mut deleted: bool = false;
    if let Some(ref mut mother) = root.mother {
        if mother.name ==  *name {
            root.mother = None;
            return true;
        } else {
            deleted = deleted || delete(mother, name);
        }
    }
    if let Some(ref mut father) = root.father {
        if father.name ==  *name {
            root.father = None;
            return true;
        } else {
            deleted = deleted || delete(father, name);
        }
    }
    return deleted;
}

fn main() {
    println!("Please enter your name");
    let mut root_name = String::new();
    io::stdin().read_line(&mut root_name).expect("Error receiving input");
    let mut root = init_node(root_name.trim().to_string());
    loop {
        println!("Next action please");
        let mut action = String::new();
        io::stdin().read_line(&mut action).expect("Error receiving input");
        let split = action.trim().split(" ");
        let vec: Vec<&str> = split.collect();
        if vec[0]  == "quit"{
            break;
        } else if vec[0] == "add" {
            if vec[2] != "mother" && vec[2] != "father" {
                println!("Invalid relationship");
                continue;
            } 
            let added = add(&mut root, &vec[1].to_string(), &vec[2].to_string(), &vec[3].to_string());
            if ! added {
                println!("Name not found");
            }
        } else if vec[0] == "delete" {
            let deleted = delete(&mut root, &vec[1].to_string());
            if deleted {
                println!("Delete completed");
            } else {
                println!("Name not found");
            }
        } else if vec[0] == "print" {
            print_tree(&root, 0);
        } else {
            println!("Invalid command");
        }
    }
    println!("Good Bye");
}
