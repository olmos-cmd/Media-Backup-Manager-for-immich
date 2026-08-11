fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("app.ico");
        res.set("ProductName", "Media Backup Manager");
        res.set("FileDescription", "Media Backup Manager");
        res.set("OriginalFilename", "Media Backup Manager.exe");
        res.set("CompanyName", "Ralf Ebert");
        res.set("LegalCopyright", "Copyright © Ralf Ebert. Alle Rechte vorbehalten.");

        if let Err(e) = res.compile() {
            panic!("Windows-Ressourcen konnten nicht erstellt werden: {e}");
        }
    }
}
