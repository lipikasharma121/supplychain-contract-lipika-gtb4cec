use soroban_sdk::{Env, test};

#[test]
fn test_add_product() {
    let env = Env::default();
    let product = Product { }; 

    
    let contract_id = env.register_contract(None, SupplyChainContract);
    let product_id = env.invoke_contract(&contract_id, &product);

}
