fn main() {
    let src = r"C:\Users\USER\Downloads".replace("\\", "/");
    let dst = r"C:\Users\USER\Desktop\temp".replace("\\", "/");
    let docs = [
        "results (5).xlsx",
        "results (4).xlsx",
        "results (3).xlsx",
        "results (2).xlsx",
        "results (1).xlsx",
        "results.xlsx",
    ];
    for (src, dst) in docs
        .iter()
        .map(|p| (format!("{src}/{p}"), format!("{dst}/{p}")))
    {
        std::fs::copy(&src, &dst).unwrap();
        std::fs::remove_file(&src).unwrap();
    }
}
