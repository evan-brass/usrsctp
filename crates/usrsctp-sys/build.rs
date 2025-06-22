use std::fs::read_dir;
use std::io::Error;
use std::ffi::OsStr;

fn main() -> Result<(), Error> {
	let mut files = Vec::new();
	let dirs = [
		read_dir("../../usrsctplib")?,
		read_dir("../../usrsctplib/netinet")?,
		read_dir("../../usrsctplib/netinet6")?,
	];
	for e in dirs.into_iter().flatten() {
		let path = e?.path();
		if path.extension() != Some(OsStr::new("c")) { continue }

		files.push(path);
	}

	let bindings = bindgen::builder()
		.raw_line("#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]")
		.clang_args([
			"-I../../usrsctplib",
			"-D__Userspace__",
			"-DSCTP_SIMPLE_ALLOCATOR",
			"-DSCTP_PROCESS_LEVEL_LOCKS",
			"-D__APPLE_USE_RFC_2292",
		])
		.header("../../usrsctplib/usrsctp.h")
		.default_enum_style(bindgen::EnumVariation::Consts)
		.allowlist_type("sctp_.+")
		.allowlist_function("usrsctp_.+")
		.generate()
		.map_err(Error::other)?;
	bindings.write_to_file("src/lib.rs")?;

	cc::Build::new()
		.define("__Userspace__", Some(""))
		.define("SCTP_SIMPLE_ALLOCATOR", Some(""))
		.define("SCTP_PROCESS_LEVEL_LOCKS", Some(""))
		.define("__APPLE_USE_RFC_2292", Some(""))
		.files(files)
		.include("../../usrsctplib")
		.try_compile("usrsctp")
		.map_err(Error::other)?;

	Ok(())
}

