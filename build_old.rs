use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use shaderc;

const SHADERS_PATH: &str = "src/renderer/pipelines/shaders/";

// Aurora build script.
fn main() {
    // Get the shaders directory contents
    let paths = fs::read_dir(SHADERS_PATH).unwrap();

    // Shader Extensions
    let ext_frag = std::ffi::OsStr::new("frag");
    let ext_vert = std::ffi::OsStr::new("vert");

    for path in paths {
        let current_path = path.unwrap().path();
        let current_extension = current_path.extension();

        if let Some(ext) = current_extension {
            if ext == ext_frag {
                // Tell Cargo that if the shader files change, to rerun this build script.
                println!("cargo:rerun-if-changed={}", &current_path.display());

                build_shader(current_path.as_path(), shaderc::ShaderKind::Fragment);
            }

            if ext == ext_vert {
                // Tell Cargo that if the shader files change, to rerun this build script.
                println!("cargo:rerun-if-changed={}", &current_path.display());

                build_shader(current_path.as_path(), shaderc::ShaderKind::Vertex);
            }
        }

    }    
}

fn build_shader(path: &std::path::Path, kind: shaderc::ShaderKind) {

    // get the file parts we need for later
    let filename = path.file_name().unwrap().to_str().unwrap();
    let filestem = path.file_stem().unwrap().to_str().unwrap();

    // Open the shader
    let shader_file = fs::File::open(path).unwrap();

    let mut buf_reader = BufReader::new(shader_file);
    let mut source = String::new();
    buf_reader.read_to_string(&mut source).expect("Couldn't read shader");

    // Create the compliler
    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut cmp_options = shaderc::CompileOptions::new().unwrap();
    cmp_options.add_macro_definition("EP", Some("main"));

    let binary_result = compiler.compile_into_spirv(
        &source, kind,
        filename, "main", Some(&cmp_options)).unwrap();

    // new filename
    let mut out_filename = String::from(filestem);
    match kind {
        shaderc::ShaderKind::Fragment => {
            out_filename.push_str("_Fragment.spirv");
        }
        shaderc::ShaderKind::Vertex => {
            out_filename.push_str("_Vertex.spirv");
        },
        _ => {
            out_filename.push_str(".spirv");
        }
    }

    // Output the compiled SPIRV file
    fs::write(path.with_file_name(&out_filename), binary_result.as_binary_u8()).expect("Couldn't write new SPIRV");
}