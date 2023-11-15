module main

import arrays
import cli
import core
import os

pub fn main() {
	mut cmd := cli.Command {
		name: 'lan'
		description: 'Numbers solvers for the Countdown numbers game'
		version: '1.0.0'
		usage: 'lan n1 n2 n3 n4 n5 n6 goal'
		required_args: 7
		execute: fn (cmd cli.Command) ! {
			numbers := assert_valid_numbers(cmd.args)!
			goal := assert_valid_goal(cmd.args)!
			solutions := core.find_solutions(numbers, goal)
			if solutions.len == 0 {
				println("No solutions")
			} else {
				for solution in solutions[..10] {
					println(solution)
				}
			}
			return
		}
	}
	cmd.setup()
	cmd.parse(os.args)
}

fn assert_valid_numbers(args []string) ![]int {
	numbers := args[..6].map(it.int())

	smalls := numbers.filter(1 <= it && it <= 10)
	larges := numbers.filter([25, 50, 75, 100].contains(it))

	n_valid_smalls := arrays.sum(arrays.map_of_counts(smalls).values().filter(it <= 2 )) or { 0 }
	n_valid_larges := arrays.sum(arrays.map_of_counts(larges).values().filter(it <= 1 )) or { 0 }
	n_valid := n_valid_smalls + n_valid_larges

	return if n_valid == 6 {
		numbers
	} else {
		error('Numbers must be six of [1..10, 1..10, 25, 50, 75, 100]')
	}
}

fn assert_valid_goal(args []string) !int {
	goal := args[6].int()
	return if 100 <= goal && goal <= 999 {
		goal
	} else {
		error('Goal must be in range 100..999 inclusive')
	}
}