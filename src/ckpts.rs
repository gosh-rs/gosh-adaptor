// [[file:../adaptors.note::3e26da27][3e26da27]]
use crate::common::*;

use gosh_database::CheckpointDb;
use gosh_model::ModelProperties;

/// Read from checkpointed `ModelProperties`
pub struct Ckpts();

impl crate::ModelAdaptor for Ckpts {
    fn parse_all<P: AsRef<Path>>(&self, dbfile: P) -> Result<Vec<ModelProperties>> {
        use gosh_database::prelude::*;

        let db = CheckpointDb::new(dbfile);
        let n = db.get_number_of_checkpoints::<ModelProperties>()?;
        (0..n).map(|i| db.load_from_slot_n(i as i32)).collect()
    }

    fn parse_last<P: AsRef<Path>>(&self, dbfile: P) -> Result<ModelProperties> {
        let db = CheckpointDb::new(dbfile);
        db.load_from_latest()
    }
}
// 3e26da27 ends here
