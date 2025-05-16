use risc0_zkvm::guest::env;
use silt_lua::lua::{Lua};

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();

    let source_in = r#"
    do
        local d=5
        function sum()
            local a=1
            local b=2
            local c=3
            return a+b+c+d+8
        end

        return sum()
    end
    "#;
    let mut vm = Lua::new();
    vm.load_standard_library();
    match vm.run(source_in) {
        Ok(_value) => {
            // let result = value.;
            env::commit(&input);
        }
        Err(_e) => {
            env::commit(&input);
        }
    }

    // TODO: do something with the input

    // write public output to the journal
    env::commit(&input);
}
