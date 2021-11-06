///////////////////////////////////////////////////////////////////////////////
// NAME:            main.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Entrypoint for the Renderbars application
//
// CREATED:         08/20/2021
//
// LAST EDITED:     11/06/2021
//
// Copyright 2021, Ethan D. Twardy
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
////

use handlebars::{Handlebars, Output, no_escape};
use std::collections::HashMap;
use clap::{Arg, App};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;

fn stdin_helper(
    _: &handlebars::Helper, _: &handlebars::Handlebars,
    _: &handlebars::Context, _: &mut handlebars::RenderContext,
    out: &mut dyn Output
) -> Result<(), handlebars::RenderError> {
    let mut contents = String::new();
    io::stdin().read_to_string(&mut contents)?;
    out.write(contents.trim_end())?;
    Ok(())
}

fn context_from_file(filename: &str) ->
    Result<HashMap<String, String>, Box<dyn Error>>
{
    let mut reader = File::open(filename)?;
    Ok(serde_yaml::from_reader::<&mut File, HashMap<String, String>>(
        &mut reader)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let application = App::new("Renderbars")
        .version(env!("CARGO_PKG_VERSION"))
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
    // Do not escape characters to HTML entities
    handlebars.register_escape_fn(no_escape);

    let mut data = match matches.value_of("context-file") {
        Some(filename) => context_from_file(&filename)?,
        None => HashMap::new(),
    };

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

///////////////////////////////////////////////////////////////////////////////
