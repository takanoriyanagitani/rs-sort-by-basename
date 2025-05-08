use std::io;

use std::io::BufRead;

use std::io::BufWriter;
use std::io::Write;

use std::path::Path;

pub struct Filepath {
    pub base: String,
    pub full: String,
}

impl From<String> for Filepath {
    fn from(s: String) -> Self {
        let p: &Path = s.as_ref();
        let oo: Option<_> = p.file_name();
        let obase: Option<&str> = oo.and_then(|o| o.to_str());
        let basestr: &str = obase.unwrap_or_default();
        let base: String = basestr.into();
        Self { full: s, base }
    }
}

impl Filepath {
    pub fn sort_by_basename(s: &mut [Self]) {
        s.sort_by(|x: &Self, y: &Self| {
            let xb: &str = &x.base;
            let yb: &str = &y.base;
            xb.cmp(yb)
        })
    }
}

pub fn reader2strings2vec<R>(brdr: R) -> Vec<String>
where
    R: BufRead,
{
    let lines = brdr.lines();
    let noerr = lines.map_while(Result::ok);
    noerr.collect()
}

pub fn stdin2strings2vec() -> Vec<String> {
    let i = io::stdin();
    let l = i.lock();
    reader2strings2vec(l)
}

pub fn strings2path2sorted(s: Vec<String>) -> impl Iterator<Item = String> {
    let mapd = s.into_iter().map(Filepath::from);
    let mut mid: Vec<Filepath> = mapd.collect();
    Filepath::sort_by_basename(&mut mid);
    mid.into_iter().map(|fp| fp.full)
}

pub fn strings2writer<W, I>(mut wtr: W) -> impl FnMut(I) -> Result<(), io::Error>
where
    W: Write,
    I: Iterator<Item = String>,
{
    move |strings| {
        let mut bw = BufWriter::new(&mut wtr);
        for line in strings {
            writeln!(&mut bw, "{line}")?;
        }
        bw.flush()?;
        drop(bw);

        wtr.flush()
    }
}

pub fn strings2stdout<I>(strings: I) -> Result<(), io::Error>
where
    I: Iterator<Item = String>,
{
    let o = io::stdout();
    let l = o.lock();

    strings2writer(l)(strings)
}

pub fn stdin2strings2sorted2stdout() -> Result<(), io::Error> {
    let original: Vec<String> = stdin2strings2vec();
    let sorted = strings2path2sorted(original);
    strings2stdout(sorted)
}
