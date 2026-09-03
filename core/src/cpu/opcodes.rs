// https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html

#[rustfmt::skip]
mod table {
    // Unprefixed opcodes.
    pub const NOP: u8 = 0x00;            // NOP          1      4  - - - -
    pub const LD_BC_D16: u8 = 0x01;      // LD BC,d16    3     12  - - - -
    pub const LD_MEM_BC_A: u8 = 0x02;    // LD (BC),A    1      8  - - - -
    pub const INC_BC: u8 = 0x03;         // INC BC       1      8  - - - -
    pub const INC_B: u8 = 0x04;          // INC B        1      4  Z 0 H -
    pub const DEC_B: u8 = 0x05;          // DEC B        1      4  Z 1 H -
    pub const LD_B_D8: u8 = 0x06;        // LD B,d8      2      8  - - - -
    pub const RLCA: u8 = 0x07;           // RLCA         1      4  0 0 0 C
    pub const LD_MEM_A16_SP: u8 = 0x08;  // LD (a16),SP  3     20  - - - -
    pub const ADD_HL_BC: u8 = 0x09;      // ADD HL,BC    1      8  - 0 H C
    pub const LD_A_MEM_BC: u8 = 0x0A;    // LD A,(BC)    1      8  - - - -
    pub const DEC_BC: u8 = 0x0B;         // DEC BC       1      8  - - - -
    pub const INC_C: u8 = 0x0C;          // INC C        1      4  Z 0 H -
    pub const DEC_C: u8 = 0x0D;          // DEC C        1      4  Z 1 H -
    pub const LD_C_D8: u8 = 0x0E;        // LD C,d8      2      8  - - - -
    pub const RRCA: u8 = 0x0F;           // RRCA         1      4  0 0 0 C
    pub const STOP_0: u8 = 0x10;         // STOP 0       2      4  - - - -
    pub const LD_DE_D16: u8 = 0x11;      // LD DE,d16    3     12  - - - -
    pub const LD_MEM_DE_A: u8 = 0x12;    // LD (DE),A    1      8  - - - -
    pub const INC_DE: u8 = 0x13;         // INC DE       1      8  - - - -
    pub const INC_D: u8 = 0x14;          // INC D        1      4  Z 0 H -
    pub const DEC_D: u8 = 0x15;          // DEC D        1      4  Z 1 H -
    pub const LD_D_D8: u8 = 0x16;        // LD D,d8      2      8  - - - -
    pub const RLA: u8 = 0x17;            // RLA          1      4  0 0 0 C
    pub const JR_R8: u8 = 0x18;          // JR r8        2     12  - - - -
    pub const ADD_HL_DE: u8 = 0x19;      // ADD HL,DE    1      8  - 0 H C
    pub const LD_A_MEM_DE: u8 = 0x1A;    // LD A,(DE)    1      8  - - - -
    pub const DEC_DE: u8 = 0x1B;         // DEC DE       1      8  - - - -
    pub const INC_E: u8 = 0x1C;          // INC E        1      4  Z 0 H -
    pub const DEC_E: u8 = 0x1D;          // DEC E        1      4  Z 1 H -
    pub const LD_E_D8: u8 = 0x1E;        // LD E,d8      2      8  - - - -
    pub const RRA: u8 = 0x1F;            // RRA          1      4  0 0 0 C
    pub const JR_NZ_R8: u8 = 0x20;       // JR NZ,r8     2   12/8  - - - -
    pub const LD_HL_D16: u8 = 0x21;      // LD HL,d16    3     12  - - - -
    pub const LD_MEM_HLI_A: u8 = 0x22;   // LD (HL+),A   1      8  - - - -
    pub const INC_HL: u8 = 0x23;         // INC HL       1      8  - - - -
    pub const INC_H: u8 = 0x24;          // INC H        1      4  Z 0 H -
    pub const DEC_H: u8 = 0x25;          // DEC H        1      4  Z 1 H -
    pub const LD_H_D8: u8 = 0x26;        // LD H,d8      2      8  - - - -
    pub const DAA: u8 = 0x27;            // DAA          1      4  Z - 0 C
    pub const JR_Z_R8: u8 = 0x28;        // JR Z,r8      2   12/8  - - - -
    pub const ADD_HL_HL: u8 = 0x29;      // ADD HL,HL    1      8  - 0 H C
    pub const LD_A_MEM_HLI: u8 = 0x2A;   // LD A,(HL+)   1      8  - - - -
    pub const DEC_HL: u8 = 0x2B;         // DEC HL       1      8  - - - -
    pub const INC_L: u8 = 0x2C;          // INC L        1      4  Z 0 H -
    pub const DEC_L: u8 = 0x2D;          // DEC L        1      4  Z 1 H -
    pub const LD_L_D8: u8 = 0x2E;        // LD L,d8      2      8  - - - -
    pub const CPL: u8 = 0x2F;            // CPL          1      4  - 1 1 -
    pub const JR_NC_R8: u8 = 0x30;       // JR NC,r8     2   12/8  - - - -
    pub const LD_SP_D16: u8 = 0x31;      // LD SP,d16    3     12  - - - -
    pub const LD_MEM_HLD_A: u8 = 0x32;   // LD (HL-),A   1      8  - - - -
    pub const INC_SP: u8 = 0x33;         // INC SP       1      8  - - - -
    pub const INC_MEM_HL: u8 = 0x34;     // INC (HL)     1     12  Z 0 H -
    pub const DEC_MEM_HL: u8 = 0x35;     // DEC (HL)     1     12  Z 1 H -
    pub const LD_MEM_HL_D8: u8 = 0x36;   // LD (HL),d8   2     12  - - - -
    pub const SCF: u8 = 0x37;            // SCF          1      4  - 0 0 1
    pub const JR_C_R8: u8 = 0x38;        // JR C,r8      2   12/8  - - - -
    pub const ADD_HL_SP: u8 = 0x39;      // ADD HL,SP    1      8  - 0 H C
    pub const LD_A_MEM_HLD: u8 = 0x3A;   // LD A,(HL-)   1      8  - - - -
    pub const DEC_SP: u8 = 0x3B;         // DEC SP       1      8  - - - -
    pub const INC_A: u8 = 0x3C;          // INC A        1      4  Z 0 H -
    pub const DEC_A: u8 = 0x3D;          // DEC A        1      4  Z 1 H -
    pub const LD_A_D8: u8 = 0x3E;        // LD A,d8      2      8  - - - -
    pub const CCF: u8 = 0x3F;            // CCF          1      4  - 0 0 C
    pub const LD_B_B: u8 = 0x40;         // LD B,B       1      4  - - - -
    pub const LD_B_C: u8 = 0x41;         // LD B,C       1      4  - - - -
    pub const LD_B_D: u8 = 0x42;         // LD B,D       1      4  - - - -
    pub const LD_B_E: u8 = 0x43;         // LD B,E       1      4  - - - -
    pub const LD_B_H: u8 = 0x44;         // LD B,H       1      4  - - - -
    pub const LD_B_L: u8 = 0x45;         // LD B,L       1      4  - - - -
    pub const LD_B_MEM_HL: u8 = 0x46;    // LD B,(HL)    1      8  - - - -
    pub const LD_B_A: u8 = 0x47;         // LD B,A       1      4  - - - -
    pub const LD_C_B: u8 = 0x48;         // LD C,B       1      4  - - - -
    pub const LD_C_C: u8 = 0x49;         // LD C,C       1      4  - - - -
    pub const LD_C_D: u8 = 0x4A;         // LD C,D       1      4  - - - -
    pub const LD_C_E: u8 = 0x4B;         // LD C,E       1      4  - - - -
    pub const LD_C_H: u8 = 0x4C;         // LD C,H       1      4  - - - -
    pub const LD_C_L: u8 = 0x4D;         // LD C,L       1      4  - - - -
    pub const LD_C_MEM_HL: u8 = 0x4E;    // LD C,(HL)    1      8  - - - -
    pub const LD_C_A: u8 = 0x4F;         // LD C,A       1      4  - - - -
    pub const LD_D_B: u8 = 0x50;         // LD D,B       1      4  - - - -
    pub const LD_D_C: u8 = 0x51;         // LD D,C       1      4  - - - -
    pub const LD_D_D: u8 = 0x52;         // LD D,D       1      4  - - - -
    pub const LD_D_E: u8 = 0x53;         // LD D,E       1      4  - - - -
    pub const LD_D_H: u8 = 0x54;         // LD D,H       1      4  - - - -
    pub const LD_D_L: u8 = 0x55;         // LD D,L       1      4  - - - -
    pub const LD_D_MEM_HL: u8 = 0x56;    // LD D,(HL)    1      8  - - - -
    pub const LD_D_A: u8 = 0x57;         // LD D,A       1      4  - - - -
    pub const LD_E_B: u8 = 0x58;         // LD E,B       1      4  - - - -
    pub const LD_E_C: u8 = 0x59;         // LD E,C       1      4  - - - -
    pub const LD_E_D: u8 = 0x5A;         // LD E,D       1      4  - - - -
    pub const LD_E_E: u8 = 0x5B;         // LD E,E       1      4  - - - -
    pub const LD_E_H: u8 = 0x5C;         // LD E,H       1      4  - - - -
    pub const LD_E_L: u8 = 0x5D;         // LD E,L       1      4  - - - -
    pub const LD_E_MEM_HL: u8 = 0x5E;    // LD E,(HL)    1      8  - - - -
    pub const LD_E_A: u8 = 0x5F;         // LD E,A       1      4  - - - -
    pub const LD_H_B: u8 = 0x60;         // LD H,B       1      4  - - - -
    pub const LD_H_C: u8 = 0x61;         // LD H,C       1      4  - - - -
    pub const LD_H_D: u8 = 0x62;         // LD H,D       1      4  - - - -
    pub const LD_H_E: u8 = 0x63;         // LD H,E       1      4  - - - -
    pub const LD_H_H: u8 = 0x64;         // LD H,H       1      4  - - - -
    pub const LD_H_L: u8 = 0x65;         // LD H,L       1      4  - - - -
    pub const LD_H_MEM_HL: u8 = 0x66;    // LD H,(HL)    1      8  - - - -
    pub const LD_H_A: u8 = 0x67;         // LD H,A       1      4  - - - -
    pub const LD_L_B: u8 = 0x68;         // LD L,B       1      4  - - - -
    pub const LD_L_C: u8 = 0x69;         // LD L,C       1      4  - - - -
    pub const LD_L_D: u8 = 0x6A;         // LD L,D       1      4  - - - -
    pub const LD_L_E: u8 = 0x6B;         // LD L,E       1      4  - - - -
    pub const LD_L_H: u8 = 0x6C;         // LD L,H       1      4  - - - -
    pub const LD_L_L: u8 = 0x6D;         // LD L,L       1      4  - - - -
    pub const LD_L_MEM_HL: u8 = 0x6E;    // LD L,(HL)    1      8  - - - -
    pub const LD_L_A: u8 = 0x6F;         // LD L,A       1      4  - - - -
    pub const LD_MEM_HL_B: u8 = 0x70;    // LD (HL),B    1      8  - - - -
    pub const LD_MEM_HL_C: u8 = 0x71;    // LD (HL),C    1      8  - - - -
    pub const LD_MEM_HL_D: u8 = 0x72;    // LD (HL),D    1      8  - - - -
    pub const LD_MEM_HL_E: u8 = 0x73;    // LD (HL),E    1      8  - - - -
    pub const LD_MEM_HL_H: u8 = 0x74;    // LD (HL),H    1      8  - - - -
    pub const LD_MEM_HL_L: u8 = 0x75;    // LD (HL),L    1      8  - - - -
    pub const HALT: u8 = 0x76;           // HALT         1      4  - - - -
    pub const LD_MEM_HL_A: u8 = 0x77;    // LD (HL),A    1      8  - - - -
    pub const LD_A_B: u8 = 0x78;         // LD A,B       1      4  - - - -
    pub const LD_A_C: u8 = 0x79;         // LD A,C       1      4  - - - -
    pub const LD_A_D: u8 = 0x7A;         // LD A,D       1      4  - - - -
    pub const LD_A_E: u8 = 0x7B;         // LD A,E       1      4  - - - -
    pub const LD_A_H: u8 = 0x7C;         // LD A,H       1      4  - - - -
    pub const LD_A_L: u8 = 0x7D;         // LD A,L       1      4  - - - -
    pub const LD_A_MEM_HL: u8 = 0x7E;    // LD A,(HL)    1      8  - - - -
    pub const LD_A_A: u8 = 0x7F;         // LD A,A       1      4  - - - -
    pub const ADD_A_B: u8 = 0x80;        // ADD A,B      1      4  Z 0 H C
    pub const ADD_A_C: u8 = 0x81;        // ADD A,C      1      4  Z 0 H C
    pub const ADD_A_D: u8 = 0x82;        // ADD A,D      1      4  Z 0 H C
    pub const ADD_A_E: u8 = 0x83;        // ADD A,E      1      4  Z 0 H C
    pub const ADD_A_H: u8 = 0x84;        // ADD A,H      1      4  Z 0 H C
    pub const ADD_A_L: u8 = 0x85;        // ADD A,L      1      4  Z 0 H C
    pub const ADD_A_MEM_HL: u8 = 0x86;   // ADD A,(HL)   1      8  Z 0 H C
    pub const ADD_A_A: u8 = 0x87;        // ADD A,A      1      4  Z 0 H C
    pub const ADC_A_B: u8 = 0x88;        // ADC A,B      1      4  Z 0 H C
    pub const ADC_A_C: u8 = 0x89;        // ADC A,C      1      4  Z 0 H C
    pub const ADC_A_D: u8 = 0x8A;        // ADC A,D      1      4  Z 0 H C
    pub const ADC_A_E: u8 = 0x8B;        // ADC A,E      1      4  Z 0 H C
    pub const ADC_A_H: u8 = 0x8C;        // ADC A,H      1      4  Z 0 H C
    pub const ADC_A_L: u8 = 0x8D;        // ADC A,L      1      4  Z 0 H C
    pub const ADC_A_MEM_HL: u8 = 0x8E;   // ADC A,(HL)   1      8  Z 0 H C
    pub const ADC_A_A: u8 = 0x8F;        // ADC A,A      1      4  Z 0 H C
    pub const SUB_B: u8 = 0x90;          // SUB B        1      4  Z 1 H C
    pub const SUB_C: u8 = 0x91;          // SUB C        1      4  Z 1 H C
    pub const SUB_D: u8 = 0x92;          // SUB D        1      4  Z 1 H C
    pub const SUB_E: u8 = 0x93;          // SUB E        1      4  Z 1 H C
    pub const SUB_H: u8 = 0x94;          // SUB H        1      4  Z 1 H C
    pub const SUB_L: u8 = 0x95;          // SUB L        1      4  Z 1 H C
    pub const SUB_MEM_HL: u8 = 0x96;     // SUB (HL)     1      8  Z 1 H C
    pub const SUB_A: u8 = 0x97;          // SUB A        1      4  Z 1 H C
    pub const SBC_A_B: u8 = 0x98;        // SBC A,B      1      4  Z 1 H C
    pub const SBC_A_C: u8 = 0x99;        // SBC A,C      1      4  Z 1 H C
    pub const SBC_A_D: u8 = 0x9A;        // SBC A,D      1      4  Z 1 H C
    pub const SBC_A_E: u8 = 0x9B;        // SBC A,E      1      4  Z 1 H C
    pub const SBC_A_H: u8 = 0x9C;        // SBC A,H      1      4  Z 1 H C
    pub const SBC_A_L: u8 = 0x9D;        // SBC A,L      1      4  Z 1 H C
    pub const SBC_A_MEM_HL: u8 = 0x9E;   // SBC A,(HL)   1      8  Z 1 H C
    pub const SBC_A_A: u8 = 0x9F;        // SBC A,A      1      4  Z 1 H C
    pub const AND_B: u8 = 0xA0;          // AND B        1      4  Z 0 1 0
    pub const AND_C: u8 = 0xA1;          // AND C        1      4  Z 0 1 0
    pub const AND_D: u8 = 0xA2;          // AND D        1      4  Z 0 1 0
    pub const AND_E: u8 = 0xA3;          // AND E        1      4  Z 0 1 0
    pub const AND_H: u8 = 0xA4;          // AND H        1      4  Z 0 1 0
    pub const AND_L: u8 = 0xA5;          // AND L        1      4  Z 0 1 0
    pub const AND_MEM_HL: u8 = 0xA6;     // AND (HL)     1      8  Z 0 1 0
    pub const AND_A: u8 = 0xA7;          // AND A        1      4  Z 0 1 0
    pub const XOR_B: u8 = 0xA8;          // XOR B        1      4  Z 0 0 0
    pub const XOR_C: u8 = 0xA9;          // XOR C        1      4  Z 0 0 0
    pub const XOR_D: u8 = 0xAA;          // XOR D        1      4  Z 0 0 0
    pub const XOR_E: u8 = 0xAB;          // XOR E        1      4  Z 0 0 0
    pub const XOR_H: u8 = 0xAC;          // XOR H        1      4  Z 0 0 0
    pub const XOR_L: u8 = 0xAD;          // XOR L        1      4  Z 0 0 0
    pub const XOR_MEM_HL: u8 = 0xAE;     // XOR (HL)     1      8  Z 0 0 0
    pub const XOR_A: u8 = 0xAF;          // XOR A        1      4  Z 0 0 0
    pub const OR_B: u8 = 0xB0;           // OR B         1      4  Z 0 0 0
    pub const OR_C: u8 = 0xB1;           // OR C         1      4  Z 0 0 0
    pub const OR_D: u8 = 0xB2;           // OR D         1      4  Z 0 0 0
    pub const OR_E: u8 = 0xB3;           // OR E         1      4  Z 0 0 0
    pub const OR_H: u8 = 0xB4;           // OR H         1      4  Z 0 0 0
    pub const OR_L: u8 = 0xB5;           // OR L         1      4  Z 0 0 0
    pub const OR_MEM_HL: u8 = 0xB6;      // OR (HL)      1      8  Z 0 0 0
    pub const OR_A: u8 = 0xB7;           // OR A         1      4  Z 0 0 0
    pub const CP_B: u8 = 0xB8;           // CP B         1      4  Z 1 H C
    pub const CP_C: u8 = 0xB9;           // CP C         1      4  Z 1 H C
    pub const CP_D: u8 = 0xBA;           // CP D         1      4  Z 1 H C
    pub const CP_E: u8 = 0xBB;           // CP E         1      4  Z 1 H C
    pub const CP_H: u8 = 0xBC;           // CP H         1      4  Z 1 H C
    pub const CP_L: u8 = 0xBD;           // CP L         1      4  Z 1 H C
    pub const CP_MEM_HL: u8 = 0xBE;      // CP (HL)      1      8  Z 1 H C
    pub const CP_A: u8 = 0xBF;           // CP A         1      4  Z 1 H C
    pub const RET_NZ: u8 = 0xC0;         // RET NZ       1   20/8  - - - -
    pub const POP_BC: u8 = 0xC1;         // POP BC       1     12  - - - -
    pub const JP_NZ_A16: u8 = 0xC2;      // JP NZ,a16    3  16/12  - - - -
    pub const JP_A16: u8 = 0xC3;         // JP a16       3     16  - - - -
    pub const CALL_NZ_A16: u8 = 0xC4;    // CALL NZ,a16  3  24/12  - - - -
    pub const PUSH_BC: u8 = 0xC5;        // PUSH BC      1     16  - - - -
    pub const ADD_A_D8: u8 = 0xC6;       // ADD A,d8     2      8  Z 0 H C
    pub const RST_00H: u8 = 0xC7;        // RST 00H      1     16  - - - -
    pub const RET_Z: u8 = 0xC8;          // RET Z        1   20/8  - - - -
    pub const RET: u8 = 0xC9;            // RET          1     16  - - - -
    pub const JP_Z_A16: u8 = 0xCA;       // JP Z,a16     3  16/12  - - - -
    pub const PREFIX_CB: u8 = 0xCB;      // PREFIX CB    1      4  - - - -
    pub const CALL_Z_A16: u8 = 0xCC;     // CALL Z,a16   3  24/12  - - - -
    pub const CALL_A16: u8 = 0xCD;       // CALL a16     3     24  - - - -
    pub const ADC_A_D8: u8 = 0xCE;       // ADC A,d8     2      8  Z 0 H C
    pub const RST_08H: u8 = 0xCF;        // RST 08H      1     16  - - - -
    pub const RET_NC: u8 = 0xD0;         // RET NC       1   20/8  - - - -
    pub const POP_DE: u8 = 0xD1;         // POP DE       1     12  - - - -
    pub const JP_NC_A16: u8 = 0xD2;      // JP NC,a16    3  16/12  - - - -
    pub const CALL_NC_A16: u8 = 0xD4;    // CALL NC,a16  3  24/12  - - - -
    pub const PUSH_DE: u8 = 0xD5;        // PUSH DE      1     16  - - - -
    pub const SUB_D8: u8 = 0xD6;         // SUB d8       2      8  Z 1 H C
    pub const RST_10H: u8 = 0xD7;        // RST 10H      1     16  - - - -
    pub const RET_C: u8 = 0xD8;          // RET C        1   20/8  - - - -
    pub const RETI: u8 = 0xD9;           // RETI         1     16  - - - -
    pub const JP_C_A16: u8 = 0xDA;       // JP C,a16     3  16/12  - - - -
    pub const CALL_C_A16: u8 = 0xDC;     // CALL C,a16   3  24/12  - - - -
    pub const SBC_A_D8: u8 = 0xDE;       // SBC A,d8     2      8  Z 1 H C
    pub const RST_18H: u8 = 0xDF;        // RST 18H      1     16  - - - -
    pub const LDH_MEM_A8_A: u8 = 0xE0;   // LDH (a8),A   2     12  - - - -
    pub const POP_HL: u8 = 0xE1;         // POP HL       1     12  - - - -
    pub const LD_MEM_C_A: u8 = 0xE2;     // LD (C),A     2      8  - - - -
    pub const PUSH_HL: u8 = 0xE5;        // PUSH HL      1     16  - - - -
    pub const AND_D8: u8 = 0xE6;         // AND d8       2      8  Z 0 1 0
    pub const RST_20H: u8 = 0xE7;        // RST 20H      1     16  - - - -
    pub const ADD_SP_R8: u8 = 0xE8;      // ADD SP,r8    2     16  0 0 H C
    pub const JP_MEM_HL: u8 = 0xE9;      // JP (HL)      1      4  - - - -
    pub const LD_MEM_A16_A: u8 = 0xEA;   // LD (a16),A   3     16  - - - -
    pub const XOR_D8: u8 = 0xEE;         // XOR d8       2      8  Z 0 0 0
    pub const RST_28H: u8 = 0xEF;        // RST 28H      1     16  - - - -
    pub const LDH_A_MEM_A8: u8 = 0xF0;   // LDH A,(a8)   2     12  - - - -
    pub const POP_AF: u8 = 0xF1;         // POP AF       1     12  Z N H C
    pub const LD_A_MEM_C: u8 = 0xF2;     // LD A,(C)     2      8  - - - -
    pub const DI: u8 = 0xF3;             // DI           1      4  - - - -
    pub const PUSH_AF: u8 = 0xF5;        // PUSH AF      1     16  - - - -
    pub const OR_D8: u8 = 0xF6;          // OR d8        2      8  Z 0 0 0
    pub const RST_30H: u8 = 0xF7;        // RST 30H      1     16  - - - -
    pub const LD_HL_SP_R8: u8 = 0xF8;    // LD HL,SP+r8  2     12  0 0 H C
    pub const LD_SP_HL: u8 = 0xF9;       // LD SP,HL     1      8  - - - -
    pub const LD_A_MEM_A16: u8 = 0xFA;   // LD A,(a16)   3     16  - - - -
    pub const EI: u8 = 0xFB;             // EI           1      4  - - - -
    pub const CP_D8: u8 = 0xFE;          // CP d8        2      8  Z 1 H C
    pub const RST_38H: u8 = 0xFF;        // RST 38H      1     16  - - - -

    // CB-prefixed opcodes.
    pub const CB_RLC_B: u8 = 0x00;         // RLC B       2   8  Z 0 0 C
    pub const CB_RLC_C: u8 = 0x01;         // RLC C       2   8  Z 0 0 C
    pub const CB_RLC_D: u8 = 0x02;         // RLC D       2   8  Z 0 0 C
    pub const CB_RLC_E: u8 = 0x03;         // RLC E       2   8  Z 0 0 C
    pub const CB_RLC_H: u8 = 0x04;         // RLC H       2   8  Z 0 0 C
    pub const CB_RLC_L: u8 = 0x05;         // RLC L       2   8  Z 0 0 C
    pub const CB_RLC_MEM_HL: u8 = 0x06;    // RLC (HL)    2  16  Z 0 0 C
    pub const CB_RLC_A: u8 = 0x07;         // RLC A       2   8  Z 0 0 C
    pub const CB_RRC_B: u8 = 0x08;         // RRC B       2   8  Z 0 0 C
    pub const CB_RRC_C: u8 = 0x09;         // RRC C       2   8  Z 0 0 C
    pub const CB_RRC_D: u8 = 0x0A;         // RRC D       2   8  Z 0 0 C
    pub const CB_RRC_E: u8 = 0x0B;         // RRC E       2   8  Z 0 0 C
    pub const CB_RRC_H: u8 = 0x0C;         // RRC H       2   8  Z 0 0 C
    pub const CB_RRC_L: u8 = 0x0D;         // RRC L       2   8  Z 0 0 C
    pub const CB_RRC_MEM_HL: u8 = 0x0E;    // RRC (HL)    2  16  Z 0 0 C
    pub const CB_RRC_A: u8 = 0x0F;         // RRC A       2   8  Z 0 0 C
    pub const CB_RL_B: u8 = 0x10;          // RL B        2   8  Z 0 0 C
    pub const CB_RL_C: u8 = 0x11;          // RL C        2   8  Z 0 0 C
    pub const CB_RL_D: u8 = 0x12;          // RL D        2   8  Z 0 0 C
    pub const CB_RL_E: u8 = 0x13;          // RL E        2   8  Z 0 0 C
    pub const CB_RL_H: u8 = 0x14;          // RL H        2   8  Z 0 0 C
    pub const CB_RL_L: u8 = 0x15;          // RL L        2   8  Z 0 0 C
    pub const CB_RL_MEM_HL: u8 = 0x16;     // RL (HL)     2  16  Z 0 0 C
    pub const CB_RL_A: u8 = 0x17;          // RL A        2   8  Z 0 0 C
    pub const CB_RR_B: u8 = 0x18;          // RR B        2   8  Z 0 0 C
    pub const CB_RR_C: u8 = 0x19;          // RR C        2   8  Z 0 0 C
    pub const CB_RR_D: u8 = 0x1A;          // RR D        2   8  Z 0 0 C
    pub const CB_RR_E: u8 = 0x1B;          // RR E        2   8  Z 0 0 C
    pub const CB_RR_H: u8 = 0x1C;          // RR H        2   8  Z 0 0 C
    pub const CB_RR_L: u8 = 0x1D;          // RR L        2   8  Z 0 0 C
    pub const CB_RR_MEM_HL: u8 = 0x1E;     // RR (HL)     2  16  Z 0 0 C
    pub const CB_RR_A: u8 = 0x1F;          // RR A        2   8  Z 0 0 C
    pub const CB_SLA_B: u8 = 0x20;         // SLA B       2   8  Z 0 0 C
    pub const CB_SLA_C: u8 = 0x21;         // SLA C       2   8  Z 0 0 C
    pub const CB_SLA_D: u8 = 0x22;         // SLA D       2   8  Z 0 0 C
    pub const CB_SLA_E: u8 = 0x23;         // SLA E       2   8  Z 0 0 C
    pub const CB_SLA_H: u8 = 0x24;         // SLA H       2   8  Z 0 0 C
    pub const CB_SLA_L: u8 = 0x25;         // SLA L       2   8  Z 0 0 C
    pub const CB_SLA_MEM_HL: u8 = 0x26;    // SLA (HL)    2  16  Z 0 0 C
    pub const CB_SLA_A: u8 = 0x27;         // SLA A       2   8  Z 0 0 C
    pub const CB_SRA_B: u8 = 0x28;         // SRA B       2   8  Z 0 0 0
    pub const CB_SRA_C: u8 = 0x29;         // SRA C       2   8  Z 0 0 0
    pub const CB_SRA_D: u8 = 0x2A;         // SRA D       2   8  Z 0 0 0
    pub const CB_SRA_E: u8 = 0x2B;         // SRA E       2   8  Z 0 0 0
    pub const CB_SRA_H: u8 = 0x2C;         // SRA H       2   8  Z 0 0 0
    pub const CB_SRA_L: u8 = 0x2D;         // SRA L       2   8  Z 0 0 0
    pub const CB_SRA_MEM_HL: u8 = 0x2E;    // SRA (HL)    2  16  Z 0 0 0
    pub const CB_SRA_A: u8 = 0x2F;         // SRA A       2   8  Z 0 0 0
    pub const CB_SWAP_B: u8 = 0x30;        // SWAP B      2   8  Z 0 0 0
    pub const CB_SWAP_C: u8 = 0x31;        // SWAP C      2   8  Z 0 0 0
    pub const CB_SWAP_D: u8 = 0x32;        // SWAP D      2   8  Z 0 0 0
    pub const CB_SWAP_E: u8 = 0x33;        // SWAP E      2   8  Z 0 0 0
    pub const CB_SWAP_H: u8 = 0x34;        // SWAP H      2   8  Z 0 0 0
    pub const CB_SWAP_L: u8 = 0x35;        // SWAP L      2   8  Z 0 0 0
    pub const CB_SWAP_MEM_HL: u8 = 0x36;   // SWAP (HL)   2  16  Z 0 0 0
    pub const CB_SWAP_A: u8 = 0x37;        // SWAP A      2   8  Z 0 0 0
    pub const CB_SRL_B: u8 = 0x38;         // SRL B       2   8  Z 0 0 C
    pub const CB_SRL_C: u8 = 0x39;         // SRL C       2   8  Z 0 0 C
    pub const CB_SRL_D: u8 = 0x3A;         // SRL D       2   8  Z 0 0 C
    pub const CB_SRL_E: u8 = 0x3B;         // SRL E       2   8  Z 0 0 C
    pub const CB_SRL_H: u8 = 0x3C;         // SRL H       2   8  Z 0 0 C
    pub const CB_SRL_L: u8 = 0x3D;         // SRL L       2   8  Z 0 0 C
    pub const CB_SRL_MEM_HL: u8 = 0x3E;    // SRL (HL)    2  16  Z 0 0 C
    pub const CB_SRL_A: u8 = 0x3F;         // SRL A       2   8  Z 0 0 C
    pub const CB_BIT_0_B: u8 = 0x40;       // BIT 0,B     2   8  Z 0 1 -
    pub const CB_BIT_0_C: u8 = 0x41;       // BIT 0,C     2   8  Z 0 1 -
    pub const CB_BIT_0_D: u8 = 0x42;       // BIT 0,D     2   8  Z 0 1 -
    pub const CB_BIT_0_E: u8 = 0x43;       // BIT 0,E     2   8  Z 0 1 -
    pub const CB_BIT_0_H: u8 = 0x44;       // BIT 0,H     2   8  Z 0 1 -
    pub const CB_BIT_0_L: u8 = 0x45;       // BIT 0,L     2   8  Z 0 1 -
    pub const CB_BIT_0_MEM_HL: u8 = 0x46;  // BIT 0,(HL)  2  16  Z 0 1 -
    pub const CB_BIT_0_A: u8 = 0x47;       // BIT 0,A     2   8  Z 0 1 -
    pub const CB_BIT_1_B: u8 = 0x48;       // BIT 1,B     2   8  Z 0 1 -
    pub const CB_BIT_1_C: u8 = 0x49;       // BIT 1,C     2   8  Z 0 1 -
    pub const CB_BIT_1_D: u8 = 0x4A;       // BIT 1,D     2   8  Z 0 1 -
    pub const CB_BIT_1_E: u8 = 0x4B;       // BIT 1,E     2   8  Z 0 1 -
    pub const CB_BIT_1_H: u8 = 0x4C;       // BIT 1,H     2   8  Z 0 1 -
    pub const CB_BIT_1_L: u8 = 0x4D;       // BIT 1,L     2   8  Z 0 1 -
    pub const CB_BIT_1_MEM_HL: u8 = 0x4E;  // BIT 1,(HL)  2  16  Z 0 1 -
    pub const CB_BIT_1_A: u8 = 0x4F;       // BIT 1,A     2   8  Z 0 1 -
    pub const CB_BIT_2_B: u8 = 0x50;       // BIT 2,B     2   8  Z 0 1 -
    pub const CB_BIT_2_C: u8 = 0x51;       // BIT 2,C     2   8  Z 0 1 -
    pub const CB_BIT_2_D: u8 = 0x52;       // BIT 2,D     2   8  Z 0 1 -
    pub const CB_BIT_2_E: u8 = 0x53;       // BIT 2,E     2   8  Z 0 1 -
    pub const CB_BIT_2_H: u8 = 0x54;       // BIT 2,H     2   8  Z 0 1 -
    pub const CB_BIT_2_L: u8 = 0x55;       // BIT 2,L     2   8  Z 0 1 -
    pub const CB_BIT_2_MEM_HL: u8 = 0x56;  // BIT 2,(HL)  2  16  Z 0 1 -
    pub const CB_BIT_2_A: u8 = 0x57;       // BIT 2,A     2   8  Z 0 1 -
    pub const CB_BIT_3_B: u8 = 0x58;       // BIT 3,B     2   8  Z 0 1 -
    pub const CB_BIT_3_C: u8 = 0x59;       // BIT 3,C     2   8  Z 0 1 -
    pub const CB_BIT_3_D: u8 = 0x5A;       // BIT 3,D     2   8  Z 0 1 -
    pub const CB_BIT_3_E: u8 = 0x5B;       // BIT 3,E     2   8  Z 0 1 -
    pub const CB_BIT_3_H: u8 = 0x5C;       // BIT 3,H     2   8  Z 0 1 -
    pub const CB_BIT_3_L: u8 = 0x5D;       // BIT 3,L     2   8  Z 0 1 -
    pub const CB_BIT_3_MEM_HL: u8 = 0x5E;  // BIT 3,(HL)  2  16  Z 0 1 -
    pub const CB_BIT_3_A: u8 = 0x5F;       // BIT 3,A     2   8  Z 0 1 -
    pub const CB_BIT_4_B: u8 = 0x60;       // BIT 4,B     2   8  Z 0 1 -
    pub const CB_BIT_4_C: u8 = 0x61;       // BIT 4,C     2   8  Z 0 1 -
    pub const CB_BIT_4_D: u8 = 0x62;       // BIT 4,D     2   8  Z 0 1 -
    pub const CB_BIT_4_E: u8 = 0x63;       // BIT 4,E     2   8  Z 0 1 -
    pub const CB_BIT_4_H: u8 = 0x64;       // BIT 4,H     2   8  Z 0 1 -
    pub const CB_BIT_4_L: u8 = 0x65;       // BIT 4,L     2   8  Z 0 1 -
    pub const CB_BIT_4_MEM_HL: u8 = 0x66;  // BIT 4,(HL)  2  16  Z 0 1 -
    pub const CB_BIT_4_A: u8 = 0x67;       // BIT 4,A     2   8  Z 0 1 -
    pub const CB_BIT_5_B: u8 = 0x68;       // BIT 5,B     2   8  Z 0 1 -
    pub const CB_BIT_5_C: u8 = 0x69;       // BIT 5,C     2   8  Z 0 1 -
    pub const CB_BIT_5_D: u8 = 0x6A;       // BIT 5,D     2   8  Z 0 1 -
    pub const CB_BIT_5_E: u8 = 0x6B;       // BIT 5,E     2   8  Z 0 1 -
    pub const CB_BIT_5_H: u8 = 0x6C;       // BIT 5,H     2   8  Z 0 1 -
    pub const CB_BIT_5_L: u8 = 0x6D;       // BIT 5,L     2   8  Z 0 1 -
    pub const CB_BIT_5_MEM_HL: u8 = 0x6E;  // BIT 5,(HL)  2  16  Z 0 1 -
    pub const CB_BIT_5_A: u8 = 0x6F;       // BIT 5,A     2   8  Z 0 1 -
    pub const CB_BIT_6_B: u8 = 0x70;       // BIT 6,B     2   8  Z 0 1 -
    pub const CB_BIT_6_C: u8 = 0x71;       // BIT 6,C     2   8  Z 0 1 -
    pub const CB_BIT_6_D: u8 = 0x72;       // BIT 6,D     2   8  Z 0 1 -
    pub const CB_BIT_6_E: u8 = 0x73;       // BIT 6,E     2   8  Z 0 1 -
    pub const CB_BIT_6_H: u8 = 0x74;       // BIT 6,H     2   8  Z 0 1 -
    pub const CB_BIT_6_L: u8 = 0x75;       // BIT 6,L     2   8  Z 0 1 -
    pub const CB_BIT_6_MEM_HL: u8 = 0x76;  // BIT 6,(HL)  2  16  Z 0 1 -
    pub const CB_BIT_6_A: u8 = 0x77;       // BIT 6,A     2   8  Z 0 1 -
    pub const CB_BIT_7_B: u8 = 0x78;       // BIT 7,B     2   8  Z 0 1 -
    pub const CB_BIT_7_C: u8 = 0x79;       // BIT 7,C     2   8  Z 0 1 -
    pub const CB_BIT_7_D: u8 = 0x7A;       // BIT 7,D     2   8  Z 0 1 -
    pub const CB_BIT_7_E: u8 = 0x7B;       // BIT 7,E     2   8  Z 0 1 -
    pub const CB_BIT_7_H: u8 = 0x7C;       // BIT 7,H     2   8  Z 0 1 -
    pub const CB_BIT_7_L: u8 = 0x7D;       // BIT 7,L     2   8  Z 0 1 -
    pub const CB_BIT_7_MEM_HL: u8 = 0x7E;  // BIT 7,(HL)  2  16  Z 0 1 -
    pub const CB_BIT_7_A: u8 = 0x7F;       // BIT 7,A     2   8  Z 0 1 -
    pub const CB_RES_0_B: u8 = 0x80;       // RES 0,B     2   8  - - - -
    pub const CB_RES_0_C: u8 = 0x81;       // RES 0,C     2   8  - - - -
    pub const CB_RES_0_D: u8 = 0x82;       // RES 0,D     2   8  - - - -
    pub const CB_RES_0_E: u8 = 0x83;       // RES 0,E     2   8  - - - -
    pub const CB_RES_0_H: u8 = 0x84;       // RES 0,H     2   8  - - - -
    pub const CB_RES_0_L: u8 = 0x85;       // RES 0,L     2   8  - - - -
    pub const CB_RES_0_MEM_HL: u8 = 0x86;  // RES 0,(HL)  2  16  - - - -
    pub const CB_RES_0_A: u8 = 0x87;       // RES 0,A     2   8  - - - -
    pub const CB_RES_1_B: u8 = 0x88;       // RES 1,B     2   8  - - - -
    pub const CB_RES_1_C: u8 = 0x89;       // RES 1,C     2   8  - - - -
    pub const CB_RES_1_D: u8 = 0x8A;       // RES 1,D     2   8  - - - -
    pub const CB_RES_1_E: u8 = 0x8B;       // RES 1,E     2   8  - - - -
    pub const CB_RES_1_H: u8 = 0x8C;       // RES 1,H     2   8  - - - -
    pub const CB_RES_1_L: u8 = 0x8D;       // RES 1,L     2   8  - - - -
    pub const CB_RES_1_MEM_HL: u8 = 0x8E;  // RES 1,(HL)  2  16  - - - -
    pub const CB_RES_1_A: u8 = 0x8F;       // RES 1,A     2   8  - - - -
    pub const CB_RES_2_B: u8 = 0x90;       // RES 2,B     2   8  - - - -
    pub const CB_RES_2_C: u8 = 0x91;       // RES 2,C     2   8  - - - -
    pub const CB_RES_2_D: u8 = 0x92;       // RES 2,D     2   8  - - - -
    pub const CB_RES_2_E: u8 = 0x93;       // RES 2,E     2   8  - - - -
    pub const CB_RES_2_H: u8 = 0x94;       // RES 2,H     2   8  - - - -
    pub const CB_RES_2_L: u8 = 0x95;       // RES 2,L     2   8  - - - -
    pub const CB_RES_2_MEM_HL: u8 = 0x96;  // RES 2,(HL)  2  16  - - - -
    pub const CB_RES_2_A: u8 = 0x97;       // RES 2,A     2   8  - - - -
    pub const CB_RES_3_B: u8 = 0x98;       // RES 3,B     2   8  - - - -
    pub const CB_RES_3_C: u8 = 0x99;       // RES 3,C     2   8  - - - -
    pub const CB_RES_3_D: u8 = 0x9A;       // RES 3,D     2   8  - - - -
    pub const CB_RES_3_E: u8 = 0x9B;       // RES 3,E     2   8  - - - -
    pub const CB_RES_3_H: u8 = 0x9C;       // RES 3,H     2   8  - - - -
    pub const CB_RES_3_L: u8 = 0x9D;       // RES 3,L     2   8  - - - -
    pub const CB_RES_3_MEM_HL: u8 = 0x9E;  // RES 3,(HL)  2  16  - - - -
    pub const CB_RES_3_A: u8 = 0x9F;       // RES 3,A     2   8  - - - -
    pub const CB_RES_4_B: u8 = 0xA0;       // RES 4,B     2   8  - - - -
    pub const CB_RES_4_C: u8 = 0xA1;       // RES 4,C     2   8  - - - -
    pub const CB_RES_4_D: u8 = 0xA2;       // RES 4,D     2   8  - - - -
    pub const CB_RES_4_E: u8 = 0xA3;       // RES 4,E     2   8  - - - -
    pub const CB_RES_4_H: u8 = 0xA4;       // RES 4,H     2   8  - - - -
    pub const CB_RES_4_L: u8 = 0xA5;       // RES 4,L     2   8  - - - -
    pub const CB_RES_4_MEM_HL: u8 = 0xA6;  // RES 4,(HL)  2  16  - - - -
    pub const CB_RES_4_A: u8 = 0xA7;       // RES 4,A     2   8  - - - -
    pub const CB_RES_5_B: u8 = 0xA8;       // RES 5,B     2   8  - - - -
    pub const CB_RES_5_C: u8 = 0xA9;       // RES 5,C     2   8  - - - -
    pub const CB_RES_5_D: u8 = 0xAA;       // RES 5,D     2   8  - - - -
    pub const CB_RES_5_E: u8 = 0xAB;       // RES 5,E     2   8  - - - -
    pub const CB_RES_5_H: u8 = 0xAC;       // RES 5,H     2   8  - - - -
    pub const CB_RES_5_L: u8 = 0xAD;       // RES 5,L     2   8  - - - -
    pub const CB_RES_5_MEM_HL: u8 = 0xAE;  // RES 5,(HL)  2  16  - - - -
    pub const CB_RES_5_A: u8 = 0xAF;       // RES 5,A     2   8  - - - -
    pub const CB_RES_6_B: u8 = 0xB0;       // RES 6,B     2   8  - - - -
    pub const CB_RES_6_C: u8 = 0xB1;       // RES 6,C     2   8  - - - -
    pub const CB_RES_6_D: u8 = 0xB2;       // RES 6,D     2   8  - - - -
    pub const CB_RES_6_E: u8 = 0xB3;       // RES 6,E     2   8  - - - -
    pub const CB_RES_6_H: u8 = 0xB4;       // RES 6,H     2   8  - - - -
    pub const CB_RES_6_L: u8 = 0xB5;       // RES 6,L     2   8  - - - -
    pub const CB_RES_6_MEM_HL: u8 = 0xB6;  // RES 6,(HL)  2  16  - - - -
    pub const CB_RES_6_A: u8 = 0xB7;       // RES 6,A     2   8  - - - -
    pub const CB_RES_7_B: u8 = 0xB8;       // RES 7,B     2   8  - - - -
    pub const CB_RES_7_C: u8 = 0xB9;       // RES 7,C     2   8  - - - -
    pub const CB_RES_7_D: u8 = 0xBA;       // RES 7,D     2   8  - - - -
    pub const CB_RES_7_E: u8 = 0xBB;       // RES 7,E     2   8  - - - -
    pub const CB_RES_7_H: u8 = 0xBC;       // RES 7,H     2   8  - - - -
    pub const CB_RES_7_L: u8 = 0xBD;       // RES 7,L     2   8  - - - -
    pub const CB_RES_7_MEM_HL: u8 = 0xBE;  // RES 7,(HL)  2  16  - - - -
    pub const CB_RES_7_A: u8 = 0xBF;       // RES 7,A     2   8  - - - -
    pub const CB_SET_0_B: u8 = 0xC0;       // SET 0,B     2   8  - - - -
    pub const CB_SET_0_C: u8 = 0xC1;       // SET 0,C     2   8  - - - -
    pub const CB_SET_0_D: u8 = 0xC2;       // SET 0,D     2   8  - - - -
    pub const CB_SET_0_E: u8 = 0xC3;       // SET 0,E     2   8  - - - -
    pub const CB_SET_0_H: u8 = 0xC4;       // SET 0,H     2   8  - - - -
    pub const CB_SET_0_L: u8 = 0xC5;       // SET 0,L     2   8  - - - -
    pub const CB_SET_0_MEM_HL: u8 = 0xC6;  // SET 0,(HL)  2  16  - - - -
    pub const CB_SET_0_A: u8 = 0xC7;       // SET 0,A     2   8  - - - -
    pub const CB_SET_1_B: u8 = 0xC8;       // SET 1,B     2   8  - - - -
    pub const CB_SET_1_C: u8 = 0xC9;       // SET 1,C     2   8  - - - -
    pub const CB_SET_1_D: u8 = 0xCA;       // SET 1,D     2   8  - - - -
    pub const CB_SET_1_E: u8 = 0xCB;       // SET 1,E     2   8  - - - -
    pub const CB_SET_1_H: u8 = 0xCC;       // SET 1,H     2   8  - - - -
    pub const CB_SET_1_L: u8 = 0xCD;       // SET 1,L     2   8  - - - -
    pub const CB_SET_1_MEM_HL: u8 = 0xCE;  // SET 1,(HL)  2  16  - - - -
    pub const CB_SET_1_A: u8 = 0xCF;       // SET 1,A     2   8  - - - -
    pub const CB_SET_2_B: u8 = 0xD0;       // SET 2,B     2   8  - - - -
    pub const CB_SET_2_C: u8 = 0xD1;       // SET 2,C     2   8  - - - -
    pub const CB_SET_2_D: u8 = 0xD2;       // SET 2,D     2   8  - - - -
    pub const CB_SET_2_E: u8 = 0xD3;       // SET 2,E     2   8  - - - -
    pub const CB_SET_2_H: u8 = 0xD4;       // SET 2,H     2   8  - - - -
    pub const CB_SET_2_L: u8 = 0xD5;       // SET 2,L     2   8  - - - -
    pub const CB_SET_2_MEM_HL: u8 = 0xD6;  // SET 2,(HL)  2  16  - - - -
    pub const CB_SET_2_A: u8 = 0xD7;       // SET 2,A     2   8  - - - -
    pub const CB_SET_3_B: u8 = 0xD8;       // SET 3,B     2   8  - - - -
    pub const CB_SET_3_C: u8 = 0xD9;       // SET 3,C     2   8  - - - -
    pub const CB_SET_3_D: u8 = 0xDA;       // SET 3,D     2   8  - - - -
    pub const CB_SET_3_E: u8 = 0xDB;       // SET 3,E     2   8  - - - -
    pub const CB_SET_3_H: u8 = 0xDC;       // SET 3,H     2   8  - - - -
    pub const CB_SET_3_L: u8 = 0xDD;       // SET 3,L     2   8  - - - -
    pub const CB_SET_3_MEM_HL: u8 = 0xDE;  // SET 3,(HL)  2  16  - - - -
    pub const CB_SET_3_A: u8 = 0xDF;       // SET 3,A     2   8  - - - -
    pub const CB_SET_4_B: u8 = 0xE0;       // SET 4,B     2   8  - - - -
    pub const CB_SET_4_C: u8 = 0xE1;       // SET 4,C     2   8  - - - -
    pub const CB_SET_4_D: u8 = 0xE2;       // SET 4,D     2   8  - - - -
    pub const CB_SET_4_E: u8 = 0xE3;       // SET 4,E     2   8  - - - -
    pub const CB_SET_4_H: u8 = 0xE4;       // SET 4,H     2   8  - - - -
    pub const CB_SET_4_L: u8 = 0xE5;       // SET 4,L     2   8  - - - -
    pub const CB_SET_4_MEM_HL: u8 = 0xE6;  // SET 4,(HL)  2  16  - - - -
    pub const CB_SET_4_A: u8 = 0xE7;       // SET 4,A     2   8  - - - -
    pub const CB_SET_5_B: u8 = 0xE8;       // SET 5,B     2   8  - - - -
    pub const CB_SET_5_C: u8 = 0xE9;       // SET 5,C     2   8  - - - -
    pub const CB_SET_5_D: u8 = 0xEA;       // SET 5,D     2   8  - - - -
    pub const CB_SET_5_E: u8 = 0xEB;       // SET 5,E     2   8  - - - -
    pub const CB_SET_5_H: u8 = 0xEC;       // SET 5,H     2   8  - - - -
    pub const CB_SET_5_L: u8 = 0xED;       // SET 5,L     2   8  - - - -
    pub const CB_SET_5_MEM_HL: u8 = 0xEE;  // SET 5,(HL)  2  16  - - - -
    pub const CB_SET_5_A: u8 = 0xEF;       // SET 5,A     2   8  - - - -
    pub const CB_SET_6_B: u8 = 0xF0;       // SET 6,B     2   8  - - - -
    pub const CB_SET_6_C: u8 = 0xF1;       // SET 6,C     2   8  - - - -
    pub const CB_SET_6_D: u8 = 0xF2;       // SET 6,D     2   8  - - - -
    pub const CB_SET_6_E: u8 = 0xF3;       // SET 6,E     2   8  - - - -
    pub const CB_SET_6_H: u8 = 0xF4;       // SET 6,H     2   8  - - - -
    pub const CB_SET_6_L: u8 = 0xF5;       // SET 6,L     2   8  - - - -
    pub const CB_SET_6_MEM_HL: u8 = 0xF6;  // SET 6,(HL)  2  16  - - - -
    pub const CB_SET_6_A: u8 = 0xF7;       // SET 6,A     2   8  - - - -
    pub const CB_SET_7_B: u8 = 0xF8;       // SET 7,B     2   8  - - - -
    pub const CB_SET_7_C: u8 = 0xF9;       // SET 7,C     2   8  - - - -
    pub const CB_SET_7_D: u8 = 0xFA;       // SET 7,D     2   8  - - - -
    pub const CB_SET_7_E: u8 = 0xFB;       // SET 7,E     2   8  - - - -
    pub const CB_SET_7_H: u8 = 0xFC;       // SET 7,H     2   8  - - - -
    pub const CB_SET_7_L: u8 = 0xFD;       // SET 7,L     2   8  - - - -
    pub const CB_SET_7_MEM_HL: u8 = 0xFE;  // SET 7,(HL)  2  16  - - - -
    pub const CB_SET_7_A: u8 = 0xFF;       // SET 7,A     2   8  - - - -

    pub const MNEMONIC: [&str; 256] = [
        "NOP",         // 0x00
        "LD BC,d16",   // 0x01
        "LD (BC),A",   // 0x02
        "INC BC",      // 0x03
        "INC B",       // 0x04
        "DEC B",       // 0x05
        "LD B,d8",     // 0x06
        "RLCA",        // 0x07
        "LD (a16),SP", // 0x08
        "ADD HL,BC",   // 0x09
        "LD A,(BC)",   // 0x0A
        "DEC BC",      // 0x0B
        "INC C",       // 0x0C
        "DEC C",       // 0x0D
        "LD C,d8",     // 0x0E
        "RRCA",        // 0x0F
        "STOP 0",      // 0x10
        "LD DE,d16",   // 0x11
        "LD (DE),A",   // 0x12
        "INC DE",      // 0x13
        "INC D",       // 0x14
        "DEC D",       // 0x15
        "LD D,d8",     // 0x16
        "RLA",         // 0x17
        "JR r8",       // 0x18
        "ADD HL,DE",   // 0x19
        "LD A,(DE)",   // 0x1A
        "DEC DE",      // 0x1B
        "INC E",       // 0x1C
        "DEC E",       // 0x1D
        "LD E,d8",     // 0x1E
        "RRA",         // 0x1F
        "JR NZ,r8",    // 0x20
        "LD HL,d16",   // 0x21
        "LD (HL+),A",  // 0x22
        "INC HL",      // 0x23
        "INC H",       // 0x24
        "DEC H",       // 0x25
        "LD H,d8",     // 0x26
        "DAA",         // 0x27
        "JR Z,r8",     // 0x28
        "ADD HL,HL",   // 0x29
        "LD A,(HL+)",  // 0x2A
        "DEC HL",      // 0x2B
        "INC L",       // 0x2C
        "DEC L",       // 0x2D
        "LD L,d8",     // 0x2E
        "CPL",         // 0x2F
        "JR NC,r8",    // 0x30
        "LD SP,d16",   // 0x31
        "LD (HL-),A",  // 0x32
        "INC SP",      // 0x33
        "INC (HL)",    // 0x34
        "DEC (HL)",    // 0x35
        "LD (HL),d8",  // 0x36
        "SCF",         // 0x37
        "JR C,r8",     // 0x38
        "ADD HL,SP",   // 0x39
        "LD A,(HL-)",  // 0x3A
        "DEC SP",      // 0x3B
        "INC A",       // 0x3C
        "DEC A",       // 0x3D
        "LD A,d8",     // 0x3E
        "CCF",         // 0x3F
        "LD B,B",      // 0x40
        "LD B,C",      // 0x41
        "LD B,D",      // 0x42
        "LD B,E",      // 0x43
        "LD B,H",      // 0x44
        "LD B,L",      // 0x45
        "LD B,(HL)",   // 0x46
        "LD B,A",      // 0x47
        "LD C,B",      // 0x48
        "LD C,C",      // 0x49
        "LD C,D",      // 0x4A
        "LD C,E",      // 0x4B
        "LD C,H",      // 0x4C
        "LD C,L",      // 0x4D
        "LD C,(HL)",   // 0x4E
        "LD C,A",      // 0x4F
        "LD D,B",      // 0x50
        "LD D,C",      // 0x51
        "LD D,D",      // 0x52
        "LD D,E",      // 0x53
        "LD D,H",      // 0x54
        "LD D,L",      // 0x55
        "LD D,(HL)",   // 0x56
        "LD D,A",      // 0x57
        "LD E,B",      // 0x58
        "LD E,C",      // 0x59
        "LD E,D",      // 0x5A
        "LD E,E",      // 0x5B
        "LD E,H",      // 0x5C
        "LD E,L",      // 0x5D
        "LD E,(HL)",   // 0x5E
        "LD E,A",      // 0x5F
        "LD H,B",      // 0x60
        "LD H,C",      // 0x61
        "LD H,D",      // 0x62
        "LD H,E",      // 0x63
        "LD H,H",      // 0x64
        "LD H,L",      // 0x65
        "LD H,(HL)",   // 0x66
        "LD H,A",      // 0x67
        "LD L,B",      // 0x68
        "LD L,C",      // 0x69
        "LD L,D",      // 0x6A
        "LD L,E",      // 0x6B
        "LD L,H",      // 0x6C
        "LD L,L",      // 0x6D
        "LD L,(HL)",   // 0x6E
        "LD L,A",      // 0x6F
        "LD (HL),B",   // 0x70
        "LD (HL),C",   // 0x71
        "LD (HL),D",   // 0x72
        "LD (HL),E",   // 0x73
        "LD (HL),H",   // 0x74
        "LD (HL),L",   // 0x75
        "HALT",        // 0x76
        "LD (HL),A",   // 0x77
        "LD A,B",      // 0x78
        "LD A,C",      // 0x79
        "LD A,D",      // 0x7A
        "LD A,E",      // 0x7B
        "LD A,H",      // 0x7C
        "LD A,L",      // 0x7D
        "LD A,(HL)",   // 0x7E
        "LD A,A",      // 0x7F
        "ADD A,B",     // 0x80
        "ADD A,C",     // 0x81
        "ADD A,D",     // 0x82
        "ADD A,E",     // 0x83
        "ADD A,H",     // 0x84
        "ADD A,L",     // 0x85
        "ADD A,(HL)",  // 0x86
        "ADD A,A",     // 0x87
        "ADC A,B",     // 0x88
        "ADC A,C",     // 0x89
        "ADC A,D",     // 0x8A
        "ADC A,E",     // 0x8B
        "ADC A,H",     // 0x8C
        "ADC A,L",     // 0x8D
        "ADC A,(HL)",  // 0x8E
        "ADC A,A",     // 0x8F
        "SUB B",       // 0x90
        "SUB C",       // 0x91
        "SUB D",       // 0x92
        "SUB E",       // 0x93
        "SUB H",       // 0x94
        "SUB L",       // 0x95
        "SUB (HL)",    // 0x96
        "SUB A",       // 0x97
        "SBC A,B",     // 0x98
        "SBC A,C",     // 0x99
        "SBC A,D",     // 0x9A
        "SBC A,E",     // 0x9B
        "SBC A,H",     // 0x9C
        "SBC A,L",     // 0x9D
        "SBC A,(HL)",  // 0x9E
        "SBC A,A",     // 0x9F
        "AND B",       // 0xA0
        "AND C",       // 0xA1
        "AND D",       // 0xA2
        "AND E",       // 0xA3
        "AND H",       // 0xA4
        "AND L",       // 0xA5
        "AND (HL)",    // 0xA6
        "AND A",       // 0xA7
        "XOR B",       // 0xA8
        "XOR C",       // 0xA9
        "XOR D",       // 0xAA
        "XOR E",       // 0xAB
        "XOR H",       // 0xAC
        "XOR L",       // 0xAD
        "XOR (HL)",    // 0xAE
        "XOR A",       // 0xAF
        "OR B",        // 0xB0
        "OR C",        // 0xB1
        "OR D",        // 0xB2
        "OR E",        // 0xB3
        "OR H",        // 0xB4
        "OR L",        // 0xB5
        "OR (HL)",     // 0xB6
        "OR A",        // 0xB7
        "CP B",        // 0xB8
        "CP C",        // 0xB9
        "CP D",        // 0xBA
        "CP E",        // 0xBB
        "CP H",        // 0xBC
        "CP L",        // 0xBD
        "CP (HL)",     // 0xBE
        "CP A",        // 0xBF
        "RET NZ",      // 0xC0
        "POP BC",      // 0xC1
        "JP NZ,a16",   // 0xC2
        "JP a16",      // 0xC3
        "CALL NZ,a16", // 0xC4
        "PUSH BC",     // 0xC5
        "ADD A,d8",    // 0xC6
        "RST 00H",     // 0xC7
        "RET Z",       // 0xC8
        "RET",         // 0xC9
        "JP Z,a16",    // 0xCA
        "PREFIX CB",   // 0xCB
        "CALL Z,a16",  // 0xCC
        "CALL a16",    // 0xCD
        "ADC A,d8",    // 0xCE
        "RST 08H",     // 0xCF
        "RET NC",      // 0xD0
        "POP DE",      // 0xD1
        "JP NC,a16",   // 0xD2
        "<undefined>", // 0xD3
        "CALL NC,a16", // 0xD4
        "PUSH DE",     // 0xD5
        "SUB d8",      // 0xD6
        "RST 10H",     // 0xD7
        "RET C",       // 0xD8
        "RETI",        // 0xD9
        "JP C,a16",    // 0xDA
        "<undefined>", // 0xDB
        "CALL C,a16",  // 0xDC
        "<undefined>", // 0xDD
        "SBC A,d8",    // 0xDE
        "RST 18H",     // 0xDF
        "LDH (a8),A",  // 0xE0
        "POP HL",      // 0xE1
        "LD (C),A",    // 0xE2
        "<undefined>", // 0xE3
        "<undefined>", // 0xE4
        "PUSH HL",     // 0xE5
        "AND d8",      // 0xE6
        "RST 20H",     // 0xE7
        "ADD SP,r8",   // 0xE8
        "JP (HL)",     // 0xE9
        "LD (a16),A",  // 0xEA
        "<undefined>", // 0xEB
        "<undefined>", // 0xEC
        "<undefined>", // 0xED
        "XOR d8",      // 0xEE
        "RST 28H",     // 0xEF
        "LDH A,(a8)",  // 0xF0
        "POP AF",      // 0xF1
        "LD A,(C)",    // 0xF2
        "DI",          // 0xF3
        "<undefined>", // 0xF4
        "PUSH AF",     // 0xF5
        "OR d8",       // 0xF6
        "RST 30H",     // 0xF7
        "LD HL,SP+r8", // 0xF8
        "LD SP,HL",    // 0xF9
        "LD A,(a16)",  // 0xFA
        "EI",          // 0xFB
        "<undefined>", // 0xFC
        "<undefined>", // 0xFD
        "CP d8",       // 0xFE
        "RST 38H",     // 0xFF
    ];

    pub const CB_MNEMONIC: [&str; 256] = [
        "RLC B",      // 0x00
        "RLC C",      // 0x01
        "RLC D",      // 0x02
        "RLC E",      // 0x03
        "RLC H",      // 0x04
        "RLC L",      // 0x05
        "RLC (HL)",   // 0x06
        "RLC A",      // 0x07
        "RRC B",      // 0x08
        "RRC C",      // 0x09
        "RRC D",      // 0x0A
        "RRC E",      // 0x0B
        "RRC H",      // 0x0C
        "RRC L",      // 0x0D
        "RRC (HL)",   // 0x0E
        "RRC A",      // 0x0F
        "RL B",       // 0x10
        "RL C",       // 0x11
        "RL D",       // 0x12
        "RL E",       // 0x13
        "RL H",       // 0x14
        "RL L",       // 0x15
        "RL (HL)",    // 0x16
        "RL A",       // 0x17
        "RR B",       // 0x18
        "RR C",       // 0x19
        "RR D",       // 0x1A
        "RR E",       // 0x1B
        "RR H",       // 0x1C
        "RR L",       // 0x1D
        "RR (HL)",    // 0x1E
        "RR A",       // 0x1F
        "SLA B",      // 0x20
        "SLA C",      // 0x21
        "SLA D",      // 0x22
        "SLA E",      // 0x23
        "SLA H",      // 0x24
        "SLA L",      // 0x25
        "SLA (HL)",   // 0x26
        "SLA A",      // 0x27
        "SRA B",      // 0x28
        "SRA C",      // 0x29
        "SRA D",      // 0x2A
        "SRA E",      // 0x2B
        "SRA H",      // 0x2C
        "SRA L",      // 0x2D
        "SRA (HL)",   // 0x2E
        "SRA A",      // 0x2F
        "SWAP B",     // 0x30
        "SWAP C",     // 0x31
        "SWAP D",     // 0x32
        "SWAP E",     // 0x33
        "SWAP H",     // 0x34
        "SWAP L",     // 0x35
        "SWAP (HL)",  // 0x36
        "SWAP A",     // 0x37
        "SRL B",      // 0x38
        "SRL C",      // 0x39
        "SRL D",      // 0x3A
        "SRL E",      // 0x3B
        "SRL H",      // 0x3C
        "SRL L",      // 0x3D
        "SRL (HL)",   // 0x3E
        "SRL A",      // 0x3F
        "BIT 0,B",    // 0x40
        "BIT 0,C",    // 0x41
        "BIT 0,D",    // 0x42
        "BIT 0,E",    // 0x43
        "BIT 0,H",    // 0x44
        "BIT 0,L",    // 0x45
        "BIT 0,(HL)", // 0x46
        "BIT 0,A",    // 0x47
        "BIT 1,B",    // 0x48
        "BIT 1,C",    // 0x49
        "BIT 1,D",    // 0x4A
        "BIT 1,E",    // 0x4B
        "BIT 1,H",    // 0x4C
        "BIT 1,L",    // 0x4D
        "BIT 1,(HL)", // 0x4E
        "BIT 1,A",    // 0x4F
        "BIT 2,B",    // 0x50
        "BIT 2,C",    // 0x51
        "BIT 2,D",    // 0x52
        "BIT 2,E",    // 0x53
        "BIT 2,H",    // 0x54
        "BIT 2,L",    // 0x55
        "BIT 2,(HL)", // 0x56
        "BIT 2,A",    // 0x57
        "BIT 3,B",    // 0x58
        "BIT 3,C",    // 0x59
        "BIT 3,D",    // 0x5A
        "BIT 3,E",    // 0x5B
        "BIT 3,H",    // 0x5C
        "BIT 3,L",    // 0x5D
        "BIT 3,(HL)", // 0x5E
        "BIT 3,A",    // 0x5F
        "BIT 4,B",    // 0x60
        "BIT 4,C",    // 0x61
        "BIT 4,D",    // 0x62
        "BIT 4,E",    // 0x63
        "BIT 4,H",    // 0x64
        "BIT 4,L",    // 0x65
        "BIT 4,(HL)", // 0x66
        "BIT 4,A",    // 0x67
        "BIT 5,B",    // 0x68
        "BIT 5,C",    // 0x69
        "BIT 5,D",    // 0x6A
        "BIT 5,E",    // 0x6B
        "BIT 5,H",    // 0x6C
        "BIT 5,L",    // 0x6D
        "BIT 5,(HL)", // 0x6E
        "BIT 5,A",    // 0x6F
        "BIT 6,B",    // 0x70
        "BIT 6,C",    // 0x71
        "BIT 6,D",    // 0x72
        "BIT 6,E",    // 0x73
        "BIT 6,H",    // 0x74
        "BIT 6,L",    // 0x75
        "BIT 6,(HL)", // 0x76
        "BIT 6,A",    // 0x77
        "BIT 7,B",    // 0x78
        "BIT 7,C",    // 0x79
        "BIT 7,D",    // 0x7A
        "BIT 7,E",    // 0x7B
        "BIT 7,H",    // 0x7C
        "BIT 7,L",    // 0x7D
        "BIT 7,(HL)", // 0x7E
        "BIT 7,A",    // 0x7F
        "RES 0,B",    // 0x80
        "RES 0,C",    // 0x81
        "RES 0,D",    // 0x82
        "RES 0,E",    // 0x83
        "RES 0,H",    // 0x84
        "RES 0,L",    // 0x85
        "RES 0,(HL)", // 0x86
        "RES 0,A",    // 0x87
        "RES 1,B",    // 0x88
        "RES 1,C",    // 0x89
        "RES 1,D",    // 0x8A
        "RES 1,E",    // 0x8B
        "RES 1,H",    // 0x8C
        "RES 1,L",    // 0x8D
        "RES 1,(HL)", // 0x8E
        "RES 1,A",    // 0x8F
        "RES 2,B",    // 0x90
        "RES 2,C",    // 0x91
        "RES 2,D",    // 0x92
        "RES 2,E",    // 0x93
        "RES 2,H",    // 0x94
        "RES 2,L",    // 0x95
        "RES 2,(HL)", // 0x96
        "RES 2,A",    // 0x97
        "RES 3,B",    // 0x98
        "RES 3,C",    // 0x99
        "RES 3,D",    // 0x9A
        "RES 3,E",    // 0x9B
        "RES 3,H",    // 0x9C
        "RES 3,L",    // 0x9D
        "RES 3,(HL)", // 0x9E
        "RES 3,A",    // 0x9F
        "RES 4,B",    // 0xA0
        "RES 4,C",    // 0xA1
        "RES 4,D",    // 0xA2
        "RES 4,E",    // 0xA3
        "RES 4,H",    // 0xA4
        "RES 4,L",    // 0xA5
        "RES 4,(HL)", // 0xA6
        "RES 4,A",    // 0xA7
        "RES 5,B",    // 0xA8
        "RES 5,C",    // 0xA9
        "RES 5,D",    // 0xAA
        "RES 5,E",    // 0xAB
        "RES 5,H",    // 0xAC
        "RES 5,L",    // 0xAD
        "RES 5,(HL)", // 0xAE
        "RES 5,A",    // 0xAF
        "RES 6,B",    // 0xB0
        "RES 6,C",    // 0xB1
        "RES 6,D",    // 0xB2
        "RES 6,E",    // 0xB3
        "RES 6,H",    // 0xB4
        "RES 6,L",    // 0xB5
        "RES 6,(HL)", // 0xB6
        "RES 6,A",    // 0xB7
        "RES 7,B",    // 0xB8
        "RES 7,C",    // 0xB9
        "RES 7,D",    // 0xBA
        "RES 7,E",    // 0xBB
        "RES 7,H",    // 0xBC
        "RES 7,L",    // 0xBD
        "RES 7,(HL)", // 0xBE
        "RES 7,A",    // 0xBF
        "SET 0,B",    // 0xC0
        "SET 0,C",    // 0xC1
        "SET 0,D",    // 0xC2
        "SET 0,E",    // 0xC3
        "SET 0,H",    // 0xC4
        "SET 0,L",    // 0xC5
        "SET 0,(HL)", // 0xC6
        "SET 0,A",    // 0xC7
        "SET 1,B",    // 0xC8
        "SET 1,C",    // 0xC9
        "SET 1,D",    // 0xCA
        "SET 1,E",    // 0xCB
        "SET 1,H",    // 0xCC
        "SET 1,L",    // 0xCD
        "SET 1,(HL)", // 0xCE
        "SET 1,A",    // 0xCF
        "SET 2,B",    // 0xD0
        "SET 2,C",    // 0xD1
        "SET 2,D",    // 0xD2
        "SET 2,E",    // 0xD3
        "SET 2,H",    // 0xD4
        "SET 2,L",    // 0xD5
        "SET 2,(HL)", // 0xD6
        "SET 2,A",    // 0xD7
        "SET 3,B",    // 0xD8
        "SET 3,C",    // 0xD9
        "SET 3,D",    // 0xDA
        "SET 3,E",    // 0xDB
        "SET 3,H",    // 0xDC
        "SET 3,L",    // 0xDD
        "SET 3,(HL)", // 0xDE
        "SET 3,A",    // 0xDF
        "SET 4,B",    // 0xE0
        "SET 4,C",    // 0xE1
        "SET 4,D",    // 0xE2
        "SET 4,E",    // 0xE3
        "SET 4,H",    // 0xE4
        "SET 4,L",    // 0xE5
        "SET 4,(HL)", // 0xE6
        "SET 4,A",    // 0xE7
        "SET 5,B",    // 0xE8
        "SET 5,C",    // 0xE9
        "SET 5,D",    // 0xEA
        "SET 5,E",    // 0xEB
        "SET 5,H",    // 0xEC
        "SET 5,L",    // 0xED
        "SET 5,(HL)", // 0xEE
        "SET 5,A",    // 0xEF
        "SET 6,B",    // 0xF0
        "SET 6,C",    // 0xF1
        "SET 6,D",    // 0xF2
        "SET 6,E",    // 0xF3
        "SET 6,H",    // 0xF4
        "SET 6,L",    // 0xF5
        "SET 6,(HL)", // 0xF6
        "SET 6,A",    // 0xF7
        "SET 7,B",    // 0xF8
        "SET 7,C",    // 0xF9
        "SET 7,D",    // 0xFA
        "SET 7,E",    // 0xFB
        "SET 7,H",    // 0xFC
        "SET 7,L",    // 0xFD
        "SET 7,(HL)", // 0xFE
        "SET 7,A",    // 0xFF
    ];
}

pub use table::*;
