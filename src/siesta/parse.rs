// [[file:../../adaptors.note::7a05223b][7a05223b]]
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::{space0, space1};
use nom::number::complete::double;

use gosh_core::text_parser::parsers::*;
// 7a05223b ends here

// [[file:../../adaptors.note::*energy][energy:1]]
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::sequence::preceded;

/// Read a line excluding line ending.
fn read_line(s: &str) -> IResult<&str, &str> {
    take_until("\n")(s)
}

fn get_total_energy(lines: &str) -> IResult<&str, f64> {
    let token = "siesta:         Total =";
    let (rest, _) = take_until(token)(lines)?;
    let (rest, line) = preceded(tag(token), read_line)(rest)?;
    let energy: f64 = line.trim().parse().unwrap();
    Ok((rest, energy))
}

#[test]
fn test_get_energy() {
    let line = "
siesta: Final energy (eV):
siesta:  Band Struct. =   -8565.028584
siesta:       Kinetic =   24443.230452
siesta:       Hartree =   30079.877056
siesta:       Eldau   =       0.000000
siesta:       Eso     =       0.000000
siesta:    Ext. field =       0.000000
siesta:       Enegf   =       0.000000
siesta:   Exch.-corr. =  -10410.095652
siesta:  Ion-electron =  -81937.616966
siesta:       Ion-ion =      94.811773
siesta:       Ekinion =       0.000000
siesta:         Total =  -37729.793337
siesta:         Fermi =      -2.515979
siesta:         Total =  -37729.793337\n
";
    let (_, en) = get_total_energy(line).unwrap();
    assert_eq!(-37729.793337, en);
}

pub fn get_total_energy_many(s: &str) -> IResult<&str, Vec<f64>> {
    nom::multi::many1(get_total_energy)(s)
}
// energy:1 ends here

// [[file:../../adaptors.note::099f46fd][099f46fd]]
// 1   0.664163041E-01   0.463152759E-01   0.711250774E-01
fn read_forces_line(s: &str) -> IResult<&str, [f64; 3]> {
    do_parse!(
        s,
        space0 >> digit1 >> space1 >> xyz: xyz_array >> line_ending >> (xyz)
    )
}

pub fn get_forces(s: &str) -> IResult<&str, Vec<[f64; 3]>> {
    use nom::multi::count;

    let (s, natoms) = read_usize(s)?;
    count(read_forces_line, natoms)(s)
}

#[test]
fn test_get_forces() {
    let line = " 32
   1   0.664163041E-01   0.463152759E-01   0.711250774E-01
   2  -0.300875794E-01  -0.270188097E-01  -0.163691871E-01
   3   0.420979864E-01  -0.283313586E-01   0.710088560E-01
   4   0.217286620E-01  -0.345173767E-01   0.381290720E-01
   5   0.452048420E-01  -0.949441936E-01   0.181431296E-01
   6  -0.174795321E-02   0.132610004E-01   0.157059456E-01
   7   0.109507439E-01   0.994376192E-03   0.250696188E-01
   8  -0.212038671E-01   0.480355678E-02   0.148474317E-01
   9   0.193064063E-01  -0.155494151E-01   0.137287666E-01
  10  -0.176782527E-01   0.413799649E-02  -0.214259207E-01
  11  -0.145144931E-01  -0.168057116E-02  -0.197795150E-01
  12  -0.972645979E-02   0.929978932E-02  -0.162019163E-01
  13   0.292509955E-02  -0.163170711E-01  -0.183647944E-01
  14  -0.192550007E-01  -0.349771558E-02  -0.834138133E-02
  15   0.418623915E-02   0.337706459E-02  -0.126382738E-01
  16   0.124076852E-01  -0.272336434E-02  -0.277053603E-01
  17  -0.369465864E-03   0.778862801E-02  -0.301159396E-01
  18  -0.864721206E-02  -0.242374010E-02   0.133446112E-01
  19  -0.518933887E-02   0.918026137E-02  -0.173311278E-02
  20  -0.471700806E-02   0.772641001E-02  -0.812663489E-02
  21  -0.950886743E-02  -0.907233616E-02   0.266184499E-02
  22  -0.765114122E-02   0.239194277E-02  -0.857015773E-02
  23   0.438734353E-05   0.738105992E-02  -0.606188360E-02
  24  -0.549614782E-03   0.373057607E-02  -0.119821348E-01
  25   0.200680618E-01  -0.245231128E-01  -0.105198773E-01
  26   0.226599568E-01  -0.705215902E-02  -0.669911666E-02
  27  -0.647351931E-02   0.151523760E-01  -0.172725458E-01
  28  -0.872907837E-02   0.150230899E-01  -0.769116206E-03
  29  -0.137308719E-01   0.145609015E-01  -0.416078751E-01
  30  -0.102261435E-01  -0.650852801E-04  -0.415911977E-02
  31  -0.244065133E-01   0.289808532E-01   0.726568353E-02
  32   0.887815950E-02   0.142891047E-01  -0.153174884E-02
";
    let (_, forces) = get_forces(line).unwrap();
    assert_eq!(32, forces.len());
}
// 099f46fd ends here

// [[file:../../adaptors.note::68246ec1][68246ec1]]
fn get_cell(s: &str) -> IResult<&str, [[f64; 3]; 3]> {
    do_parse!(
        s,
        space0 >> va: xyz_array >> line_ending >> // cell vector a
        space0 >> vb: xyz_array >> line_ending >> // cell vector b
        space0 >> vc: xyz_array >> line_ending >> // cell vector c
        ([va, vb, vc])
    )
}

// read element and coordinates
// 4    45       0.993284236       0.996245743       0.237524061
fn read_atom(s: &str) -> IResult<&str, (&str, [f64; 3])> {
    do_parse!(
        s,
        space0  >> digit1 >> space1 >>      // atom type
        n:      digit1    >> space1 >>      // atomic number
        coords: xyz_array >> line_ending >> // xyz coordinates
        ((n, coords))
    )
}

/// Return cell and atoms
pub fn get_structure(s: &str) -> IResult<&str, ([[f64; 3]; 3], Vec<(&str, [f64; 3])>)> {
    use nom::multi::count;

    let (r, cell) = get_cell(s)?;
    let (r, natoms) = read_usize(r)?;
    let (r, atoms) = count(read_atom, natoms)(r)?;
    Ok((r, (cell, atoms)))
}

#[test]
fn test_get_structure() {
    let s = " 8.070000000       0.000000000       0.000000000
       -4.035000000       6.988825010       0.000000000
        0.000000000       0.000000000      18.540000000
        32
1     1       0.274096080       0.295033079       0.387800756
1     1       0.318246806       0.533498492       0.355543545
1     1       0.442996307       0.220268152       0.291731707
2     6       0.231159083       0.371820528       0.349056718
3     8       0.041725176       0.297890059       0.344679331
4    45       0.993284236       0.996245743       0.237524061
4    45       0.992095258       0.331313822       0.239291516
4    45       0.327565057       0.995134970       0.238145547
4    45       0.325683840       0.336444627       0.246642397
4    45       0.661919740       0.996057778       0.236086761
4    45       0.665647214       0.335054335       0.237924991
4    45       0.997230598       0.667082014       0.236157376
4    45       0.334003903       0.669931944       0.236686803
4    45       0.663288875       0.666525717       0.235955894
4    45       0.222489836       0.114075237       0.117766921
4    45       0.221875701       0.442711172       0.117015194
4    45       0.551032740       0.441592168       0.117043275
4    45       0.553808371       0.111373200       0.117259096
4    45       0.889745957       0.113210125       0.117369993
4    45       0.887884870       0.443126461       0.118575032
4    45       0.221550042       0.778555073       0.117396831
4    45       0.554138712       0.777194107       0.116328890
4    45       0.888133440       0.776965432       0.117131454
4    45       0.110539859       0.221350322      -0.002509633
4    45       0.111449074       0.556103594      -0.002475325
4    45       0.446291127       0.224080745      -0.003135954
4    45       0.445849310       0.557401440      -0.002824889
4    45       0.777900995       0.224435778      -0.002206331
4    45       0.779471420       0.558003403      -0.003724283
4    45       0.108918922       0.885867149      -0.002900800
4    45       0.442894480       0.888302294      -0.002851287
4    45       0.778788669       0.889091725      -0.002546506
";

    let (_, (cell, atoms)) = get_structure(s).unwrap();
    assert_eq!(atoms.len(), 32);
}
// 68246ec1 ends here
