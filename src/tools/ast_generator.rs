use std::fs::File;
use std::io::{ Write, Result };

pub fn define_ast(target_dir: &str, base_name: &str, types: Vec<&str>) -> Result<()> {

    let path = format!("{}/{}.rs", target_dir, base_name);
    println!("{}", &path);
    let mut out = File::create(path).expect("Can't create the file");

    // importing what we'll be using
    writeln!(out, "use std::boxed::Box;\nuse super::token::{{ Token, TokenType, Object, KEYWORDS }};\n").unwrap();

    define_visitor(&mut out, base_name, &types).unwrap();

    // The AST
    writeln!(out, "pub enum {} {{\n", base_name).unwrap();
    for type_def in &types {
        let parts: Vec<&str> = type_def.split(':').collect();
        let struct_name = parts[0].trim();
        let field_list = parts[1].trim();
        define_type(&mut out, base_name, struct_name, field_list).unwrap();
    }
    writeln!(out, "}}\n").unwrap();

    define_accept_impl(&mut out, base_name, &types).unwrap();

    Ok(())

}

fn define_type(out: &mut File, base_name: &str, struct_name: &str, field_list: &str) -> Result<()> {

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
        let type_def = type_def.split(':').next().unwrap().trim();
        let method_name = format!("visit_{}_{}", type_def.to_lowercase(), base_name.to_lowercase());
        writeln!(
            out,
            "    fn {}(&mut self, expr: &{}) -> T;",
            method_name, type_def
        )?;
    }

    writeln!(out, "\n}}\n")?;
    Ok(())

}


fn define_accept_impl(out: &mut File, base_name: &str, types: &Vec<&str>) -> Result<()> {

    writeln!(out, "impl {} {{\n", base_name)?;
    writeln!(out, "    pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {{")?;
    writeln!(out, "        match self {{")?;

    for type_def in types {
        let type_name = type_def.split(':').next().unwrap().trim();
        writeln!(
            out,
            "            {}::{} {{ .. }} => visitor.visit_{}_{}(self),",
            base_name,
            type_name,
            type_name.to_lowercase(),
            base_name.to_lowercase()
        )?;
    }

    writeln!(out, "        }}")?;
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
