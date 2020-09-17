/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use super::super::*;
use super::op_code::OpCodeInfo;
#[cfg(not(feature = "std"))]
use alloc::string::String;
use core::fmt::Write;
use core::{fmt, u16, u8};

// GENERATOR-BEGIN: LKind
// ⚠️This was generated by GENERATOR!🦹‍♂️
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub(crate) enum LKind {
	None,
	/// .128, .256, .512
	L128,
	/// .L0, .L1
	L0,
	/// .LZ
	LZ,
}
#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
static GEN_DEBUG_LKIND: [&str; 4] = [
	"None",
	"L128",
	"L0",
	"LZ",
];
impl fmt::Debug for LKind {
	#[inline]
	fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
		write!(f, "{}", GEN_DEBUG_LKIND[*self as usize])?;
		Ok(())
	}
}
impl Default for LKind {
	#[cfg_attr(has_must_use, must_use)]
	#[inline]
	fn default() -> Self {
		LKind::None
	}
}
// GENERATOR-END: LKind

pub(super) struct OpCodeFormatter<'a, 'b> {
	op_code: &'a OpCodeInfo,
	sb: &'b mut String,
	#[allow(dead_code)]
	lkind: LKind,
}

impl<'a, 'b> OpCodeFormatter<'a, 'b> {
	pub(super) fn new(op_code: &'a OpCodeInfo, sb: &'b mut String, lkind: LKind) -> OpCodeFormatter<'a, 'b> {
		Self { op_code, sb, lkind }
	}

	pub(super) fn format(&mut self) -> String {
		if !self.op_code.is_instruction() {
			match self.op_code.code() {
				// GENERATOR-BEGIN: OpCodeFmtNotInstructionString
				// ⚠️This was generated by GENERATOR!🦹‍♂️
				Code::INVALID => String::from("<invalid>"),
				Code::DeclareByte => String::from("<db>"),
				Code::DeclareWord => String::from("<dw>"),
				Code::DeclareDword => String::from("<dd>"),
				Code::DeclareQword => String::from("<dq>"),
				// GENERATOR-END: OpCodeFmtNotInstructionString
				_ => unreachable!(),
			}
		} else {
			match self.op_code.encoding() {
				EncodingKind::Legacy => self.format_legacy(),
				#[cfg(not(feature = "no_vex"))]
				EncodingKind::VEX => self.format_vex_xop_evex("VEX"),
				#[cfg(not(feature = "no_evex"))]
				EncodingKind::EVEX => self.format_vex_xop_evex("EVEX"),
				#[cfg(not(feature = "no_xop"))]
				EncodingKind::XOP => self.format_vex_xop_evex("XOP"),
				#[cfg(not(feature = "no_d3now"))]
				EncodingKind::D3NOW => self.format_3dnow(),
				#[cfg(feature = "no_vex")]
				EncodingKind::VEX => String::new(),
				#[cfg(feature = "no_evex")]
				EncodingKind::EVEX => String::new(),
				#[cfg(feature = "no_xop")]
				EncodingKind::XOP => String::new(),
				#[cfg(feature = "no_d3now")]
				EncodingKind::D3NOW => String::new(),
			}
		}
	}

	fn append_hex_byte(&mut self, value: u8) {
		write!(self.sb, "{:02X}", value).unwrap();
	}

	fn append_op_code(&mut self, value: u32, sep: bool) {
		if value <= u8::MAX as u32 {
			self.append_hex_byte(value as u8);
		} else if value <= u16::MAX as u32 {
			self.append_hex_byte((value >> 8) as u8);
			if sep {
				self.sb.push(' ');
			}
			self.append_hex_byte(value as u8);
		} else {
			unreachable!();
		}
	}

	fn append_table(&mut self, sep: bool) {
		match self.op_code.table() {
			OpCodeTableKind::Normal => {}
			OpCodeTableKind::T0F => self.append_op_code(0x0F, sep),
			OpCodeTableKind::T0F38 => self.append_op_code(0x0F38, sep),
			OpCodeTableKind::T0F3A => self.append_op_code(0x0F3A, sep),
			OpCodeTableKind::XOP8 => self.sb.push_str("X8"),
			OpCodeTableKind::XOP9 => self.sb.push_str("X9"),
			OpCodeTableKind::XOPA => self.sb.push_str("XA"),
		}
	}

	fn has_mod_rm(&self) -> bool {
		let op_count = self.op_code.op_count();
		if op_count == 0 {
			return false;
		}

		match self.op_code.encoding() {
			EncodingKind::Legacy => {}
			EncodingKind::VEX | EncodingKind::EVEX | EncodingKind::XOP | EncodingKind::D3NOW => return true,
		}

		for i in 0..op_count {
			#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
			match self.op_code.op_kind(i) {
				// GENERATOR-BEGIN: HasModRM
				// ⚠️This was generated by GENERATOR!🦹‍♂️
				OpCodeOperandKind::mem
				| OpCodeOperandKind::mem_mpx
				| OpCodeOperandKind::mem_mib
				| OpCodeOperandKind::mem_vsib32x
				| OpCodeOperandKind::mem_vsib64x
				| OpCodeOperandKind::mem_vsib32y
				| OpCodeOperandKind::mem_vsib64y
				| OpCodeOperandKind::mem_vsib32z
				| OpCodeOperandKind::mem_vsib64z
				| OpCodeOperandKind::r8_or_mem
				| OpCodeOperandKind::r16_or_mem
				| OpCodeOperandKind::r32_or_mem
				| OpCodeOperandKind::r32_or_mem_mpx
				| OpCodeOperandKind::r64_or_mem
				| OpCodeOperandKind::r64_or_mem_mpx
				| OpCodeOperandKind::mm_or_mem
				| OpCodeOperandKind::xmm_or_mem
				| OpCodeOperandKind::ymm_or_mem
				| OpCodeOperandKind::zmm_or_mem
				| OpCodeOperandKind::bnd_or_mem_mpx
				| OpCodeOperandKind::k_or_mem
				| OpCodeOperandKind::r8_reg
				| OpCodeOperandKind::r16_reg
				| OpCodeOperandKind::r16_reg_mem
				| OpCodeOperandKind::r16_rm
				| OpCodeOperandKind::r32_reg
				| OpCodeOperandKind::r32_reg_mem
				| OpCodeOperandKind::r32_rm
				| OpCodeOperandKind::r64_reg
				| OpCodeOperandKind::r64_reg_mem
				| OpCodeOperandKind::r64_rm
				| OpCodeOperandKind::seg_reg
				| OpCodeOperandKind::k_reg
				| OpCodeOperandKind::kp1_reg
				| OpCodeOperandKind::k_rm
				| OpCodeOperandKind::mm_reg
				| OpCodeOperandKind::mm_rm
				| OpCodeOperandKind::xmm_reg
				| OpCodeOperandKind::xmm_rm
				| OpCodeOperandKind::ymm_reg
				| OpCodeOperandKind::ymm_rm
				| OpCodeOperandKind::zmm_reg
				| OpCodeOperandKind::zmm_rm
				| OpCodeOperandKind::cr_reg
				| OpCodeOperandKind::dr_reg
				| OpCodeOperandKind::tr_reg
				| OpCodeOperandKind::bnd_reg
				| OpCodeOperandKind::sibmem
				| OpCodeOperandKind::tmm_reg
				| OpCodeOperandKind::tmm_rm
				=> return true,
				// GENERATOR-END: HasModRM

				_ => {}
			}
		}
		false
	}

	fn has_vsib(&self) -> bool {
		for i in 0..self.op_code.op_count() {
			#[cfg_attr(feature = "cargo-fmt", rustfmt::skip)]
			match self.op_code.op_kind(i) {
				// GENERATOR-BEGIN: HasVsib
				// ⚠️This was generated by GENERATOR!🦹‍♂️
				OpCodeOperandKind::mem_vsib32x
				| OpCodeOperandKind::mem_vsib64x
				| OpCodeOperandKind::mem_vsib32y
				| OpCodeOperandKind::mem_vsib64y
				| OpCodeOperandKind::mem_vsib32z
				| OpCodeOperandKind::mem_vsib64z
				=> return true,
				// GENERATOR-END: HasVsib

				_ => {}
			}
		}
		false
	}

	fn get_op_code_bits_operand(&self) -> OpCodeOperandKind {
		for i in 0..self.op_code.op_count() {
			let op_kind = self.op_code.op_kind(i);
			match op_kind {
				OpCodeOperandKind::r8_opcode | OpCodeOperandKind::r16_opcode | OpCodeOperandKind::r32_opcode | OpCodeOperandKind::r64_opcode => {
					return op_kind
				}
				_ => {}
			}
		}
		OpCodeOperandKind::None
	}

	fn get_modrm_info(&self) -> Option<(bool, i32, i32)> {
		let mut is_reg_only = true;
		let rrr = self.op_code.group_index();
		let mut bbb = self.op_code.rm_group_index();
		let mut has_modrm_info = bbb >= 0;
		match self.op_code.code() {
			#[cfg(not(feature = "no_vex"))]
			Code::VEX_Ldtilecfg_m512 | Code::VEX_Sttilecfg_m512 => has_modrm_info = true,
			Code::Aesencwide128kl_m384
			| Code::Aesdecwide128kl_m384
			| Code::Aesencwide256kl_m512
			| Code::Aesdecwide256kl_m512
			| Code::Loadiwkey_xmm_xmm
			| Code::Aesenc128kl_xmm_m384
			| Code::Aesdec128kl_xmm_m384
			| Code::Aesenc256kl_xmm_m512
			| Code::Aesdec256kl_xmm_m512
			| Code::Encodekey128_r32_r32
			| Code::Encodekey256_r32_r32 => has_modrm_info = true,
			_ => {}
		}
		for i in 0..self.op_code.op_count() {
			match self.op_code.op_kind(i) {
				OpCodeOperandKind::mem => is_reg_only = false,
				OpCodeOperandKind::sibmem => {
					has_modrm_info = true;
					is_reg_only = false;
					bbb = 4;
				}
				OpCodeOperandKind::r32_reg | OpCodeOperandKind::xmm_reg => is_reg_only = true,
				OpCodeOperandKind::tmm_reg | OpCodeOperandKind::tmm_rm | OpCodeOperandKind::tmm_vvvv => has_modrm_info = true,
				_ => {}
			}
		}
		if has_modrm_info {
			Some((is_reg_only, rrr, bbb))
		} else {
			None
		}
	}

	fn append_bits(&mut self, name: &str, bits: i32, num_bits: u32) {
		if bits < 0 {
			self.sb.push_str(name);
		} else {
			for bit in (0..num_bits).rev() {
				self.sb.push(if ((bits >> bit) & 1) != 0 { '1' } else { '0' });
			}
		}
	}

	fn append_rest(&mut self) {
		if let Some((is_reg_only, rrr, bbb)) = self.get_modrm_info() {
			self.sb.push_str(if is_reg_only { " 11:" } else { " !(11):" });
			self.append_bits("rrr", rrr, 3);
			self.sb.push_str(":");
			self.append_bits("bbb", bbb, 3);
		} else {
			let is_vsib = self.op_code.encoding() == EncodingKind::EVEX && self.has_vsib();
			if self.op_code.is_group() {
				self.sb.push_str(" /");
				write!(self.sb, "{}", self.op_code.group_index()).unwrap();
			} else if !is_vsib && self.has_mod_rm() {
				self.sb.push_str(" /r");
			}
			if is_vsib {
				self.sb.push_str(" /vsib");
			}
		}

		for i in 0..self.op_code.op_count() {
			match self.op_code.op_kind(i) {
				OpCodeOperandKind::br16_1 | OpCodeOperandKind::br32_1 | OpCodeOperandKind::br64_1 => self.sb.push_str(" cb"),
				OpCodeOperandKind::br16_2 | OpCodeOperandKind::xbegin_2 | OpCodeOperandKind::brdisp_2 => self.sb.push_str(" cw"),

				OpCodeOperandKind::farbr2_2
				| OpCodeOperandKind::br32_4
				| OpCodeOperandKind::br64_4
				| OpCodeOperandKind::xbegin_4
				| OpCodeOperandKind::brdisp_4 => self.sb.push_str(" cd"),

				OpCodeOperandKind::farbr4_2 => self.sb.push_str(" cp"),

				OpCodeOperandKind::imm8 | OpCodeOperandKind::imm8sex16 | OpCodeOperandKind::imm8sex32 | OpCodeOperandKind::imm8sex64 => {
					self.sb.push_str(" ib")
				}

				OpCodeOperandKind::imm16 => self.sb.push_str(" iw"),
				OpCodeOperandKind::imm32 | OpCodeOperandKind::imm32sex64 => self.sb.push_str(" id"),
				OpCodeOperandKind::imm64 => self.sb.push_str(" io"),
				OpCodeOperandKind::xmm_is4 | OpCodeOperandKind::ymm_is4 => self.sb.push_str(" /is4"),

				OpCodeOperandKind::xmm_is5 | OpCodeOperandKind::ymm_is5 => {
					self.sb.push_str(" /is5");
					// don't show the next imm8
					break;
				}

				OpCodeOperandKind::mem_offs => self.sb.push_str(" mo"),
				_ => {}
			}
		}
	}

	fn format_legacy(&mut self) -> String {
		self.sb.clear();

		if self.op_code.fwait() {
			self.append_hex_byte(0x9B);
			self.sb.push(' ');
		}

		match self.op_code.address_size() {
			0 => {}
			16 => self.sb.push_str("a16 "),
			32 => self.sb.push_str("a32 "),
			64 => self.sb.push_str("a64 "),
			_ => unreachable!(),
		}

		match self.op_code.operand_size() {
			0 => {}
			16 => self.sb.push_str("o16 "),
			32 => self.sb.push_str("o32 "),
			64 => {} // REX.W must be immediately before the opcode and is handled below
			_ => unreachable!(),
		}

		match self.op_code.mandatory_prefix() {
			MandatoryPrefix::None => {}
			MandatoryPrefix::PNP => self.sb.push_str("NP "),
			MandatoryPrefix::P66 => {
				self.append_hex_byte(0x66);
				self.sb.push(' ');
			}
			MandatoryPrefix::PF3 => {
				self.append_hex_byte(0xF3);
				self.sb.push(' ');
			}
			MandatoryPrefix::PF2 => {
				self.append_hex_byte(0xF2);
				self.sb.push(' ');
			}
		}

		if self.op_code.operand_size() == 64 {
			// There's no '+' because Intel isn't consistent, some opcodes use it others don't
			self.sb.push_str("REX.W ");
		}

		self.append_table(true);
		if self.op_code.table() != OpCodeTableKind::Normal {
			self.sb.push(' ');
		}
		self.append_op_code(self.op_code.op_code(), true);
		match self.get_op_code_bits_operand() {
			OpCodeOperandKind::r8_opcode => self.sb.push_str("+rb"),
			OpCodeOperandKind::r16_opcode => self.sb.push_str("+rw"),
			OpCodeOperandKind::r32_opcode => self.sb.push_str("+rd"),
			OpCodeOperandKind::r64_opcode => self.sb.push_str("+ro"),
			OpCodeOperandKind::None => {}
			_ => unreachable!(),
		}
		for i in 0..self.op_code.op_count() {
			if self.op_code.op_kind(i) == OpCodeOperandKind::sti_opcode {
				self.sb.push_str("+i");
				break;
			}
		}

		self.append_rest();

		self.sb.clone()
	}

	#[cfg(not(feature = "no_d3now"))]
	fn format_3dnow(&mut self) -> String {
		self.sb.clear();

		self.append_op_code(0x0F0F, true);
		self.sb.push_str(" /r");
		self.sb.push(' ');
		self.append_op_code(self.op_code.op_code(), true);

		self.sb.clone()
	}

	#[cfg(any(not(feature = "no_vex"), not(feature = "no_xop"), not(feature = "no_evex")))]
	fn format_vex_xop_evex(&mut self, encoding_name: &str) -> String {
		self.sb.clear();

		self.sb.push_str(encoding_name);
		self.sb.push('.');
		if self.op_code.is_lig() {
			self.sb.push_str("LIG");
		} else {
			match self.lkind {
				LKind::L128 => write!(self.sb, "{}", 128 << self.op_code.l()).unwrap(),
				LKind::L0 => {
					self.sb.push('L');
					write!(self.sb, "{}", self.op_code.l()).unwrap();
				}
				LKind::LZ => {
					if self.op_code.l() != 0 {
						unreachable!();
					}
					self.sb.push_str("LZ");
				}
				LKind::None => unreachable!(),
			}
		}
		match self.op_code.mandatory_prefix() {
			MandatoryPrefix::None | MandatoryPrefix::PNP => {}
			MandatoryPrefix::P66 => {
				self.sb.push('.');
				self.append_hex_byte(0x66);
			}
			MandatoryPrefix::PF3 => {
				self.sb.push('.');
				self.append_hex_byte(0xF3);
			}
			MandatoryPrefix::PF2 => {
				self.sb.push('.');
				self.append_hex_byte(0xF2);
			}
		}
		self.sb.push('.');
		self.append_table(false);
		if self.op_code.is_wig() {
			self.sb.push_str(".WIG");
		} else {
			self.sb.push_str(".W");
			write!(self.sb, "{}", self.op_code.w()).unwrap();
		}
		self.sb.push(' ');
		self.append_op_code(self.op_code.op_code(), true);
		self.append_rest();

		self.sb.clone()
	}
}
