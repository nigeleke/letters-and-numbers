const std = @import("std");
const ArrayList = std.ArrayList;

pub fn main() !void {}

fn getArray() !ArrayList(ArrayList(i32)) {
    var outer = ArrayList(ArrayList(i32)).init(std.testing.allocator);
    var inner = ArrayList(i32).init(std.testing.allocator);
    try outer.append(inner);
    std.debug.print("\n\ngetArray:outer@ {*}\n", .{&outer});
    std.debug.print("\n\ngetArray:inner@ {*}\n", .{&inner});
    return outer;
}

test "simple test" {
    var outer = try getArray();
    defer outer.deinit();
    var outers = outer.allocatedSlice();
    std.debug.print("\n\ngotOuterArray:@ {*}\n", .{&outer});
    std.debug.print("\n\ngotOuterSlice:@ {*}\n", .{&outers});
    var inner = outers[0];
    std.debug.print("\n\ngotInnerArray:@ {*}\n", .{&inner});
}
