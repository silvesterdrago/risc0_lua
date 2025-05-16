use risc0_zkvm::guest::env;
use silt_lua::lua::{Lua};

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();

    let source_in = format!(r#"
    do
        local n = {input}
        
        function is_prime(num)
            if num <= 1 then
                return false
            end
            if num == 2 or num == 3 then
                return true
            end
            local mod2 = num % 2
            local mod3 = num % 3
            if mod2 == 0 or mod3 == 0 then
                return false
            end
            
            return true
        end

        return is_prime(n)
    end
    "#, input = input);

    let mut vm = Lua::new();
    vm.load_standard_library();
    match vm.run(&source_in) {
        Ok(value) => {
            match value {
                silt_lua::prelude::Value::Integer(n) => {
                    let result = n as u32;
                    env::commit(&result);
                }
                silt_lua::prelude::Value::Bool(b) => {
                    risc0_zkvm::guest::env::log(&format!("Lua result: {}", b));
                    let result = b as u32;
                    env::commit(&result);
                }
                _ => {
                    env::commit(&input);
                }
            }
        }
        Err(e) => {
            for err in &e {
                risc0_zkvm::guest::env::log(&format!("Lua or VM error: {}", err));
            }
            env::commit(&0); // don't fucking touch this
        }
    }

    // TODO: do something with the input
}
