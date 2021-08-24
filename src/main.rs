use handlebars::{Handlebars, Output};
use std::collections::HashMap;
use clap::{Arg, App};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, Read};

fn stdin_helper(
    _: &handlebars::Helper, _: &handlebars::Handlebars,
    _: &handlebars::Context, _: &mut handlebars::RenderContext,
    out: &mut dyn Output
) -> Result<(), handlebars::RenderError> {
    let mut contents = String::new();
    io::stdin().read_to_string(&mut contents)?;
    out.write(contents.as_str())?;
    Ok(())
}

fn context_from_file(context: &mut HashMap<String, String>, filename: &str) -> Result<(), Box<dyn Error>> {
    for entry in io::BufReader::new(File::open(filename)?).lines().map(|v| v.unwrap()) {
        let line = entry.trim();
        if line.is_empty() || line.chars().nth(0) == Some('#') {
            continue;
        }

        let line = entry.splitn(2, '=').collect::<Vec<&str>>();
        context.insert(line[0].trim().to_string(), line[1].trim().to_string());
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let application = App::new("Renderbars")
        .version("0.1.0")
        .author("Ethan D. Twardy <ethan.twardy@gmail.com>")
        .about("Render Handlebars templates on the command line")
        .arg(Arg::with_name("context")
             .help("Set a variable in the context. Takes a parameter of the \
                    form 'name=val'")
             .short("c")
             .long("context")
             .takes_value(true)
             .number_of_values(1)
             .multiple(true))
        .arg(Arg::with_name("input")
             .help("Input file name")
             .required(true)
             .index(1))
        .arg(Arg::with_name("output")
             .help("Output file name. If not provided, renders to stdout."))
        .arg(Arg::with_name("context-file")
            .help("Read context from file")
            .short("f")
            .long("context-file")
            .takes_value(true)
            .number_of_values(1)
         );
    let matches = application.get_matches();

    let mut handlebars = Handlebars::new();
    handlebars.register_helper("stdin", Box::new(stdin_helper));
    // raise RenderError if variable does not exist
    handlebars.set_strict_mode(true);

    let mut data = HashMap::new();
    if let Some(context_file) = matches.value_of("context-file") {
        context_from_file(&mut data, context_file)?;
    }
    if let Some(context) = matches.values_of("context") {
        for entry in context.collect::<Vec<&str>>() {
            assert!(entry.contains('='),
		    "Context field '{}' must be of the form 'name=val'",
		    entry);
            let entry = entry.splitn(2, '=').collect::<Vec<&str>>();
            if entry[1].starts_with("{{") || entry[1].starts_with("\\{{") {
                data.insert(entry[0].to_string(),
			    handlebars.render_template(entry[1], &data)?);
            } else {
                data.insert(entry[0].to_string(), entry[1].to_string());
            }
        }
    }

    handlebars.register_template_file(
	"template", matches.value_of("input").unwrap()).unwrap();
    // Grab output filename from args, or use stdout
    let mut output_file: Box<dyn io::Write> = match matches.value_of("output")
    {
	Some(filename) => Box::new(File::create(filename)?),
        None => Box::new(io::stdout()),
    };
    handlebars.render_to_write("template", &data, &mut output_file)?;

    Ok(())
}
