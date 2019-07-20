/*
 * MIT License
 *
 * Copyright (c) 2018-2019 Matthew Henderson <mattw2018@hotmail.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

pub fn u8tou64(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8) -> u64
{
    return (a as u64) << 56 | (b as u64) << 48 | (c as u64) << 40 | (d as u64) << 32 |
           (e as u64) << 24 | (f as u64) << 16 | (g as u64) << 8 | (h as u64); 
}

pub fn u8tou32(a: u8, b: u8, c: u8, d: u8) -> u32
{
    return (a as u32) << 24 | (b as u32) << 16 | (c as u32) << 8 | (d as u32); 
}

pub fn u8tou16(a: u8, b: u8) -> u16
{
    return (a as u16) << 8 | (b as u16);
}

pub fn u8stou64(slice: &[u8]) -> u64
{
    return u8tou64(slice[7], slice[6], slice[5], slice[4], 
                   slice[3], slice[2], slice[1], slice[0]);  
}

pub fn u8stou32(slice: &[u8]) -> u32
{
    return u8tou32(slice[3], slice[2], slice[1], slice[0]);
}

pub fn u8stou16(slice: &[u8]) -> u16
{
    return u8tou16(slice[1], slice[0]);
}