use crate::commands::GenerateArgs;

pub fn validate_generator_args(gen_args: &GenerateArgs) -> Result<&GenerateArgs, String> {
    let length = gen_args.length;
    if !(1..=256).contains(&length) {
        Err(String::from("length should be between 1 and 256"))
    } else {
        Ok(gen_args)
    }
}
