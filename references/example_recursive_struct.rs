use std::rc::Rc;

struct Tree {
    parent: Option<Rc<Tree>>,
    nbr:    i32,
}

impl Tree {
    pub fn new(parent: Option<Rc<Tree>>) -> Tree {
        Tree {
            parent:  parent,
            nbr:    42,
        }
    }

    // pub fn change_parent_nbr(&mut self, new_nbr: i32) {
    //     match self.parent {
    //         Some(ref mut x) => x.nbr = new_nbr,
    //         None        => {},
    //     }
    // }
}

fn main() {
    // Rc<Option<Rc<Tree>>>
    let parent = Rc::new(Tree::new(None));

    // Rc::new(parent) has the lifetime of the function main
    let child1  = Tree::new(Some(parent.clone()));
    let child2  = Tree::new(Some(parent.clone()));

    // error: use of moved value: `parent`
    // parent.nbr = 1;

    // child1.change_parent_nbr(1);

    println!("nbr 1 {}", child1.parent.unwrap().nbr);
    println!("nbr 2 {}", child2.parent.unwrap().nbr);
}
