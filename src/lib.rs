use handlebars::Handlebars;
use std::collections::HashMap;

pub fn format(template: &String, map: &HashMap<String, String>) -> String {
	let handlebars = Handlebars::new();

	handlebars.render_template(template, map).unwrap()
}

#[derive(Debug, PartialEq)]
pub struct Serp {
	pub template: String,
	pub map: HashMap<String, String>,
}

impl Default for Serp {
	fn default() -> Self {
		Self {
			template: "".to_string(),
			map: HashMap::new(),
		}
	}
}

impl Serp {
	pub fn new(template: String, map: HashMap<String, String>) -> Self {
		Self {
			template,
			map,
		}
	}

	pub fn push(&mut self, key: String, value: String){
		self.map.insert(key, value);
	}

	pub fn format(&self) -> String {
		format(&self.template, &self.map)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn default() {
		let def = Serp::default();

		let explicit = Serp {
			template: "".to_string(),
			map: HashMap::new(),
		};

		assert_eq!(def, explicit);
	}

	#[test]
	fn struct_basic_format() {
		let s = Serp {
			template: "{{sample}} {{string}}".to_string(),
			map: HashMap::from(
				[
					("sample".into(), "hello".into()),
					("string".into(), "world".into())
				]
			),
		};
		assert_eq!(s.format(), "hello world");
	}

	#[test]
	fn struct_push() {
		let mut s = Serp{
			template: "{{sample}} {{string}}".to_string(),
			map: HashMap::new(),
		};
		s.push("sample".into(), "hello".into());
		s.push("string".into(), "world".into());
		assert_eq!(s.format(), "hello world");
	}

	#[test]
	fn basic_format() {
		let template = "Hello, {{name}}!".to_string();
		let map = HashMap::from([("name".into(), "Ning Sun".into())]);

		assert_eq!(format(&template, &map), "Hello, Ning Sun!".to_string());
	}
}
