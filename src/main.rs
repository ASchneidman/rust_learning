// modules encapsulate code
// each of these modules define one public function 
pub mod doubles;
pub mod scopes;
pub mod references;
pub mod mutability;

fn main() {
    doubles::doubles();
    scopes::scopes();
    references::references();
    mutability::mutability();
} 