const std = @import("std");
const parseInt = std.fmt.parseInt;
const argMax = std.sort.argMax;

fn part1(reader: *std.io.Reader) !void {
    var sum: u32 = 0;
    while (try reader.takeDelimiter('\n')) |line| {
        var max1: u8 = 0;
        var max2: u8 = 0;
        for (line, 0..) |c, i| {
            const n: u8 = c - '0';
            var max1Mutated = false;
            if (max1 < n and i < line.len - 1) {
                max1 = n;
                max2 = line[i + 1] - '0';
                max1Mutated = true;
            }

            if (max2 < n and !max1Mutated) {
                max2 = n;
            }
        }
        sum += (max1 * 10) + max2;
    }
    std.debug.print("part 1 : {d}\n", .{sum});
}

fn part2(reader: *std.io.Reader) !void {
    var sum: u64 = 0;
    while (try reader.takeDelimiter('\n')) |line| {
        var numbers: [12]u8 = undefined;
        var index: u8 = 0;
        var minIndex: usize = 0;
        var maxIndex: usize = line.len - 11;
        while (index < 12) {
            const slice = line[minIndex..maxIndex];
            const sliceIndex = argMax(u8, slice, {}, std.sort.asc(u8)) orelse @panic("error");
            numbers[index] = slice[sliceIndex];
            minIndex += sliceIndex + 1;
            maxIndex += 1;
            index += 1;
        }
        sum += try parseInt(u64, &numbers, 10);
    }
    std.debug.print("part 2 : {d}\n", .{sum});
}

pub fn main() !void {
    // const exampleInput =
    //     \\987654321111111
    //     \\811111111111119
    //     \\234234234234278
    //     \\818181911112111
    // ;
    // var exampleReader = std.io.Reader.fixed(exampleInput);
    var file = try std.fs.cwd().openFile("days/day3.txt", .{ .mode = .read_only });

    var fileBuffer: [4096]u8 = undefined;
    var inputFileReader = file.reader(&fileBuffer);
    const inputReader = &inputFileReader.interface;

    try part1(inputReader);
    try inputFileReader.seekTo(0);
    try part2(inputReader);
}
