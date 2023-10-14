const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

const expression = @import("expression.zig");
const Expression = expression.Expression;
const Expressions = expression.Expressions;

const Resolver = struct {
    fn swap(slice: []u32, a: usize, b: usize) void {
        var tmp = slice[a];
        slice[a] = slice[b];
        slice[b] = tmp;
    }

    fn permutations(slice: []u32, start: usize, allocator: Allocator) !ArrayList(ArrayList(u32)) {
        var ps = ArrayList(ArrayList(u32)).init(allocator);
        const n = slice.len - start;
        if (n <= 1) {
           var p = ArrayList(u32).init(allocator);
           try p.appendSlice(slice);
           try ps.append(p);
        } else {
            for (0..n) |i| {
                const index = start + i;
                swap(slice, start, index);
                var subps = try permutations(slice, start + 1, allocator);
                defer subps.deinit();
                for (subps.items) |subp| {
                    try ps.append(subp);
                }
                swap(slice, start, index);
            }
        }
        return ps;
    }

    pub fn find_solutions(operands: []u32, goal: u32, allocator: Allocator) !Expressions {
        var solutions = Expressions.init(allocator);
        defer solutions.deinit();
        for (1..7) |n| {
            std.debug.print("\n\nPermutations from {any} => {}\n", .{operands, n});
            var ps = try permutations(operands, n, allocator);
            defer ps.deinit();
            for (ps.items) |p| {
                const pslice = p.items;
                std.debug.print("\n\nExpression from {any}\n", .{pslice});
                var es = try Expression.from(pslice, allocator);
                _ = es;
            }
        }
        _ = goal;
        //     let solutions = iter! {
        //       let n <- 1..=6;

        //       let permutation <- Vec::from(operands).into_iter().permutations(n);
        //       let expressions = permutation.expressions();
        //       let expression <- expressions;
        //       let value = expression.value();
        //       if value == Some(goal);
        //       expression
        //     };

        //     solutions
        //       .into_iter()
        //       .map(|s| s.description())
        //       .collect::<Vec<String>>()
        return solutions;
    }
};

const test_allocator = std.testing.allocator;
const expect = std.testing.expect;
const expectEqual = std.testing.expectEqual;
const expectEqualStrings = std.testing.expectEqualStrings;
const expectError = std.testing.expectError;

test "a resolver will use permutations" {
    var a = [_]u32{ 1, 2, 3 };
    var ps = try Resolver.permutations(&a, 0, test_allocator);
    defer {
        for (ps.items) |p| p.deinit();
        ps.deinit();
    }
    var expectedLen: usize = 6;
    try expectEqual(expectedLen, ps.items.len);
}

test "a resolver returns a none result for an impossible goal" {
    var numbers = [_]u32{ 1, 2, 3, 4, 5, 6 };
    const goal = 999;
    const results = try Resolver.find_solutions(&numbers, goal, test_allocator);
    defer results.deinit();
    var expectedLen: usize = 0;
    try expectEqual(expectedLen, results.items.len);
}

// test "a resolver returns valid expressions for a simple possible goal" {
//     var numbers = [_]u32{ 1, 2, 3, 4, 5, 100 };
//     const goal = 100;
//     const results = try Resolver.find_solutions(&numbers, goal, test_allocator);
//     defer results.deinit();
//     try expect(results.items.len > 0);
// }

//   #[test]
//   fn a_resolver_returns_valid_expressions_for_a_complex_possible_goal() {
//     let results = Resolver::find_solutions(&vec![1, 2, 3, 4, 5, 6], 720);
//     assert!(results.len() > 0);
//   }

//   #[test]
//   fn a_resolver_returns_valid_expressions_for_defects() {
//     let defects = vec![
//       (vec![75, 3, 6, 5, 5, 1], 559),
//       (vec![5, 8, 8, 2, 100, 50], 543),
//     ];
//     for defect in defects {
//       let (numbers, goal) = defect;
//       let results = Resolver::find_solutions(&numbers, goal);
//       assert!(results.len() > 0);
//     }
//   }
// }
