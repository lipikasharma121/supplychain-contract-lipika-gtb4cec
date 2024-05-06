use soroban_sdk::{contractimport, Env, Bytes, Symbol, BigInt};
#[contractimport]
pub struct Product {
    pub id: Symbol,
    pub current_owner: Symbol,
    pub location: Symbol,
    pub status: Symbol,  
    pub history: Vec<(Symbol, Symbol, Symbol)> 
}

pub trait SupplyChainContract {
    
    fn add_product(env: Env, product: Product);
    fn update_product(env: Env, product_id: Symbol, new_location: Symbol, new_status: Symbol);
    fn transfer_ownership(env: Env, product_id: Symbol, new_owner: Symbol);
    fn get_product_history(env: Env, product_id: Symbol) -> Vec<(Symbol, Symbol, Symbol)>;
}

struct SupplyChainContract;

impl SupplyChainContract for SupplyChainContract {
    fn add_product(env: Env, product: Product) {
    
    }

    fn update_product(env: Env, product_id: Symbol, new_location: Symbol, new_status: Symbol) {
        
    }

    fn transfer_ownership(env: Env, product_id: Symbol, new_owner: Symbol) {
    
    }

    fn get_product_history(env: Env, product_id: Symbol) -> Vec<(Symbol, Symbol, Symbol)> {
       
    }
}
