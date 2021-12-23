// imports
// #+name: fe4a6230

use super::*;

// spec
// 解析用于指定选定的区域命令参数, 即行范围. 比如 1-3 相当于选择 1-3 行, 1 相当于选
// 择行 1. "1-3,6-9" 选择1-3 及 6-9 行.

// #+name: a7c4eb05

use text_parser::parsers::*;

#[derive(Debug, Clone)]
struct Selection {
    line_beg: usize,
    line_end: usize,
}

impl Selection {
    fn new(line_beg: usize, line_end: usize) -> Self {
        assert!(line_beg <= line_end);
        Self { line_beg, line_end }
    }
}

impl std::fmt::Display for Selection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.line_beg, self.line_end)
    }
}

impl Selection {
    pub fn parse_multi_selection(s: &str) -> Result<Vec<Self>> {
        let (_, s) = line_blocks(s).nom_trace_err()?;
        Ok(s)
    }

    pub fn parse_selection(s: &str) -> Result<Self> {
        let (_, s) = line_block(s).nom_trace_err()?;
        Ok(s)
    }

    pub fn format_multi_selection(selections: &[Self]) -> String {
        let spec = selections.iter().map(|x| x.to_string()).collect_vec();
        spec.join(",")
    }
}

// 1
fn line_block_1(s: &str) -> IResult<&str, Selection> {
    let (rest, line_beg) = unsigned_digit(s)?;
    // alt(line_block, digit)(s)
    let spec = Selection::new(line_beg, line_beg);
    Ok((rest, spec))
}

// 1-3
fn line_block_2(s: &str) -> IResult<&str, Selection> {
    let (rest, (line_beg, line_end)) = separated_pair(unsigned_digit, tag("-"), unsigned_digit)(s)?;
    let spec = Selection::new(line_beg, line_end);
    Ok((rest, spec))
}

// 1-3 or 1
fn line_block(s: &str) -> IResult<&str, Selection> {
    alt((line_block_2, line_block_1))(s)
}

// 1-4,5-10
fn line_blocks(s: &str) -> IResult<&str, Vec<Selection>> {
    separated_list0(tag(","), line_block)(s)
}

#[test]
fn test_line_spec() -> Result<()> {
    let s = "1-3";
    let (_, specs) = line_blocks(s).nom_trace_err()?;
    assert_eq!(specs.len(), 1);
    assert_eq!(specs[0].line_beg, 1);
    assert_eq!(specs[0].line_end, 3);

    let s = "6,1-3,6-9";
    let (_, specs) = line_blocks(s).nom_trace_err()?;
    assert_eq!(specs.len(), 3);
    assert_eq!(specs[0].line_beg, 6);
    assert_eq!(specs[1].line_beg, 1);
    assert_eq!(specs[2].line_end, 9);

    Ok(())
}

// api/core
// #+name: e9b2e6d8

use text_parser::TextViewer;

#[derive(Debug, Clone)]
pub struct Glance {
    view: TextViewer,
    selections: Vec<Selection>,
}

impl Glance {
    pub fn try_from_path(p: &Path) -> Result<Self> {
        let view = TextViewer::try_from_path(p)?;
        let x = Self {
            view,
            selections: vec![],
        };
        Ok(x)
    }

    pub fn goto_line(&mut self, n: usize) {
        self.view.goto_line(n);
    }

    pub fn goto_first_line(&mut self) {
        self.view.goto_line(1);
    }

    pub fn goto_last_line(&mut self) {
        let n = self.view.num_lines();
        self.view.goto_line(n);
    }

    pub fn next_line(&mut self, n: usize) {
        let m = self.view.current_line_num();
        self.view.goto_line(m + n);
    }

    pub fn prev_line(&mut self, n: usize) {
        let m = self.view.current_line_num();
        self.view.goto_line(m - n);
    }

    pub fn search_forward(&mut self, pattern: &str) -> Result<usize> {
        self.view.search_forward(pattern)
    }

    pub fn select_lines(&mut self, spec: &str) -> Result<()> {
        let s = Selection::parse_multi_selection(spec)?;
        self.selections = s;

        Ok(())
    }

    pub fn select_lines_relative(&mut self, spec: &str) -> Result<()> {
        let n = self.view.current_line_num();
        let mut selections = Selection::parse_multi_selection(spec)?;
        for selection in selections.iter_mut() {
            selection.line_beg += n;
            selection.line_end += n;
        }
        self.selections = selections;

        Ok(())
    }
    pub fn print_selection(&self) -> String {
        let s = self
            .selections
            .iter()
            .map(|x| self.view.peek_lines(x.line_beg, x.line_end))
            .collect_vec();

        s.join("\n")
    }

    pub fn print_column_selection(&self, col_spec: &str) -> Result<String> {
        let c = Selection::parse_selection(col_spec)?;
        let s = self
            .selections
            .iter()
            .map(|x| {
                self.view
                    .column_selection(x.line_beg, x.line_end, c.line_beg, c.line_end)
            })
            .collect_vec();

        Ok(s.join("\n"))
    }
}

// test
// #+name: 37af7667

#[test]
fn test_skim() -> Result<()> {
    let f = "./tests/files/siesta-opt/siesta.log";
    let mut glance = Glance::try_from_path(f.as_ref())?;
    glance.goto_line(22);
    glance.select_lines_relative("1-3");
    let x = glance.print_selection();
    assert_eq!(x, "1 1 H 3\n2 6 C 1\n3 8 O 1\n");

    glance.search_forward("^siesta: Atomic forces")?;
    glance.select_lines_relative("1");
    let x = glance.print_selection();
    assert_eq!(x, "     1    0.171561   -0.146511    0.151677\n");

    glance.select_lines_relative("1-2");
    let x = glance.print_column_selection("7-30")?;
    assert_eq!(x, "   0.171561   -0.146511\n   0.076999    0.320968");

    Ok(())
}
