/*
Tunnul
Copyright (c) 2015, Richard Lettich
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:
1. Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.
3. The name of the author may not be used to endorse or promote products
   derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

--------------------------------------------------------------------------

THIS IS NOT AN OFFICIAL MINECRAFT PRODUCT,
NEITHER APPROVED NOR ASSOCIATED WITH MOJANG.
*/

// Calulates size  of varint what how to decode
// transfomed input flips negatives, which are then fliped back, to use less data
// the u8 depends on if it was flipped at first
fn var_int_flip(src: i32) -> (i32, u8) {
    match (src & (0x1 << 31i32)) as u32 {
        0x80000000u32 => ((src ^ ((0x1i32 << 31) >> 31), 0xFFu8)),
        _ => (src, 0x80u8),
    }

}
// bytes is number of bytes it uses
pub fn var_int_size(transformed: i32) -> usize {
    (((((32 - transformed.leading_zeros()) as usize) * 9 + 7) / 8) + 7) / 8
}

pub fn to(src: i32) -> Vec<u8> {
    let (transform, y) = var_int_flip(src);
    let bytes = var_int_size(transform);
    let mut result = (0..bytes)
                         .map(|i| y ^ ((transform >> (7 * i)) as u8) & 0x7F)
                         .collect::<Vec<u8>>();
    result[bytes - 1] ^= 0x80u8;
    result
}
pub fn write_to(src: i32, mut vector: &mut Vec<u8>) {
    let (transform, y) = var_int_flip(src);
    let start = vector.len();
    let end = start + var_int_size(transform);
    unsafe { vector.set_len(end) };
    for (i, val) in (start..end).enumerate() {
        vector[val] = y ^ (((transform >> (7 * i)) as u8) & 0x7F);
    }
    vector[end - 1] ^= 0x80u8;
}
