use std::fs::File;
use std::io::{ Write, Result };

pub fn define_ast(target_dir: &str, base_name: &str, types: Vec<&str>) -> Result<()> {

    let lower_base_name = &base_name.to_lowercase();
    let path = format!("{}/{}.rs", target_dir, lower_base_name);
    println!("generated in {}", &path);
    let mut out = File::create(path).expect("Can't create the file");

    // importing what we'll be using
    if base_name == "Stmt" {
        writeln!(out, "use std::boxed::Box;\nuse crate::crux::token::Token;\nuse crate::frontend::expr::Expr;\n").unwrap(); 
    }
    else {
        writeln!(out, "use std::boxed::Box;\nuse crate::crux::token::{{ Token, Object }};\n").unwrap(); 
    }

    define_visitor(&mut out, base_name, &types).unwrap();

    // The AST
    writeln!(out, "#[derive(Clone)]");
    writeln!(out, "pub enum {} {{\n", base_name).unwrap();
    for type_def in &types {
        let parts: Vec<&str> = type_def.split(':').collect();
        let struct_name = parts[0].trim();
        let field_list = parts[1].trim();
        define_type(&mut out, struct_name, field_list).unwrap();
    }
    writeln!(out, "}}\n").unwrap();

    define_accept_impl(&mut out, base_name, &types).unwrap();

    Ok(())

}

fn define_type(out: &mut File, struct_name: &str, field_list: &str) -> Result<()> {

    writeln!(out, "    {} {{", struct_name)?;

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

fn define_visitor(out: &mut File, base_name: &str, types: &Vec<&str>) -> Result<()> {

    writeln!(out, "pub trait Visitor<T> {{\n").unwrap();

    for type_def in types {

        let parts = type_def.split(':').nth(1).unwrap();
        let types: Vec<String> = if parts.contains(',') {
            let mut temp: Vec<String> = Vec::new();
            let a: Vec<&str> = parts.split(',').collect();
            for i in a {
                let i: Vec<&str> = i.trim().split(' ').rev().collect();
                let i = i.join(": &");
                temp.push(i);
            }
            temp
        }
        else {
            let a: Vec<&str> = parts.trim().split(' ').rev().collect();
            let a = a.join(": &");
            vec![a]

        };
        let types = types.join(", ");
        let type_def = type_def.split(':').next().unwrap().trim().to_lowercase();
        let method_name = format!("visit_{}_{}", type_def, base_name.to_lowercase());

        writeln!(
            out,
            "    fn {}(&mut self, {}) -> T;",
            method_name,
            types
        )?;
    }

    writeln!(out, "\n}}\n")?;
    Ok(())

}


fn define_accept_impl(out: &mut File, base_name: &str, types: &Vec<&str>) -> Result<()> {

    writeln!(out, "impl {} {{\n", base_name)?;
    writeln!(out, "    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {{\n")?;
    writeln!(out, "        match self {{")?;

    for type_def in types {

        let parts = type_def.split(':').nth(1).unwrap();
        let types: Vec<String> = if parts.contains(',') {
            let mut temp: Vec<String> = Vec::new();
            let a: Vec<&str> = parts.split(',').collect();
            for i in a {
                let i: Vec<&str> = i.trim().split(' ').collect();
                let i = i.get(1).unwrap();
                temp.push(i.to_string());
            }
            temp
        }
        else {
            let a: Vec<&str> = parts.trim().split(' ').collect();
            vec![a.get(1).unwrap().to_string()]

        };

        let types = types.join(", ");
        let type_name = type_def.split(':').next().unwrap().trim();

        writeln!(
            out,
            "            {}::{} {{ {} }} => visitor.visit_{}_{}({}),",
            base_name,
            type_name,
            types,
            type_name.to_lowercase(),
            base_name.to_lowercase(),
            types
        )?;
    }

    writeln!(out, "        }}\n")?;
    writeln!(out, "    }}")?;
    writeln!(out, "\n}}\n")?;
    Ok(())

}

fn rustify_type(ty: &str) -> &str {

    match ty {
        "Expr" => "Box<Expr>",
        "Token" => "Token",
        _ => ty,
    }

}
