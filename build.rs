fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("app.ico");
        res.set("ProductName", "Media Backup Manager");
        res.set("FileDescription", "Media Backup Manager");
        res.set("OriginalFilename", "Media Backup Manager.exe");
        res.set("CompanyName", "Ralf Ebert");
        res.set("LegalCopyright", "Copyright © 2026 Ralf Ebert");
        res.set("FileVersion", env!("CARGO_PKG_VERSION"));
        res.set("ProductVersion", env!("CARGO_PKG_VERSION"));
        res.set(
            "Comments",
            "Open-source software licensed under GNU GPL v3.0",
        );

        if let Err(e) = res.compile() {
            panic!("Windows-Ressourcen konnten nicht erstellt werden: {e}");
        }
    }
}
