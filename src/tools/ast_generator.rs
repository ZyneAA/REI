use std::fs::File;
use std::io::{ Write, Result };

pub fn define_ast(target_dir: &str, base_name: &str, types: Vec<&str>) -> Result<()> {

    let path = format!("{}/{}.rs", target_dir, base_name);
    println!("{}", &path);
    let mut out = File::create(path).expect("Can't create the file");

    // importing what we'll be using
    writeln!(out, "use std::boxed::Box;\n use super::token::Token;\n").unwrap();

    writeln!(out, "pub enum {} {{", base_name).unwrap();

    for type_def in types {
        let parts: Vec<&str> = type_def.split(':').collect();
        let struct_name = parts[0].trim();
        let field_list = parts[1].trim();
        define_type(&mut out, base_name, struct_name, field_list).unwrap();
    }

    writeln!(out, "}}\n").unwrap();

    Ok(())

}

fn define_type(out: &mut File, base_name: &str, struct_name: &str, field_list: &str) -> Result<()> {

    writeln!(out, " {} {{", struct_name)?;

    let fields: Vec<&str> = field_list.split(", ").collect();

    for field in fields {
        let mut parts = field.split_whitespace();
        let ty = parts.next().unwrap();
        let name = parts.next().unwrap();
        writeln!(out, "        {}: {},", name, rustify_type(ty))?;
    }

    writeln!(out, "    }},\n")?;

    Ok(())

}

fn rustify_type(ty: &str) -> &str {

    match ty {
        "Expr" => "Box<Expr>",
        "Token" => "Token",
        _ => ty,
    }

}
