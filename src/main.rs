pub mod lexer;
pub mod parser;
pub mod token;
pub mod opcode;
pub mod cond;
pub mod register;
pub mod immediate;
pub mod shift;
pub mod psrf;
pub mod cpn;
pub mod label;

use crossterm::{QueueableCommand, style::{self, Stylize}};

use lexer::Lexer;
use opcode::OpCode;
use token::{Token, TokenType};
use label::Label;

use parser::{
    parser::InstrParser,
    brxchg::BrXchgParser,
    branch::BranchParser,
    mul::MulParser,
    mullong::MulLongParser,
    dataproc::DataProcParser,
    transfer::{PsrTransferParser, DataTransferParser, BlockTransferParser},
    swap::DataSwapParser,
    coproc::{CpOpsParser, CpTransfersParser, CpRegTransParser},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // If the file name is missing
    if args.len() < 2 {
        println!("Usage: ./rasm <file>");
        std::process::exit(1);
    }

    // Read the file
    let content = match std::fs::read_to_string(&args[1]) {
        Ok(content) => content,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                println!("No such file or directory.");
                std::process::exit(1);
            },
            _ => {
                println!("Unable to read file.");
                std::process::exit(1);
            },
        },
    };

    // List of labels
    let mut labels = Vec::<Label>::new();
    // Lex the input
    let mut lexer = Lexer::new(&content);
    lexer.lex();

    if lexer.tokens.len() == 0 {
        panic!("Excepted at least one argument.");
    }

    // Split the tokens after each endline
    let split: Vec<&[Token]> = lexer.tokens
        .split(|token| token.token_type == TokenType::Endline)
        .filter(|tokens| !tokens.is_empty())
        .collect();

    // First iteration, parsing only labels
    for tokens in &split {
        // Ignore instructions
        if tokens[0].token_type != TokenType::Label {
            continue;
        }

        // Save the label
        labels.push(Label::from(&tokens[0]));
    }

    // Second iteration, parsing instructions
    for tokens in &split {
        // Ignore labels
        if tokens[0].token_type != TokenType::Keyword {
            continue;
        }

        // We know that the first token must be a keyword which contains an
        // opcode
        let opcode = InstrParser::parse_opcode(&tokens[0])
            .expect("Invalid opcode!");

        // Now that we have our opcode, we match it to its parser and store
        // the result
        let parsed: u32 = match opcode {
            OpCode::BX => {
                // BX{cond} Rn
                BrXchgParser::parse(opcode, &tokens)
            },
            OpCode::B | OpCode::BL => {
                // B{L}{cond} <expression>
                BranchParser::parse(opcode, &tokens, &labels)
            }
            OpCode::AND | OpCode::EOR | OpCode::SUB | OpCode::RSB |
            OpCode::ADD | OpCode::ADC | OpCode::SBC | OpCode::RSC |
            OpCode::TST | OpCode::TEQ | OpCode::CMP | OpCode::CMN |
            OpCode::ORR | OpCode::MOV | OpCode::BIC | OpCode::MVN => {
                // * MOV, MVN:
                //      <opcode>{cond}{S} Rd,<Op2>
                // * CMP, CMN, TEQ, TST:
                //      <opcode>{cond} Rn,<Op2>
                // * AND, EOR, SUB, RSB, ADD, ADC, SBC, RSC, ORR, BIC:
                //      <opcode>{cond}{S} Rd,Rn,<Op2>
                DataProcParser::parse(opcode, &tokens)
            },
            OpCode::MRS | OpCode::MSR => {
                // MRS{cond} Rd,<psr>
                // MSR{cond} <psr>,Rm
                // MSR{cond} <psrf>,Rm
                // MSR{cond} <psrf>,<#expression>
                PsrTransferParser::parse(opcode, &tokens)
            },
            OpCode::MUL | OpCode::MLA => {
                // MUL{cond}{S} Rd,Rm,Rs
                // MLA{cond}{S} Rd,Rm,Rs,Rn
                MulParser::parse(opcode, &tokens)
            },
            OpCode::UMULL | OpCode::UMLAL | OpCode::SMULL | OpCode::SMLAL => {
                // UMULL{cond}{S} RdLo,RdHi,Rm,Rs
                // UMLAL{cond}{S} RdLo,RdHi,Rm,Rs
                // SMULL{cond}{S} RdLo,RdHi,Rm,Rs
                // SMLAL{cond}{S} RdLo,RdHi,Rm,Rs
                MulLongParser::parse(opcode, &tokens)
            },
            OpCode::LDR | OpCode::STR => {
                // <LDR|STR>{cond}{B}{T} Rd,<address>
                // <LDR|STR>{cond}<H|SH|SB> Rd,<address>
                DataTransferParser::parse(opcode, &tokens, &labels)
            },
            OpCode::LDM | OpCode::STM => {
                // <LDM|STM>{cond}<FD|ED|FA|EA|IA|IB|DA|DB> Rn{!},<Rlist>{^}
                BlockTransferParser::parse(opcode, &tokens)
            },
            OpCode::SWP => {
                // <SWP>{cond}{B} Rd,Rm,[Rn]
                DataSwapParser::parse(opcode, &tokens)
            },
            OpCode::CDP => {
                // CDP{cond} p#,<expression1>,cd,cn,cm{,<expression2>}
                CpOpsParser::parse(opcode, &tokens)
            },
            OpCode::LDC | OpCode::STC => {
                // <LDC|STC>{cond}{L} p#,cd,<address>
                CpTransfersParser::parse(opcode, &tokens, &labels)
            },
            OpCode::MRC | OpCode::MCR => {
                // <MCR|MRC>{cond} p#,<expression1>,Rd,cn,cm{,<expression2>}
                CpRegTransParser::parse(opcode, tokens)
            }
            _ => panic!("Opcode not handled yet."),
        };

        // Line count
        let line = format!("{:>width$} | ", tokens[0].line,
            // Calculate the padding needed
            width=split.len().to_string().len());

        // Hex format of the output
        let hex = format!("{:08x} ", parsed.to_be());

        std::io::stdout()
            .queue(style::PrintStyledContent(line.grey())).unwrap()
            .queue(style::PrintStyledContent(hex.green())).unwrap();

        for token in *tokens {
            if token.token_type == TokenType::Comma {
                print!("{}", token.value);
            } else {
                print!(" {}", token.value);
            }
        }
        print!("\n");
    }
}
