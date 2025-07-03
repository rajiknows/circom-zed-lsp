pub struct ProgramArchive {
    pub inner: program_structure::program_archive::ProgramArchive,
}
impl ProgramArchive {
    pub fn new(inner: program_structure::program_archive::ProgramArchive) -> Self {
        Self { inner }
    }
}
impl std::fmt::Debug for ProgramArchive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProgramArchive").finish()
    }
}
