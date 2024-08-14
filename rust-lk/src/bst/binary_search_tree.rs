use std::array;

trait BstTrait<T> {
    fn insert(&mut self, element: T);
    fn delete(&mut self, element: T) -> Result<(), String>;
    fn search(&self, element: T) -> Result<(), String>;
    fn size(&self) -> usize;
    fn preOrder(&self) -> Vec<T>;
    fn inOrder(&self) -> Vec<T>;
    fn posOrder(&self) -> Vec<T>;
    fn isComplete(&self) -> bool;
}


#[derive(Clone)]
struct BstNode<T> {
    data: T,
    right: Option<Box<BstNode<T>>>,
    left: Option<Box<BstNode<T>>>,
}

impl<T: PartialOrd> BstNode<T> {
    pub fn new(t_element: T) -> Self {
        BstNode {
            data: t_element,
            right: None,
            left: None,
        }
    }

    pub fn set_right(&mut self, right_node: Option<Box<BstNode<T>>>) {
        self.right = right_node;
    }

    pub fn set_left(&mut self, left_node: Option<Box<BstNode<T>>>) {
        self.left = left_node;
    }

    pub fn get_right(&self) -> Option<&T> {
        self.right.as_ref().map(|node| &node.data)    
    }

    pub fn get_left(&self) -> Option<&T> {
        self.left.as_ref().map(|node| &node.data)
    }
}


#[derive(Clone)]
struct Bstree<T> {
    root: Option<Box<BstNode<T>>>,
}

impl<T: PartialOrd + Clone> Bstree<T> {
    pub fn new(t_element: T) -> Self {
        Bstree {
            root: Some(Box::new(BstNode::new(t_element))),
        }
    }

    fn search_aux(&self, root: Option<Box<BstNode<T>>>, element: T) -> Option<BstNode<T>> {
        if let Some(node) = root {
            if node.data == element {
                return Some(*node);
            }
            else if node.data > element {
                return self.search_aux(node.left, element);
            }
            else if node.data < element {
                return self.search_aux(node.right, element);
            }
        }
        None
    }

    fn size_aux(&self, root: &Option<Box<BstNode<T>>>) -> usize {
        if let Some(node) = root {
            return 1 + self.size_aux(&node.left) + self.size_aux(&node.right);
        }
        return 0;
    }

    fn insert_aux(&mut self, root: &mut Option<Box<BstNode<T>>>, new_node: Box<BstNode<T>>) {
        if let Some(node) = root {
            if new_node.data == node.data {
                // Já existe um nó com o mesmo valor, não faz nada
                return;
            } else if new_node.data < node.data {
                // Inserir na subárvore esquerda
                self.insert_aux(&mut node.left, new_node);
            } else {
                // Inserir na subárvore direita
                self.insert_aux(&mut node.right, new_node);
            }
        } else {
            // `root` é `None`, então aqui é onde o novo nó deve ser inserido
            *root = Some(new_node);
        }
    }

    fn preOrder_aux(&self, node: &Option<Box<BstNode<T>>>, array: Vec<T>, index: i32) -> usize {
        if let Some(node) = self.root {
            array.push(node.data);
            

        }
    }
}

impl<T: PartialOrd + Clone> BstTrait<T> for Bstree<T> {

    fn insert(&mut self, element: T) {
        self.insert_aux(&mut self.root.clone(), Box::new(BstNode::new(element)));
    }

    fn delete(&mut self, element: T) -> Result<(), String> {
        todo!()
    }

    fn search(&self, element: T) -> Result<(), String> {
        match self.search_aux(self.root.clone(), element) {
            Some(_) => Ok(()),
            None => Err("Elemento não encontrado".to_string()),
        }
    }

    fn size(&self) -> usize {
        self.size_aux(&self.root)
    }

    fn preOrder(&self) -> Vec<T> {
        let array = Vec::with_capacity(self.size());
        self.preOrder_aux(&self.root, array, 0);
        
    }

    fn inOrder(&self) -> Vec<T> {
        todo!()
    }

    fn posOrder(&self) -> Vec<T> {
        todo!()
    }

    fn isComplete(&self) -> bool {
        todo!()
    }
}