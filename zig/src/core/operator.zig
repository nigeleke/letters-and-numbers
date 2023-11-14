const std = @import("std");

pub const OperatorError = error{
    InvalidAdd,
    InvalidMinus,
    InvalidTimes,
    InvalidDivides,
};

pub const Operator = enum {
    plus,
    minus,
    times,
    divides,

    pub fn toString(self: Operator) []const u8 {
        return switch (self) {
            .plus => "+",
            .minus => "-",
            .times => "*",
            .divides => "/",
        };
    }

    pub fn apply(self: Operator, a: u32, b: u32) OperatorError!u32 {
        return switch (self) {
            .plus => if (a <= b) a + b else OperatorError.InvalidAdd,
            .minus => if (a > b) a - b else OperatorError.InvalidMinus,
            .times => if (a != 1 and b != 1 and a <= b) a * b else OperatorError.InvalidTimes,
            .divides => if (b != 0 and b != 1 and a % b == 0) a / b else OperatorError.InvalidDivides,
        };
    }
};

const expectEqual = std.testing.expectEqual;
const expectEqualStrings = std.testing.expectEqualStrings;
const expectError = std.testing.expectError;

test "operator.toString shows symbol" {
    try expectEqualStrings("+", Operator.plus.toString());
    try expectEqualStrings("-", Operator.minus.toString());
    try expectEqualStrings("*", Operator.times.toString());
    try expectEqualStrings("/", Operator.divides.toString());
}

test "plus returns value when first operand <= to second" {
    try expectEqual(4, comptime try Operator.plus.apply(2, 2));
    try expectEqual(6, comptime try Operator.plus.apply(2, 4));
    try expectError(OperatorError.InvalidAdd, Operator.plus.apply(4, 2));
}

test "minus returns value when first operand > second" {
    try expectError(OperatorError.InvalidMinus, Operator.minus.apply(2, 2));
    try expectError(OperatorError.InvalidMinus, Operator.minus.apply(2, 4));
    try expectEqual(2, comptime try Operator.minus.apply(4, 2));
}

test "times returns value when both operands are not identity and first operand is <= second" {
    try expectError(OperatorError.InvalidTimes, Operator.times.apply(1, 1));
    try expectError(OperatorError.InvalidTimes, Operator.times.apply(1, 2));
    try expectError(OperatorError.InvalidTimes, Operator.times.apply(2, 1));
    try expectEqual(4, comptime try Operator.times.apply(2, 2));
    try expectEqual(8, comptime try Operator.times.apply(2, 4));
    try expectError(OperatorError.InvalidTimes, Operator.times.apply(4, 2));
}

test "divides returns value when denominator not zero or identity and result is whole" {
    try expectError(OperatorError.InvalidDivides, Operator.divides.apply(2, 0));
    try expectError(OperatorError.InvalidDivides, Operator.divides.apply(2, 1));
    try expectError(OperatorError.InvalidDivides, Operator.divides.apply(3, 2));
    try expectEqual(3, comptime try Operator.divides.apply(6, 2));
}
