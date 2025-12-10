const std = @import("std");
const mem = std.mem;
const utils = @import("utils.zig");

const Point = struct {
    x: usize,
    y: usize,
};

fn boolToInt(value: bool) u32 {
    return if (value) 1 else 0;
}

fn checkBounds(x: usize, y: usize, width: usize, height: usize, map: [][]u8) u32 {
    var res: u32 = 0;

    if (y != 0) {
        // top
        res += boolToInt(map[y - 1][x] == '@');
    }
    if (y != height - 1) {
        // bottom
        res += boolToInt(map[y + 1][x] == '@');
    }
    if (x != 0) {
        // left
        res += boolToInt(map[y][x - 1] == '@');
    }
    if (x != width - 1) {
        // right
        res += boolToInt(map[y][x + 1] == '@');
    }
    if (x != 0 and y != 0) {
        // top left
        res += if (x != 0) boolToInt(map[y - 1][x - 1] == '@') else 0;
    }
    if (x != width - 1 and y != 0) {
        // top right
        res += if (x != width - 1) boolToInt(map[y - 1][x + 1] == '@') else 0;
    }
    if (x != 0 and y != height - 1) {
        // bottom left
        res += if (x != 0) boolToInt(map[y + 1][x - 1] == '@') else 0;
    }
    if (x != width - 1 and y != height - 1) {
        // bottom right
        res += if (x != width - 1) boolToInt(map[y + 1][x + 1] == '@') else 0;
    }
    return res;
}

fn part1(input: [][]u8) !void {
    const height = input.len - 1;
    const width = input[0].len;
    var sum: u32 = 0;
    for (0..height) |y| {
        for (0..width) |x| {
            const char = input[y][x];
            if (char == '@') {
                if (checkBounds(x, y, width, height, input) < 4) {
                    sum += 1;
                } else {}
            } else {}
        }
    }
    std.debug.print("part 1 : {d}\n", .{sum});
}

fn part2(input: [][]u8, alloc: mem.Allocator) !void {
    const height = input.len - 1;
    const width = input[0].len;
    var sum: u32 = 0;
    while (true) {
        var tempSum: u32 = 0;
        var toModify: std.ArrayList(Point) = .empty;
        for (0..height) |y| {
            for (0..width) |x| {
                const char = input[y][x];
                if (char == '@') {
                    const nRemoved = checkBounds(x, y, width, height, input);
                    if (nRemoved < 4) {
                        sum += 1;
                        tempSum += 1;
                        try toModify.append(alloc, .{ .x = x, .y = y });
                    } else {}
                } else {}
            }
        }
        if (tempSum == 0) {
            break;
        }
        for (toModify.items) |point| {
            input[point.y][point.x] = '.';
        }
        toModify.clearAndFree(alloc);
        tempSum = 0;
    }
    std.debug.print("part 2 : {d}\n", .{sum});
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const alloc = gpa.allocator();
    const exampleInput =
        \\..@@.@@@@.
        \\@@@.@.@.@@
        \\@@@@@.@.@@
        \\@.@@@@..@.
        \\@@.@@@@.@@
        \\.@@@@@@@.@
        \\.@.@.@.@@@
        \\@.@@@.@@@@
        \\.@@@@@@@@.
        \\@.@.@@@.@.
    ;
    const inputFile = try std.fs.cwd().openFile("days/day4.txt", .{ .mode = .read_only });
    const input = try inputFile.readToEndAlloc(alloc, try inputFile.getEndPos());
    defer alloc.free(input);

    const parsedExampleInput = try utils.splitScalar(exampleInput, '\n', alloc);
    defer utils.free(parsedExampleInput, alloc);
    const parsedInput = try utils.splitScalar(input, '\n', alloc);
    defer utils.free(parsedInput, alloc);

    try part1(parsedInput);
    try part2(parsedInput, alloc);
}
