fn main() {
    #[cfg(target_os = "windows")]
    {
        let _ = embed_resource::compile("tray.rc", embed_resource::NONE);
    }
}
