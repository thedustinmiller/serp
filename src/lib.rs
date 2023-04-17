use handlebars::Handlebars;
use std::collections::HashMap;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn format(template: String, map: HashMap<String, String>) -> String {
    let handlebars = Handlebars::new();

    handlebars.render_template(&template, &map).unwrap()

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn print_works() {
        let template = "Hello, {{name}}!".to_string();
		let map = HashMap::from([("name".to_string(), "Ning Sun".to_string())]);

        assert_eq!(format(template, map), "Hello, Ning Sun!".to_string());
    }
}
