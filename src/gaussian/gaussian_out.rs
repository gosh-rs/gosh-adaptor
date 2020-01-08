// energy

// [[file:~/Workspace/Programming/gosh-rs/adaptors/adaptors.note::*energy][energy:1]]
#[test]
fn test_gaussian_out() {
    let txt = r"
Test job not archived.
1\1\GINC-ARCHTOWER\Force\RB3LYP\STO-3G\C3H8\YBYYGU\19-Apr-2019\0\\#p f
orce B3LYP/STO-3G nosymm geom=connectivity fchk=all test\\Title Card R
equired\\0,1\C,-3.48071446,0.16877637,0.04556497\H,-3.12406003,-0.8400
3363,0.04556497\H,-3.12404162,0.67317456,-0.82808654\H,-4.55071446,0.1
6878955,0.04556497\C,-2.96737224,0.89473264,1.30296994\H,-3.32564135,0
.39146421,2.17661983\H,-1.89737405,0.89302596,1.30394869\C,-3.47838792
,2.34747522,1.30156304\H,-4.5483857,2.34918139,1.3002081\H,-3.12202242
,2.85176548,2.17540225\H,-3.11981182,2.85085146,0.42810123\\Version=ES
64L-G09RevD.01\HF=-117.7266856\RMSD=2.913e-09\RMSF=1.468e-02\Dipole=0.
0046287,-0.0033005,0.0057148\Quadrupole=0.1438175,-0.1845061,0.0406886
,0.089481,-0.1310461,-0.158725\PG=C01 [X(C3H8)]\\@
";

    // remove line endings

    let lines: Vec<_> = txt.lines().map(|line| line.trim_end()).collect();
    let s = lines.join("");
    // println!("{}", s.replace(r"\", "\n"));

}
// energy:1 ends here
