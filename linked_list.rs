
#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Box<Node>>
}

struct LinkedList {
    head: Option<Box<Node>>
}

impl LinkedList {
    fn add_node(&mut self, data: i32){
        let new_node = Box::new( Node{
            data:data,
            next: None
        });
        
        
        match self.head {
            Some(ref mut node) =>{
                let mut current = node.as_mut();
                
                loop {
                    match current.next {
                        Some(ref mut node) => {
                            current = node.as_mut();
                        },
                        _ => {
                            current.next = Some(new_node);
                            break;
                        }
                    }
                }
            },
            _ =>{
                self.head = Some(new_node);
            }
        }
    }
    
    fn remove_node_at_index(&mut self, index:usize){
        if index == 0 {
            if let Some(ref mut head) = self.head {
                self.head = head.next.take();
            }
        }
        else {
            let mut current_index = 0;
            let mut current = &mut self.head;
            
            while let Some(ref mut node) = current {
                if current_index + 1 == index {
                    if let Some(ref mut next_node) = node.next { 
                        node.next = next_node.next.take();
                    }
                }
                current = &mut node.next;
                current_index += 1;
            }
        }
    }
    
    fn print_list(&self) {
        let mut current = &self.head;
        while let Some(ref node) = current {
            println!("{}", node.data);
            current = &node.next;
        }
    }
}

fn main() {

    let mut ll = LinkedList{head:None};
    
    ll.add_node(22);
    
    ll.add_node(21);
    
    ll.print_list();
    
    ll.remove_node_at_index(1);
    
    ll.print_list()
    
}
