// [[file:../../adaptors.note::*imports][imports:1]]
use gosh_core::gut::fs::*;
use gosh_core::gut::prelude::*;
use gosh_model::ModelProperties;

use crate::parser::nom;

use super::parse::*;
// imports:1 ends here

// [[file:../../adaptors.note::*pub][pub:1]]
macro_rules! trace_nom_err {
    ($error:expr, $input:expr) => {
        // early return when found the right parser
        match $error {
            nom::Err::Failure(e) | nom::Err::Error(e) => {
                error!("encouted nom parsing failure.");
                let s = nom::error::convert_error($input, e);
                format_err!("Text parsing failed. Traceback:\n{}", s)
            }
            _ => {
                error!("nom Incomplete error should be found here.");
                unreachable!()
            }
        }
    };
}

pub(crate) fn get_mopac_results<P: AsRef<Path>>(fout: P) -> Result<Vec<ModelProperties>> {
    let s = read_file(fout)?;
    // FIXME: 2021-06-07: why need this since nom 6?
    let s = s.as_str();
    let (_, mps) = get_results(s).map_err(|e| trace_nom_err!(e, s))?;
    // let (_, mps) = get_results(&s).unwrap();

    Ok(mps)
}

#[test]
fn test_parse_mopac() {
    let f = "tests/files/mopac-multiple/mopac.out";
    let mps = get_mopac_results(f).unwrap();
    assert_eq!(mps.len(), 6);
    assert_eq!(mps[5].get_energy(), Some(-747.22443));
}
// pub:1 ends here
