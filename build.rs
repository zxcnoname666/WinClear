fn main() -> std::io::Result<()> {
    use winres::WindowsResource;

    WindowsResource::new()
        .set_icon("./bins/build.ico")
        .set("ProductName", "WinClear")
        .set("OriginalFilename", "WinClear.exe")
        .set("FileDescription", "Clear logs of windows")
        .set("LegalCopyright", "Wrote by noname")
        //.set_manifest_file("./bins/installer_manifest.xml")
        .compile()?;

    Ok(())
}