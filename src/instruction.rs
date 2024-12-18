#[derive(Debug, Clone)]
pub enum Instruction {
    NOP,
    PUSH,
    POP,
    PEEK,
    CALL,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JNE,
    JE,
    JGE,
    JG,
    JLE,
    JL,
    MOV,
    AND,
    OR,
    XOR,
    NOT,
    LSH,
    RSH,
    VAR,
    RET,
    DEREF,
    REF,
    INST,
    MOD,
    PMOV,
    ALLOC,
    FREE,
    CALLC,
    CMP,
}