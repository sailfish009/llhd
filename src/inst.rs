// Copyright (c) 2017 Fabian Schuiki
#![allow(dead_code)]

// use std::collections::HashMap;
use ty::*;
use value::*;
use unit::UnitContext;
pub use self::InstKind::*;

pub struct Inst {
	id: InstRef,
	/// An optional name for the instruction used when emitting assembly.
	name: Option<String>,
	/// The instruction data.
	kind: InstKind,
}

impl Inst {
	/// Create a new instruction.
	pub fn new(name: Option<String>, kind: InstKind) -> Inst {
		Inst {
			id: InstRef(ValueId::alloc()),
			name: name,
			kind: kind,
		}
	}

	/// Obtain a reference to this instruction.
	pub fn as_ref(&self) -> InstRef {
		self.id
	}

	/// Determine the mnemonic for this instruction. The mnemonic is a short
	/// sequence of characters that uniquely identifies the instruction in human
	/// readable assembly text.
	pub fn mnemonic(&self) -> Mnemonic {
		self.kind.mnemonic()
	}

	/// Obtain a reference to the data for this instruction. See `InstKind`.
	pub fn kind(&self) -> &InstKind {
		&self.kind
	}
}

impl Value for Inst {
	fn id(&self) -> ValueId {
		self.id.into()
	}

	fn ty(&self) -> Type {
		self.kind.ty()
	}

	fn name(&self) -> Option<&str> {
		self.name.as_ref().map(|x| x as &str)
	}

	fn is_global(&self) -> bool {
		false
	}
}

declare_ref!(InstRef, Inst);


pub struct InstIter<'tf> {
	refs: std::slice::Iter<'tf, InstRef>,
	ctx: &'tf UnitContext,
}

impl<'tf> InstIter<'tf> {
	pub fn new(refs: std::slice::Iter<'tf, InstRef>, ctx: &'tf UnitContext) -> InstIter<'tf> {
		InstIter {
			refs: refs,
			ctx: ctx,
		}
	}
}

impl<'tf> std::iter::Iterator for InstIter<'tf> {
	type Item = &'tf Inst;

	fn next(&mut self) -> Option<&'tf Inst> {
		let n = self.refs.next();
		n.map(|r| self.ctx.inst(*r))
	}
}


pub enum InstKind {
	BinaryInst(BinaryOp, Type, ValueRef, ValueRef),
}

impl InstKind {
	/// Get the result type of the instruction.
	pub fn ty(&self) -> Type {
		match *self {
			BinaryInst(_, ref ty, _, _) => ty.clone(),
		}
	}

	pub fn mnemonic(&self) -> Mnemonic {
		match *self {
			BinaryInst(op, _, _, _) => Mnemonic::Binary(op.mnemonic()),
		}
	}
}


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BinaryOp {
	Add,
}

impl BinaryOp {
	pub fn mnemonic(&self) -> BinaryMnemonic {
		match *self {
			BinaryOp::Add => BinaryMnemonic::Add,
		}
	}
}


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mnemonic {
	Binary(BinaryMnemonic),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BinaryMnemonic {
	Add,
}

impl Mnemonic {
	/// Convert the mnemonic to its textual representation.
	pub fn as_str(self) -> &'static str {
		match self {
			Mnemonic::Binary(m) => m.as_str(),
		}
	}
}

impl BinaryMnemonic {
	/// Convert the binary mnemonic to its textual representation.
	pub fn as_str(self) -> &'static str {
		match self {
			BinaryMnemonic::Add => "add",
		}
	}
}
