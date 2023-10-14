const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const Error = std.mem.Allocator.Error;

const operator = @import("operator.zig");
const Operator = operator.Operator;
const OperatorError = operator.OperatorError;

pub const Expressions = ArrayList(Expression);

pub const Expression = union(enum) {
    const Self = @This();

    operand: u32,
    operation: struct { left: *const Expression, op: Operator, right: *const Expression },

    fn asOperand(value: u32) Expression {
        return Expression{ .operand = value };
    }

    fn asOperation(left: *const Expression, op: Operator, right: *const Expression) Expression {
        return Expression{ .operation = .{
            .left = left,
            .op = op,
            .right = right,
        } };
    }

    pub fn from(numbers: []u32, allocator: Allocator) Error!Expressions {
        var expressions = Expressions.init(allocator);
        for (numbers) |n| {
            try expressions.append(asOperand(n));
        }

        const size = numbers.len;
        if (size > 1) {
            for (1..size) |i| {
                var left_expressions = try from(numbers[0..i], allocator);
                defer left_expressions.deinit();
                var right_expressions = try from(numbers[i..size], allocator);
                defer right_expressions.deinit();

                for (left_expressions.items) |le| {
                    for (right_expressions.items) |re| {
                        for (std.enums.values(Operator)) |o| {
                            var leAlloc = try allocator.create(Expression);
                            leAlloc.* = le;
                            var reAlloc = try allocator.create(Expression);
                            reAlloc.* = re;
                            try expressions.append(asOperation(leAlloc, o, reAlloc));
                        }
                    }
                }
            }
        }

        return expressions;
    }

    fn destroy(self: *const Self, allocator: Allocator) void {
        switch (self.*) {
            .operand => {},
            .operation => |o| {
                o.left.destroy(allocator);
                allocator.destroy(o.left);
                o.right.destroy(allocator);
                allocator.destroy(o.right);
            },
        }
    }

    fn result(self: *const Self) OperatorError!u32 {
        return switch (self.*) {
            .operand => |o| o,
            .operation => |o| {
                const op = o.op;
                const a = try o.left.result();
                const b = try o.right.result();
                return try op.apply(a, b);
            },
        };
    }

    fn description(self: *const Self) ![]u8 {
        // TODO: Determine best approach to stop allocated strings leaking.
        const allocator = std.heap.page_allocator;
        switch (self.*) {
            .operand => |o| {
                return try std.fmt.allocPrint(allocator, "{}", .{
                    o,
                });
            },
            .operation => |o| {
                return try std.fmt.allocPrint(allocator, "({s} {s} {s})", .{
                    try o.left.description(),
                    o.op.toString(),
                    try o.right.description(),
                });
            },
        }
    }
};

const test_allocator = std.testing.allocator;
const expectEqual = std.testing.expectEqual;
const expectEqualStrings = std.testing.expectEqualStrings;
const expectError = std.testing.expectError;

test "operand has a value" {
    const operand = Expression.asOperand(42);
    var expected: u32 = 42;
    try expectEqual(expected, try operand.result());
}

test "operation returns the same value as the operator applied to the operands" {
    const a = Expression.asOperand(10);
    const b = Expression.asOperand(5);

    for (std.enums.values(Operator)) |o| {
        const expression = Expression.asOperation(&a, o, &b);
        try expectEqual(o.apply(try a.result(), try b.result()), expression.result());
    }
}

test "single number returns single result" {
    var numbers = [_]u32{
        1,
    };
    var expressions = try Expression.from(&numbers, test_allocator);
    defer expressions.deinit();
    var expected: usize = 1;
    try expectEqual(expected, expressions.items.len);
}

test "multiple numbers returns result per operand and per operation" {
    var numbers = [_]u32{
        4,
        2,
    };
    var expressions = try Expression.from(&numbers, test_allocator);
    defer {
        for (expressions.items) |i| {
            i.destroy(test_allocator);
        }
        expressions.deinit();
    }
    var expectedLen: usize = 6;
    const items = expressions.items;
    try expectEqual(expectedLen, items.len);

    var expected0: u32 = 4;
    try expectEqual(expected0, try items[0].result());

    var expected1: u32 = 2;
    try expectEqual(expected1, try items[1].result());

    try expectError(OperatorError.InvalidAdd, items[2].result());

    var expected3: u32 = 2;
    try expectEqual(expected3, try items[3].result());

    try expectError(OperatorError.InvalidTimes, items[4].result());

    var expected5: u32 = 2;
    try expectEqual(expected5, try items[5].result());
}

test "multiple numbers returns expression descriptions" {
    var numbers = [_]u32{
        4,
        2,
    };
    var expressions = try Expression.from(&numbers, test_allocator);
    defer {
        for (expressions.items) |i| {
            i.destroy(test_allocator);
        }
        expressions.deinit();
    }
    var expectedLen: usize = 6;
    const items = expressions.items;
    try expectEqual(expectedLen, items.len);

    var expected0 = "4";
    try expectEqualStrings(expected0, try items[0].description());

    var expected1 = "2";
    try expectEqualStrings(expected1, try items[1].description());

    var expected2 = "(4 + 2)";
    try expectEqualStrings(expected2, try items[2].description());

    var expected3 = "(4 - 2)";
    try expectEqualStrings(expected3, try items[3].description());

    var expected4 = "(4 * 2)";
    try expectEqualStrings(expected4, try items[4].description());

    var expected5 = "(4 / 2)";
    try expectEqualStrings(expected5, try items[5].description());
}
