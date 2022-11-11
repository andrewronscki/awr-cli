use error_chain::error_chain;
use clap::Parser;
use std::path::Path;
use std::{io, fs};
use std::fs::File;
use std::io::prelude::*;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Language that is for use (NestJS or Go)
  language: String,

	/// Command that is for use
  command: String,

	/// Information that is to help the command
	info: String,
}

error_chain! {
	foreign_links {
			Io(std::io::Error);
			HttpRequest(reqwest::Error);
	}
}

// Faz a extração do template zipado
fn unzip_file() -> Result<()> {
	println!("🚀 Extraindo o template zipado...");
	let file = fs::File::open("./download.zip").unwrap();
	let mut archive = zip::ZipArchive::new(file).unwrap();

	for i in 0..archive.len() {
		let mut file = archive.by_index(i).unwrap();

		let outpath = match file.enclosed_name() {
			Some(path) => path.to_owned(),
			None => continue,
		};
		{
			let comment = file.comment();
			if !comment.is_empty() {
				println!("File {} comment:{}", i, comment);
			}
		}

		if (*file.name()).ends_with('/') {
			fs::create_dir_all(&outpath).unwrap();
		} else {
			if let Some(p) = outpath.parent() {
				if !p.exists() {
					fs::create_dir_all(&p).unwrap();
				}
			}
			let mut outfile = fs::File::create(&outpath).unwrap();
			io::copy(&mut file, &mut outfile).unwrap();		
		}
		
		#[cfg(unix)]
		{
			use std::os::unix::fs::PermissionsExt;

			if let Some(mode) = file.unix_mode() {
				fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
			}
		}
 	}

	println!("✅ Extração do template zipado finalizado!");
	Ok(())
}

// Faz a busca do template
#[tokio::main]
async fn find_template() -> Result<()> {
println!("🚀 Fazendo download do template...");
 let target = "https://github.com/andrewronscki/code-styles-nestjs/archive/v1.0.0.zip";
 let response = reqwest::get(target).await?;

 let path = Path::new("./download.zip");

 let mut file = match File::create(&path) {
		 Err(why) => panic!("couldn't create {}", why),
		 Ok(file) => file,
 };
 let content =  response.bytes().await?;
 file.write_all(&content)?;
 
 println!("✅ Download do template finalizado!");
 Ok(())
}

// Seta o nome do projeto para o nome passado por parâmetro
fn set_name(project_name: &String) -> Result<()> {
	println!("🚀 Renomeando pasta do projeto para o nome requerido...");
	
	fs::rename("code-styles-nestjs-1.0.0", project_name)?;

	println!("✅ Pasta renomeada!");
	Ok(())
}

fn delete_zip_file() -> Result<()> {
	println!("🚀 Removendo .zip do template baixado...");

	fs::remove_file("download.zip")?;
  
	println!("✅ .zip removido!");
	Ok(())
}

// Inicia a criação do projeto
fn create_project(project_name: String) -> std::io::Result<()> {
	println!("🚀 Criando projeto {}...", project_name);

	find_template().ok();
	unzip_file().ok();
	set_name(&project_name).ok();
	delete_zip_file().ok();
	
	println!("✅ Projeto {} criado!", project_name);
	Ok(())
}

fn real_main() -> i32  {
  let args = Args::parse();

	if args.language == "nest" {
		println!("🚀 Iniciando criação de projeto NestJS...");

		if args.command == "new" {
			let project_name = args.info; 

			create_project(project_name).ok();
		}

		println!("✅ Criação do projeto NestJS finalizado!");
	}

	0
}

fn main() {
	std::process::exit(real_main());
}