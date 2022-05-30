use lynis_hardening_index_helper::*;

fn main() {
    check();
    let files1 = find_files_in_folder("/usr/share/lynis");
    let files2 = find_files_in_folder("/etc/lynis");
    files1
        .into_iter()
        .chain(files2.into_iter())
        .for_each(|filename| edit_file(&filename));
    modify_lynis_AddHP();
}
