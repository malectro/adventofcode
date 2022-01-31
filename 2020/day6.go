package main

import "fmt"
import "bufio"
import "os"

func main() {
	total_yes := 0
	current_form := map[rune]bool{}

	s := bufio.NewScanner(os.Stdin)
    for s.Scan() {
		line := s.Text()
		if line == "" {
			total_yes += len(current_form)
			current_form = map[rune]bool{}
		} else {
			for _, c := range line {
				current_form[c] = true
			}
		}
	}
	total_yes += len(current_form)

	fmt.Println(total_yes)
}
