pub fn reverse(input: &str) -> String {
    let mut original = input.to_string(); 
    let mut response:String = String::new();
    while let Some(value) = original.pop(){
            response.push(value);
        }
    response
}
