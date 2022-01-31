package main

import "fmt"
import "bufio"
import "os"

func main() {
	total_yes := 0
	total_part_2 := 0
    group_count := 0
	current_form := make(map[rune]int)

	s := bufio.NewScanner(os.Stdin)
    for s.Scan() {
		line := s.Text()
		if line == "" {
			total_yes += len(current_form)
			total_part_2 += count_value(&current_form, group_count)
			current_form = make(map[rune]int)
			group_count = 0
		} else {
			for _, c := range line {
				current_form[c] += 1
			}
			group_count += 1
		}
	}
	total_yes += len(current_form)
	total_part_2 += count_value(&current_form, group_count)

	fmt.Println("part 1", total_yes)
	fmt.Println("part 2", total_part_2)
}

func count_value(form *map[rune]int, value int) int {
	total := 0
	for _, count := range *form {
		if count == value {
			total += 1
		}
	}
	return total
}
