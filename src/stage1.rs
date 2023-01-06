use stl_io::{self, Vertex};
use std::fs::OpenOptions;

mod normal;

fn munge(v: &mut Vertex) {
    if normal::near_xr(v, 265.72113, 28.0701692088578, 0.1) {
        normal::delta_xr(v, -2.0, -2.0);
    }
}

fn main() {
    let path = "jet/files/LPT_Stage_1_No_Supports.stl";
    let mut file = OpenOptions::new().read(true).open(path).unwrap();
    let mesh = stl_io::read_stl(&mut file).unwrap();
    let mut mesh = normal::reduce(&mesh);
    for t in mesh.iter_mut() {
        munge(&mut t.vertices[0]);
        munge(&mut t.vertices[1]);
        munge(&mut t.vertices[2]);
        normal::renormal(t);
    }
    let mut file = OpenOptions::new().write(true).create(true).open("stage1.stl").unwrap();
    stl_io::write_stl(&mut file, mesh.iter()).unwrap();
}
