use askama::Template;

// Define a struct to hold the variable values
struct MyTemplateVariables {
    model_name: String,
    record_prefix: String,
}

// Define the template using Askama syntax
#[derive(askama::Template)]
#[template(path = "EasierRepository.java.j2")]
struct MyTemplate {
    variables: MyTemplateVariables,
}


fn main() {
// Render the template with the variable values
    let variables = MyTemplateVariables {
        model_name: "MyModel".to_owned(),
        record_prefix: "Record_".to_owned(),
    };
    let template = MyTemplate { variables };
    let rendered = template.render().unwrap();
    println!("{}", rendered);
}
