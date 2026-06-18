use crate::Diag;

pub(super) fn report_diag(diags: Vec<&Diag>) {
    for diag in diags {
        eprintln!("{:?}", diag)
    }
}
