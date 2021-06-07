pub mod ast;
pub mod parser;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(cds, "/parser/cds.rs");
