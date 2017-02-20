use std::collections::HashMap;
use parser::{Schema, Parameter};
use std::fs::File;
use std::io::Write;
use std::error::Error;

struct Constructor {
    name: String,
    params: Vec<Parameter>,
}

#[derive(Default)]
struct Type {
    constructors: Vec<Constructor>,
}

#[derive(Default)]
struct Module {
    types: HashMap<String, Type>,
}

fn translate_typename(typename: &str, current_module: &Option<String>) -> String {
    if typename.contains("Vector<") {
        let s = typename.split(|c| c == '<' || c == '>').collect::<Vec<_>>();
        let typename = translate_typename(s[1], current_module);

        format!("Vec<{}>", typename)
    } else if typename.contains('.') {
        let s = typename.splitn(2, '.').collect::<Vec<_>>();

        if let Some(ref current_module) = *current_module {
            if current_module == s[0] {
                return s[1].to_string();
            }
        }

        format!("::{}::{}", s[0], s[1])
    } else {
        match typename {
            // Primitive conversion
            "string" => "String".to_string(),
            "Bool" => "bool".to_string(),
            "int" => "i32".to_string(),
            "Vec<int>" => "Vec<i32>".to_string(),
            "Vec<long>" => "Vec<i64>".to_string(),
            "long" => "i64".to_string(),
            "double" => "f64".to_string(),
            "bytes" => "Vec<u8>".to_string(),

            _ => format!("::{}", typename)
        }
    }
}

fn translate_id(id: &str, current_module: &Option<String>) -> String {
    if id.contains('.') {
        let s = id.splitn(2, '.').collect::<Vec<_>>();

        if let Some(ref current_module) = *current_module {
            if current_module == s[0] {
                return s[1].to_string();
            }
        }

        format!("{}::{}", s[0], s[1])
    } else {
        match id {
            // Keyword
            "type" => "type_".to_string(),
            _ => id.to_string(),
        }
    }
}

/// Generate Rust definitions to the file from the schema
// TODO(@rust): Use error_chain!
pub fn generate(filename: &str, schema: Schema) -> Result<(), Box<Error>> {
    let mut modules = HashMap::<Option<String>, Module>::new();

    // Translate raw parse
    for constructor in &schema.constructors {
        // Recognized primitive types are ignored when defined
        // and raised to the associated Rust primitive type when requested
        //  - Bool => bool
        //  - True => true (wtf is this; it's not used anywhere in the schema)
        //  - Vector t => Vec<T>
        //  - Null => ? (figure out what to do with this)
        // TODO(@rust): Is there a clean way to check against a constant set ?
        if constructor.kind == "Bool" || constructor.kind == "True" ||
           constructor.kind == "Vector t" || constructor.kind == "Null" {
            continue;
        }

        // Check for exceptions
        if constructor.kind == "PeerSettings" {
            // 1 - PeerSettings doesn't seem to exist (along with the associated method) but its still in the
            //     schema with a seemingly illegal definition
            continue;
        }

        // Split kind into <module>.<name>
        let s = constructor.kind.splitn(2, '.').collect::<Vec<_>>();
        let (module, name) = if s.len() == 1 {
            (None, s[0])
        } else {
            (Some(s[0].to_string()), s[1])
        };

        // Translate
        let c = Constructor {
            name: constructor.predicate.clone(),
            params: constructor.params.clone(),
        };

        // Build up type in module
        let ref mut module_ = modules.entry(module).or_insert_with(Default::default);
        let ref mut type_ = module_.types.entry(name.to_string()).or_insert_with(Default::default);
        type_.constructors.push(c);
    }

    // Output buffered information
    let mut f = File::create(filename).unwrap();
    for (module_name, module) in &modules {
        if let Some(ref module_name) = *module_name {
            // Open module
            writeln!(f, "pub mod {} {{\n", module_name)?;
        }

        for (name, type_) in &module.types {
            writeln!(f, "#[derive(Debug, Deserialize, Serialize)]")?;

            // Open type
            if type_.constructors.len() == 1 {
                // A single constructor is output as a struct
                if type_.constructors[0].params.len() == 0 {
                    // A single constructor with no parameters is a unit
                    writeln!(f, "pub struct {};", name)?;
                    continue;
                } else {
                    writeln!(f, "pub struct {} {{", name)?;
                }
            } else {
                writeln!(f, "pub enum {} {{", name)?;
            }

            for constructor in &type_.constructors {
                let constructor_name = translate_id(&constructor.name, module_name);

                if constructor.params.len() == 0 {
                    // No parameters
                    writeln!(f, "  {},", constructor_name)?;
                } else {
                    // Open constructor (if more than 1)
                    if type_.constructors.len() > 1 {
                        writeln!(f, "  {} {{", constructor_name)?;
                    }

                    // Write out parameters
                    for param in &constructor.params {
                        writeln!(f,
                                 "    {}: {},",
                                 translate_id(&param.name, module_name),
                                 translate_typename(&param.kind, module_name))?;
                    }

                    // Close constructor (if more than 1)
                    if type_.constructors.len() > 1 {
                        writeln!(f, "  }},")?;
                    }
                }
            }

            // Close type
            writeln!(f, "}}\n")?;
        }

        if module_name.is_some() {
            // Close module
            writeln!(f, "}}\n")?;
        }
    }

    Ok(())
}
