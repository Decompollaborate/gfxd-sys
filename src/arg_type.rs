/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

//! Argument types

/// generic word
pub const gfxd_Word: ArgType = ArgType::gfxd_Word;
/// command opcode (G_*)
pub const gfxd_Opcode: ArgType = ArgType::gfxd_Opcode;
/// integer coordinate
pub const gfxd_Coordi: ArgType = ArgType::gfxd_Coordi;
/// fractional (q10.2) coordinate
pub const gfxd_Coordq: ArgType = ArgType::gfxd_Coordq;
/// palette index
pub const gfxd_Pal: ArgType = ArgType::gfxd_Pal;
/// tlut pointer
pub const gfxd_Tlut: ArgType = ArgType::gfxd_Tlut;
/// texture image pointer
pub const gfxd_Timg: ArgType = ArgType::gfxd_Timg;
/// tmem address
pub const gfxd_Tmem: ArgType = ArgType::gfxd_Tmem;
/// tile index
pub const gfxd_Tile: ArgType = ArgType::gfxd_Tile;
/// texture format
pub const gfxd_Fmt: ArgType = ArgType::gfxd_Fmt;
/// texture pixel size
pub const gfxd_Siz: ArgType = ArgType::gfxd_Siz;
/// integer dimension (width / height)
pub const gfxd_Dim: ArgType = ArgType::gfxd_Dim;
/// clamp and mirror flags
pub const gfxd_Cm: ArgType = ArgType::gfxd_Cm;
/// tile mask
pub const gfxd_Tm: ArgType = ArgType::gfxd_Tm;
/// tile shift
pub const gfxd_Ts: ArgType = ArgType::gfxd_Ts;
/// texture dxt
pub const gfxd_Dxt: ArgType = ArgType::gfxd_Dxt;
/// generic tag
pub const gfxd_Tag: ArgType = ArgType::gfxd_Tag;
/// pipeline mode
pub const gfxd_Pm: ArgType = ArgType::gfxd_Pm;
/// color component
pub const gfxd_Colorpart: ArgType = ArgType::gfxd_Colorpart;
/// color
pub const gfxd_Color: ArgType = ArgType::gfxd_Color;
/// lod fraction (q0.8)
pub const gfxd_Lodfrac: ArgType = ArgType::gfxd_Lodfrac;
/// color image pointer
pub const gfxd_Cimg: ArgType = ArgType::gfxd_Cimg;
/// depth image pointer
pub const gfxd_Zimg: ArgType = ArgType::gfxd_Zimg;
/// alpha compare mode
pub const gfxd_Ac: ArgType = ArgType::gfxd_Ac;
/// alpha dither mode
pub const gfxd_Ad: ArgType = ArgType::gfxd_Ad;
/// color dither mode
pub const gfxd_Cd: ArgType = ArgType::gfxd_Cd;
/// color combiner preset index
pub const gfxd_Ccpre: ArgType = ArgType::gfxd_Ccpre;
/// color mux operand (a)
pub const gfxd_Ccmuxa: ArgType = ArgType::gfxd_Ccmuxa;
/// color mux operand (b)
pub const gfxd_Ccmuxb: ArgType = ArgType::gfxd_Ccmuxb;
/// color mux operand (c)
pub const gfxd_Ccmuxc: ArgType = ArgType::gfxd_Ccmuxc;
/// color mux operand (d)
pub const gfxd_Ccmuxd: ArgType = ArgType::gfxd_Ccmuxd;
/// alpha mux operand (a, b, or d)
pub const gfxd_Acmuxabd: ArgType = ArgType::gfxd_Acmuxabd;
/// alpha mux operand (c)
pub const gfxd_Acmuxc: ArgType = ArgType::gfxd_Acmuxc;
/// color convert operand
pub const gfxd_Cv: ArgType = ArgType::gfxd_Cv;
/// texture convert mode
pub const gfxd_Tc: ArgType = ArgType::gfxd_Tc;
/// cycle type
pub const gfxd_Cyc: ArgType = ArgType::gfxd_Cyc;
/// depth source mode
pub const gfxd_Zs: ArgType = ArgType::gfxd_Zs;
/// combine key mode
pub const gfxd_Ck: ArgType = ArgType::gfxd_Ck;
/// combine key scale
pub const gfxd_Keyscale: ArgType = ArgType::gfxd_Keyscale;
/// combine key width
pub const gfxd_Keywidth: ArgType = ArgType::gfxd_Keywidth;
/// integer depth
pub const gfxd_Zi: ArgType = ArgType::gfxd_Zi;
/// cycle 1 render mode
pub const gfxd_Rm1: ArgType = ArgType::gfxd_Rm1;
/// cycle 2 render mode
pub const gfxd_Rm2: ArgType = ArgType::gfxd_Rm2;
/// scissor mode
pub const gfxd_Sc: ArgType = ArgType::gfxd_Sc;
/// texture detail mode
pub const gfxd_Td: ArgType = ArgType::gfxd_Td;
/// texture filter mode
pub const gfxd_Tf: ArgType = ArgType::gfxd_Tf;
/// texture LOD mode
pub const gfxd_Tl: ArgType = ArgType::gfxd_Tl;
/// textuure LUT mode
pub const gfxd_Tt: ArgType = ArgType::gfxd_Tt;
/// texture perspective mode
pub const gfxd_Tp: ArgType = ArgType::gfxd_Tp;
/// texture line size
pub const gfxd_Line: ArgType = ArgType::gfxd_Line;
/// vertex index
pub const gfxd_Vtx: ArgType = ArgType::gfxd_Vtx;
/// vertex flag
pub const gfxd_Vtxflag: ArgType = ArgType::gfxd_Vtxflag;
/// display list pointer
pub const gfxd_Dl: ArgType = ArgType::gfxd_Dl;
/// raw depth value (q16.16)
pub const gfxd_Zraw: ArgType = ArgType::gfxd_Zraw;
/// display list flag
pub const gfxd_Dlflag: ArgType = ArgType::gfxd_Dlflag;
/// clip ratio
pub const gfxd_Cr: ArgType = ArgType::gfxd_Cr;
/// element count
pub const gfxd_Num: ArgType = ArgType::gfxd_Num;
/// fog factor
pub const gfxd_Fogz: ArgType = ArgType::gfxd_Fogz;
/// fog position (0 - 1000)
pub const gfxd_Fogp: ArgType = ArgType::gfxd_Fogp;
/// matrix pointer
pub const gfxd_Mtxptr: ArgType = ArgType::gfxd_Mtxptr;
/// geometry mode
pub const gfxd_Gm: ArgType = ArgType::gfxd_Gm;
/// matrix moveword offset
pub const gfxd_Mwo_matrix: ArgType = ArgType::gfxd_Mwo_matrix;
/// line width (1.5 + q7.1)
pub const gfxd_Linewd: ArgType = ArgType::gfxd_Linewd;
/// microcode text pointer
pub const gfxd_Uctext: ArgType = ArgType::gfxd_Uctext;
/// microcode data pointer
pub const gfxd_Ucdata: ArgType = ArgType::gfxd_Ucdata;
/// data size
pub const gfxd_Size: ArgType = ArgType::gfxd_Size;
/// lookat pointer
pub const gfxd_Lookatptr: ArgType = ArgType::gfxd_Lookatptr;
/// matrix param
pub const gfxd_Mtxparam: ArgType = ArgType::gfxd_Mtxparam;
/// matrix param (stack select only)
pub const gfxd_Mtxstack: ArgType = ArgType::gfxd_Mtxstack;
/// vertex moveword offset
pub const gfxd_Mwo_point: ArgType = ArgType::gfxd_Mwo_point;
/// w-component scale (perspnorm)
pub const gfxd_Wscale: ArgType = ArgType::gfxd_Wscale;
/// segment number
pub const gfxd_Seg: ArgType = ArgType::gfxd_Seg;
/// segment pointer
pub const gfxd_Segptr: ArgType = ArgType::gfxd_Segptr;
/// dereferenced LightsM (0-7 or n) pointer
pub const gfxd_Lightsn: ArgType = ArgType::gfxd_Lightsn;
/// light count (NUMLIGHTS_*)
pub const gfxd_Numlights: ArgType = ArgType::gfxd_Numlights;
/// light number (LIGHT_*)
pub const gfxd_Lightnum: ArgType = ArgType::gfxd_Lightnum;
/// diffuse or ambient light pointer
pub const gfxd_Lightptr: ArgType = ArgType::gfxd_Lightptr;
/// texture coordinate scale
pub const gfxd_Tcscale: ArgType = ArgType::gfxd_Tcscale;
/// on-off value
pub const gfxd_Switch: ArgType = ArgType::gfxd_Switch;
/// vertex coordinate (q10.5)
pub const gfxd_St: ArgType = ArgType::gfxd_St;
/// vertex coordinate delta (q5.10)
pub const gfxd_Stdelta: ArgType = ArgType::gfxd_Stdelta;
/// vertex pointer
pub const gfxd_Vtxptr: ArgType = ArgType::gfxd_Vtxptr;
/// viewport pointer
pub const gfxd_Vpptr: ArgType = ArgType::gfxd_Vpptr;
/// generic dram address
pub const gfxd_Dram: ArgType = ArgType::gfxd_Dram;
/// othermode lo shift
pub const gfxd_Sftlo: ArgType = ArgType::gfxd_Sftlo;
/// othermode lo value
pub const gfxd_Othermodelo: ArgType = ArgType::gfxd_Othermodelo;
/// othermode hi shift
pub const gfxd_Sfthi: ArgType = ArgType::gfxd_Sfthi;
/// othermode hi value
pub const gfxd_Othermodehi: ArgType = ArgType::gfxd_Othermodehi;
/// moveword index
pub const gfxd_Mw: ArgType = ArgType::gfxd_Mw;
/// moveword offset
pub const gfxd_Mwo: ArgType = ArgType::gfxd_Mwo;
/// clip ratio moveword offset
pub const gfxd_Mwo_clip: ArgType = ArgType::gfxd_Mwo_clip;
/// light color moveword offset
pub const gfxd_Mwo_lightcol: ArgType = ArgType::gfxd_Mwo_lightcol;
/// movemem index
pub const gfxd_Mv: ArgType = ArgType::gfxd_Mv;
/// movemem offset
pub const gfxd_Mvo: ArgType = ArgType::gfxd_Mvo;
/// dmem address
pub const gfxd_Dmem: ArgType = ArgType::gfxd_Dmem;
/// dma io flag
pub const gfxd_Dmaflag: ArgType = ArgType::gfxd_Dmaflag;

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ArgType {
    /// generic word
    gfxd_Word = 0,
    /// command opcode (G_*)
    gfxd_Opcode = 1,
    /// integer coordinate
    gfxd_Coordi = 2,
    /// fractional (q10.2) coordinate
    gfxd_Coordq = 3,
    /// palette index
    gfxd_Pal = 4,
    /// tlut pointer
    gfxd_Tlut = 5,
    /// texture image pointer
    gfxd_Timg = 6,
    /// tmem address
    gfxd_Tmem = 7,
    /// tile index
    gfxd_Tile = 8,
    /// texture format
    gfxd_Fmt = 9,
    /// texture pixel size
    gfxd_Siz = 10,
    /// integer dimension (width / height)
    gfxd_Dim = 11,
    /// clamp and mirror flags
    gfxd_Cm = 12,
    /// tile mask
    gfxd_Tm = 13,
    /// tile shift
    gfxd_Ts = 14,
    /// texture dxt
    gfxd_Dxt = 15,
    /// generic tag
    gfxd_Tag = 16,
    /// pipeline mode
    gfxd_Pm = 17,
    /// color component
    gfxd_Colorpart = 18,
    /// color
    gfxd_Color = 19,
    /// lod fraction (q0.8)
    gfxd_Lodfrac = 20,
    /// color image pointer
    gfxd_Cimg = 21,
    /// depth image pointer
    gfxd_Zimg = 22,
    /// alpha compare mode
    gfxd_Ac = 23,
    /// alpha dither mode
    gfxd_Ad = 24,
    /// color dither mode
    gfxd_Cd = 25,
    /// color combiner preset index
    gfxd_Ccpre = 26,
    /// color mux operand (a)
    gfxd_Ccmuxa = 27,
    /// color mux operand (b)
    gfxd_Ccmuxb = 28,
    /// color mux operand (c)
    gfxd_Ccmuxc = 29,
    /// color mux operand (d)
    gfxd_Ccmuxd = 30,
    /// alpha mux operand (a, b, or d)
    gfxd_Acmuxabd = 31,
    /// alpha mux operand (c)
    gfxd_Acmuxc = 32,
    /// color convert operand
    gfxd_Cv = 33,
    /// texture convert mode
    gfxd_Tc = 34,
    /// cycle type
    gfxd_Cyc = 35,
    /// depth source mode
    gfxd_Zs = 36,
    /// combine key mode
    gfxd_Ck = 37,
    /// combine key scale
    gfxd_Keyscale = 38,
    /// combine key width
    gfxd_Keywidth = 39,
    /// integer depth
    gfxd_Zi = 40,
    /// cycle 1 render mode
    gfxd_Rm1 = 41,
    /// cycle 2 render mode
    gfxd_Rm2 = 42,
    /// scissor mode
    gfxd_Sc = 43,
    /// texture detail mode
    gfxd_Td = 44,
    /// texture filter mode
    gfxd_Tf = 45,
    /// texture LOD mode
    gfxd_Tl = 46,
    /// textuure LUT mode
    gfxd_Tt = 47,
    /// texture perspective mode
    gfxd_Tp = 48,
    /// texture line size
    gfxd_Line = 49,
    /// vertex index
    gfxd_Vtx = 50,
    /// vertex flag
    gfxd_Vtxflag = 51,
    /// display list pointer
    gfxd_Dl = 52,
    /// raw depth value (q16.16)
    gfxd_Zraw = 53,
    /// display list flag
    gfxd_Dlflag = 54,
    /// clip ratio
    gfxd_Cr = 55,
    /// element count
    gfxd_Num = 56,
    /// fog factor
    gfxd_Fogz = 57,
    /// fog position (0 - 1000)
    gfxd_Fogp = 58,
    /// matrix pointer
    gfxd_Mtxptr = 59,
    /// geometry mode
    gfxd_Gm = 60,
    /// matrix moveword offset
    gfxd_Mwo_matrix = 61,
    /// line width (1.5 + q7.1)
    gfxd_Linewd = 62,
    /// microcode text pointer
    gfxd_Uctext = 63,
    /// microcode data pointer
    gfxd_Ucdata = 64,
    /// data size
    gfxd_Size = 65,
    /// lookat pointer
    gfxd_Lookatptr = 66,
    /// matrix param
    gfxd_Mtxparam = 67,
    /// matrix param (stack select only)
    gfxd_Mtxstack = 68,
    /// vertex moveword offset
    gfxd_Mwo_point = 69,
    /// w-component scale (perspnorm)
    gfxd_Wscale = 70,
    /// segment number
    gfxd_Seg = 71,
    /// segment pointer
    gfxd_Segptr = 72,
    /// dereferenced LightsM (0-7 or n) pointer
    gfxd_Lightsn = 73,
    /// light count (NUMLIGHTS_*)
    gfxd_Numlights = 74,
    /// light number (LIGHT_*)
    gfxd_Lightnum = 75,
    /// diffuse or ambient light pointer
    gfxd_Lightptr = 76,
    /// texture coordinate scale
    gfxd_Tcscale = 77,
    /// on-off value
    gfxd_Switch = 78,
    /// vertex coordinate (q10.5)
    gfxd_St = 79,
    /// vertex coordinate delta (q5.10)
    gfxd_Stdelta = 80,
    /// vertex pointer
    gfxd_Vtxptr = 81,
    /// viewport pointer
    gfxd_Vpptr = 82,
    /// generic dram address
    gfxd_Dram = 83,
    /// othermode lo shift
    gfxd_Sftlo = 84,
    /// othermode lo value
    gfxd_Othermodelo = 85,
    /// othermode hi shift
    gfxd_Sfthi = 86,
    /// othermode hi value
    gfxd_Othermodehi = 87,
    /// moveword index
    gfxd_Mw = 88,
    /// moveword offset
    gfxd_Mwo = 89,
    /// clip ratio moveword offset
    gfxd_Mwo_clip = 90,
    /// light color moveword offset
    gfxd_Mwo_lightcol = 91,
    /// movemem index
    gfxd_Mv = 92,
    /// movemem offset
    gfxd_Mvo = 93,
    /// dmem address
    gfxd_Dmem = 94,
    /// dma io flag
    gfxd_Dmaflag = 95,
}
