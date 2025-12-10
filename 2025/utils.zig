const std = @import("std");
const mem = std.mem;

pub fn splitScalar(string: []const u8, delim: u8, alloc: mem.Allocator) ![][]u8 {
    var it = mem.splitScalar(u8, string, delim);
    const linesCount = mem.count(u8, string, &[1]u8{delim}) + 1;
    var res = try alloc.alloc([]u8, linesCount);
    var i: usize = 0;
    while (it.next()) |line| : (i += 1) {
        const mutLine = try alloc.alloc(u8, line.len);
        mem.copyForwards(u8, mutLine, line);
        res[i] = mutLine;
    }
    return res;
}

pub fn split(string: []const u8, delim: []const u8, alloc: mem.Allocator) ![][]u8 {
    var it = mem.splitSequence(u8, string, delim);
    const linesCount = mem.count(u8, string, delim) + 1;
    var res = try alloc.alloc([]u8, linesCount);
    var i: usize = 0;
    while (it.next()) |line| : (i += 1) {
        const mutLine = try alloc.alloc(u8, line.len);
        mem.copyForwards(u8, mutLine, line);
        res[i] = mutLine;
    }
    return res;
}

pub fn free(strings: [][]u8, alloc: mem.Allocator) void {
    for (strings) |s| {
        alloc.free(s);
    }
    alloc.free(strings);
}
